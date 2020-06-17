use crate::oop::{Oop, OopRef};
use crate::runtime::Slot;
use crate::util;
use std::sync::Arc;

pub struct Local {
    locals: Vec<Slot>,
}

// Add type information, set_xx is detected
impl Local {
    pub fn new(size: usize) -> Self {
        let size = size + 1;
        let locals = vec![Slot::Nop; size];
        Self { locals }
    }

    pub fn set_int(&mut self, pos: usize, i: i32) {
        let v = i.to_be_bytes();
        self.set_primitive2(pos, v);
    }

    pub fn set_long(&mut self, pos: usize, l: i64) {
        let v = l.to_be_bytes();
        self.set_primitive3(pos, v);
    }

    pub fn set_float(&mut self, pos: usize, f: f32) {
        let v = f.to_bits().to_be_bytes();
        self.set_primitive2(pos, v);
    }

    pub fn set_double(&mut self, pos: usize, d: f64) {
        let v = d.to_bits().to_be_bytes();
        self.set_primitive3(pos, v);
    }

    pub fn set_ref(&mut self, pos: usize, v: Oop) {
        self.locals[pos] = Slot::Ref(v);
    }

    pub fn get_int(&self, pos: usize) -> i32 {
        match self.locals.get(pos).unwrap() {
            Slot::Primitive(v) => i32::from_be_bytes([v[0], v[1], v[2], v[3]]),
            Slot::Ref(v) => OopRef::java_lang_integer_value(v.extract_ref()),
            t => panic!("Illegal type {:?}", t),
        }
    }

    pub fn get_long(&self, pos: usize) -> i64 {
        if let Slot::Primitive(v) = self.locals.get(pos).unwrap() {
            i64::from_be_bytes([v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7]])
        } else {
            panic!("Illegal type");
        }
    }

    pub fn get_float(&self, pos: usize) -> f32 {
        if let Slot::Primitive(v) = self.locals.get(pos).unwrap() {
            let v = u32::from_be_bytes([v[0], v[1], v[2], v[3]]);
            f32::from_bits(v)
        } else {
            panic!("Illegal type");
        }
    }

    pub fn get_double(&self, pos: usize) -> f64 {
        if let Slot::Primitive(v) = self.locals.get(pos).unwrap() {
            let v = u64::from_be_bytes([v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7]]);
            f64::from_bits(v)
        } else {
            panic!("Illegal type");
        }
    }

    pub fn get_ref(&self, pos: usize) -> Oop {
        match self.locals.get(pos) {
            Some(Slot::Ref(v)) => v.clone(),
            t => panic!("Illegal type = {:?}", t),
        }
    }
}

impl Local {
    fn set_primitive(&mut self, pos: usize, buf: Vec<u8>) {
        self.locals[pos] = Slot::Primitive(buf);
    }

    fn set_primitive2(&mut self, pos: usize, v: [u8; 4]) {
        let v = vec![v[0], v[1], v[2], v[3]];
        self.set_primitive(pos, v);
    }

    fn set_primitive3(&mut self, pos: usize, v: [u8; 8]) {
        let v = vec![v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7]];
        self.set_primitive(pos, v);
    }
}
