
use bytes::{BigEndian, Bytes};

use crate::classfile::constant_pool::ConstantType;
use crate::classfile::method_info::MethodInfo;
use crate::classfile::types::*;
use crate::classfile::ClassFile;
use crate::runtime::Slot;

pub struct Stack {
    inner: Vec<Slot>,
}

impl Stack {
    pub fn new(size: usize) -> Self {
        Self {
            inner: Vec::with_capacity(size),
        }
    }

    pub fn push_int(&mut self, i: i32) {
        let v = i.to_be_bytes();
        self.push(Bytes::from(vec![v[0], v[1], v[2], v[3]]));
    }

    pub fn push_float(&mut self, f: f32) {
        let v = f.to_bits().to_be_bytes();
        self.push(Bytes::from(vec![v[0], v[1], v[2], v[3]]));
    }

    pub fn push_double(&mut self, d: f64) {
        let v = d.to_bits().to_be_bytes();
        self.push(Bytes::from(vec![
            v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7],
        ]));
    }

    pub fn push_long(&mut self, l: i64) {
        let v = l.to_be_bytes();
        self.push(Bytes::from(vec![
            v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7],
        ]));
    }

    pub fn push(&mut self, b: Bytes) {
        self.inner.push(Slot::Primitive(b));
    }

    pub fn push2(&mut self, v: [u8; 4]) {
        let v = vec![v[0], v[1], v[2], v[3]];
        let v = Bytes::from(v);
        self.push(v)
    }

    pub fn push3(&mut self, v: [u8; 8]) {
        let v = vec![v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7]];
        let v = Bytes::from(v);
        self.push(v);
    }

    pub fn push_null(&mut self) {
        self.inner.push(Slot::Null);
    }

    pub fn push_const_m1(&mut self) {
        self.inner.push(Slot::ConstM1);
    }

    pub fn push_const0(&mut self) {
        self.inner.push(Slot::Const0);
    }

    pub fn push_const1(&mut self) {
        self.inner.push(Slot::Const1);
    }

    pub fn push_const2(&mut self) {
        self.inner.push(Slot::Const2);
    }

    pub fn push_const3(&mut self) {
        self.inner.push(Slot::Const3);
    }

    pub fn push_const4(&mut self) {
        self.inner.push(Slot::Const4);
    }

    pub fn push_const5(&mut self) {
        self.inner.push(Slot::Const5);
    }

    pub fn pop_int(&mut self) -> i32 {
        match self.inner.pop().unwrap() {
            Slot::ConstM1 => -1,
            Slot::Const0 => 0,
            Slot::Const1 => 1,
            Slot::Const2 => 2,
            Slot::Const3 => 3,
            Slot::Const4 => 4,
            Slot::Const5 => 5,
            Slot::Primitive(v) => i32::from_be_bytes([v[0], v[1], v[2], v[3]]),
            _ => panic!("Illegal type"),
        }
    }

    pub fn pop_float(&mut self) -> f32 {
        match self.inner.pop().unwrap() {
            Slot::Const0 => 0.0,
            Slot::Const1 => 1.0,
            Slot::Const2 => 2.0,
            Slot::Primitive(v) => {
                let v = u32::from_be_bytes([v[0], v[1], v[2], v[3]]);
                f32::from_bits(v)
            }
            _ => panic!("Illegal type"),
        }
    }

    pub fn pop_double(&mut self) -> f64 {
        match self.inner.pop().unwrap() {
            Slot::Const0 => 0.0,
            Slot::Const1 => 1.0,
            Slot::Primitive(v) => {
                let v = u64::from_be_bytes([v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7]]);
                f64::from_bits(v)
            }
            _ => panic!("Illegal type"),
        }
    }

    pub fn pop_long(&mut self) -> i64 {
        match self.inner.pop().unwrap() {
            Slot::Const0 => 0,
            Slot::Const1 => 1,
            Slot::Primitive(v) => {
                i64::from_be_bytes([v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7]])
            }
            _ => panic!("Illegal type"),
        }
    }

    pub fn drop_top(&mut self) {
        let _ = self.inner.pop();
    }

    fn clear(&mut self) {
        self.inner.clear();
    }
}

