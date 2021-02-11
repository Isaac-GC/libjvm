use crate::oop::Oop;
use crate::runtime::local::Local;
use crate::runtime::stack::Stack;
use std::cell::RefCell;
use std::sync::{Arc, RwLock};

/*
The origin of DataArea

java method execution method:
Every time a method is called, a new Frame is constructed,
and the frame is pushed to the current thread.frames stack.
After the method is executed, the Frame is popped.
If an exception occurs, jvm_fillInStackTrace traverses the current thread frames:
extract the class name, method name, and pc (pc for LineNumberTable Attributes from each frame)
Locate the error line of code) and construct an exception stack.

The DataArea in the Frame is wrapped with RefCell, so that java_call::invoke_java can execute Java
Method, you can use the read-only frame to execute bytecode; when there is an exception, you can also let
jvm_fillInStackTrace traverse the frames to get the necessary information.
The nature of RefCell makes this possible.
In a read-only Frame context, to modify the DataArea, borrow_mut is fine.
*/
pub struct DataArea {
    pub stack: RefCell<Stack>,
    pub return_v: RefCell<Option<Oop>>,
}

unsafe impl Sync for DataArea {}

impl DataArea {
    pub fn new(max_stack: usize) -> Self {
        let stack = RefCell::new(Stack::new(max_stack));

        Self {
            stack,
            return_v: RefCell::new(None),
        }
    }
}
