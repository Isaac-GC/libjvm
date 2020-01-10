#![allow(unused)]

mod class_loader;
mod consts;
mod cp_manager;
mod execution;
mod frame;
mod java_call;
mod local;
mod slot;
mod stack;
mod sys_dic;
pub mod thread;

pub use class_loader::{require_class, require_class2, require_class3, ClassLoader};
pub use consts::THREAD_MAX_STACK_FRAMES;
pub use cp_manager::{find_class as find_class_in_classpath, ClassPathResult, ClassSource};
pub use execution::instance_of;
pub use frame::Frame;
pub use local::Local;
pub use slot::Slot;
pub use stack::Stack;
pub use sys_dic::{find as sys_dic_find, put as sys_dic_put};
pub use thread::JavaThreadRef;

pub fn init() {
    sys_dic::init();
    cp_manager::init();
}
