#![allow(bad_style)]

extern crate jni_sys;
extern crate libc;

use jni_sys::*;
use libc::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
