#![allow(non_snake_case)]
use crate::native::{new_fn, JNIEnv, JNINativeMethod, JNIResult};
use crate::oop::{self, Oop, TypeArrayDesc};
use crate::runtime::{self, require_class3};
use crate::types::JavaThreadRef;
use crate::util;
use classfile::consts as cls_consts;

pub fn get_native_methods() -> Vec<JNINativeMethod> {
    vec![
        new_fn("initIDs", "()V", Box::new(jvm_initIDs)),
        new_fn("open0", "(Ljava/lang/String;)V", Box::new(jvm_open0)),
        new_fn("readBytes", "([BII)I", Box::new(jvm_readBytes)),
        //available0 used by zulu8 jdk
        new_fn("available0", "()I", Box::new(jvm_available0)),
        new_fn("available", "()I", Box::new(jvm_available0)),
        new_fn("close0", "()V", Box::new(jvm_close0)),
    ]
}

fn jvm_initIDs(_jt: JavaThreadRef, _env: JNIEnv, _args: Vec<Oop>) -> JNIResult {
    Ok(None)
}

fn jvm_open0(_jt: JavaThreadRef, _env: JNIEnv, args: Vec<Oop>) -> JNIResult {
    let this = args.get(0).unwrap();
    let name = {
        let v = args.get(1).unwrap();
        util::oop::extract_str(v)
    };
    let fd = unsafe {
        use std::ffi::CString;
        let name = CString::new(name).unwrap();
        libc::open(name.as_ptr(), libc::O_RDONLY)
    };

    set_file_descriptor_fd(this, fd);

    Ok(None)
}

fn jvm_readBytes(jt: JavaThreadRef, _env: JNIEnv, args: Vec<Oop>) -> JNIResult {
    let this = args.get(0).unwrap();
    let fd = get_file_descriptor_fd(this);
    let byte_ary = args.get(1).unwrap();
    let off = {
        let v = args.get(2).unwrap();
        util::oop::extract_int(v)
    };
    let len = {
        let v = args.get(3).unwrap();
        util::oop::extract_int(v)
    };

    let v = util::oop::extract_ref(byte_ary);
    let mut v = v.write().unwrap();
    let n = match &mut v.v {
        oop::RefKind::TypeArray(ary) => match ary {
            TypeArrayDesc::Byte(ary) => {
                let (_, ptr) = ary.split_at_mut(off as usize);
                let ptr = ptr.as_mut_ptr() as *mut libc::c_void;
                let n = unsafe { libc::read(fd, ptr, len as usize) };
                // error!("readBytes n = {}", n);
                if n > 0 {
                    n as i32
                } else if n == -1 {
                    let ex = runtime::exception::new(
                        jt,
                        cls_consts::J_IOEXCEPTION,
                        Some(String::from("Read Error")),
                    );
                    error!("read error");
                    return Err(ex);
                } else {
                    -1
                }
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };

    Ok(Some(Oop::new_int(n)))
}

fn jvm_available0(_jt: JavaThreadRef, _env: JNIEnv, args: Vec<Oop>) -> JNIResult {
    let this = args.get(0).unwrap();
    let fd = get_file_descriptor_fd(this);

    if fd == -1 {
        unimplemented!("Stream Closed");
    }

    let mut size = -1i64;

    unsafe {
        let mut stat: libc::stat = std::mem::zeroed();
        if libc::fstat(fd, &mut stat) != -1 {
            let mode = stat.st_mode;
            if (mode & libc::S_IFIFO == libc::S_IFIFO)
                || (mode & libc::S_IFCHR == libc::S_IFCHR)
                || (mode & libc::S_IFSOCK == libc::S_IFSOCK)
            {
                let mut n = 0;
                if libc::ioctl(fd, libc::FIONREAD, &mut n) >= 0 {
                    return Ok(Some(Oop::new_int(n)));
                }
            } else if mode & libc::S_IFREG == libc::S_IFREG {
                size = stat.st_size;
            }
        }

        let current = libc::lseek(fd, 0, libc::SEEK_CUR);
        if current == -1 {
            return Ok(Some(Oop::new_int(0)));
        }

        if size < current {
            size = libc::lseek(fd, 0, libc::SEEK_END);
            if size == -1 {
                return Ok(Some(Oop::new_int(0)));
            }

            if libc::lseek(fd, current, libc::SEEK_SET) == -1 {
                return Ok(Some(Oop::new_int(0)));
            }
        }

        return Ok(Some(Oop::new_int((size - current) as i32)));
    }
}

fn jvm_close0(_jt: JavaThreadRef, _env: JNIEnv, args: Vec<Oop>) -> JNIResult {
    let this = args.get(0).unwrap();
    let fd = get_file_descriptor_fd(this);
    unsafe {
        libc::close(fd);
    }
    Ok(None)
}

fn set_file_descriptor_fd(fin: &Oop, fd: i32) {
    let cls = require_class3(None, b"java/io/FileInputStream").unwrap();
    let fd_this = {
        let cls = cls.read().unwrap();
        let id = cls.get_field_id(b"fd", b"Ljava/io/FileDescriptor;", false);
        cls.get_field_value(fin, id)
    };

    let cls = require_class3(None, b"java/io/FileDescriptor").unwrap();
    let cls = cls.read().unwrap();
    let id = cls.get_field_id(b"fd", b"I", false);
    cls.put_field_value(fd_this, id, Oop::new_int(fd));
}

fn get_file_descriptor_fd(fin: &Oop) -> i32 {
    let cls = require_class3(None, b"java/io/FileInputStream").unwrap();
    let fd_this = {
        let cls = cls.read().unwrap();
        let id = cls.get_field_id(b"fd", b"Ljava/io/FileDescriptor;", false);
        cls.get_field_value(fin, id)
    };

    let cls = require_class3(None, b"java/io/FileDescriptor").unwrap();
    let fd = {
        let cls = cls.read().unwrap();
        let id = cls.get_field_id(b"fd", b"I", false);
        cls.get_field_value(&fd_this, id)
    };

    util::oop::extract_int(&fd)
}
