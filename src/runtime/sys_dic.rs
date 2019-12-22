use crate::oop::ClassRef;
use crate::util;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

type SystemDictionary = Mutex<HashMap<String, ClassRef>>;

lazy_static! {
    static ref SYS_DIC: SystemDictionary = { Mutex::new(HashMap::new()) };
}

pub fn put(key: &[u8], klass: ClassRef) {
    util::sync_call_ctx(&SYS_DIC, |dic| {
        let key = String::from_utf8_lossy(key);
        dic.insert(key.to_string(), klass);
    })
}

pub fn find(key: &[u8]) -> Option<ClassRef> {
    let key = std::str::from_utf8(key).unwrap();
    util::sync_call_ctx(&SYS_DIC, |dic| dic.get(key).map(|it| it.clone()))
}

pub fn init() {
    lazy_static::initialize(&SYS_DIC);
}
