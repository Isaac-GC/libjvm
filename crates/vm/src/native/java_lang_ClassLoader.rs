#![allow(non_snake_case)]

use crate::native::{new_fn, JNIEnv, JNINativeMethod, JNIResult};
use crate::oop::{self, Oop};
use crate::runtime;
use crate::util;

pub fn get_native_methods() -> Vec<JNINativeMethod> {
    vec![
        new_fn("registerNatives", "()V", Box::new(jvm_registerNatives)),
        new_fn(
            "findBuiltinLib",
            "(Ljava/lang/String;)Ljava/lang/String;",
            Box::new(jvm_findBuiltinLib),
        ),
        new_fn(
            "findLoadedClass0",
            "(Ljava/lang/String;)Ljava/lang/Class;",
            Box::new(jvm_findLoadedClass0),
        ),
        new_fn(
            "findBootstrapClass",
            "(Ljava/lang/String;)Ljava/lang/Class;",
            Box::new(jvm_findBootstrapClass),
        ),
    ]
}

fn jvm_registerNatives(_env: JNIEnv, _args: Vec<Oop>) -> JNIResult {
    Ok(None)
}

fn jvm_findBuiltinLib(_env: JNIEnv, args: Vec<Oop>) -> JNIResult {
    let name = args.get(0).unwrap();
    let name = util::oop::extract_str(name);
    info!("findBuiltinLib: {}", name);
    Ok(None)
}

fn jvm_findLoadedClass0(_env: JNIEnv, args: Vec<Oop>) -> JNIResult {
    let name = args.get(1).unwrap();
    let name = util::oop::extract_str(name);
    info!("findLoadedClass0: {}", name);
    let name = name.replace(".", util::FILE_SEP);
    let v = match runtime::sys_dic_find(name.as_bytes()) {
        Some(cls) => {
            let cls = cls.read().unwrap();
            cls.get_mirror()
        }
        None => oop::consts::get_null(),
    };
    Ok(Some(v))
}

// fixme: Is this correct? uncertain
fn jvm_findBootstrapClass(_env: JNIEnv, args: Vec<Oop>) -> JNIResult {
    info!("findBootstrapClass");
    jvm_findLoadedClass0(_env, args)
}
