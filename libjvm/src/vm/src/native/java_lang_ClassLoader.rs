#![allow(non_snake_case)]
#![allow(unused)]

use crate::native::{new_fn, JNIEnv, JNINativeMethod, JNIResult};
use crate::oop::{self, Oop, OopPtr};
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

fn jvm_registerNatives(_env: JNIEnv, _args: &[Oop]) -> JNIResult {
    Ok(None)
}

fn jvm_findBuiltinLib(_env: JNIEnv, args: &[Oop]) -> JNIResult {
    let name = args.get(0).unwrap();
    let name = OopPtr::java_lang_string(name.extract_ref());
    info!("findBuiltinLib: {}", name);
    Ok(None)
}

fn jvm_findLoadedClass0(_env: JNIEnv, args: &[Oop]) -> JNIResult {
    let name = args.get(1).unwrap();
    let name = OopPtr::java_lang_string(name.extract_ref());
    info!("findLoadedClass0: {}", name);
    let name = name.replace(".", util::FILE_SEP);
    let v = match runtime::sys_dic_find(name.as_bytes()) {
        Some(cls) => {
            let cls = cls.get_class();
            cls.get_mirror()
        }
        None => Oop::Null,
    };
    Ok(Some(v))
}

// fixme: Is this correct? uncertain
fn jvm_findBootstrapClass(_env: JNIEnv, args: &[Oop]) -> JNIResult {
    info!("findBootstrapClass");
    jvm_findLoadedClass0(_env, args)
}
