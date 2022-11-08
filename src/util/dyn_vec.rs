use std::any::{type_name, Any, TypeId};

use crate::info::pool::constant_utf8_info::ConstantUtf8Info;

pub struct DynVec {
    raw: Vec<Box<dyn Any>>,
}

impl DynVec {
    pub fn new() -> Self {
        Self { raw: Vec::new() }
    }

    pub fn with_capacity(c: usize) -> Self {
        Self {
            raw: Vec::with_capacity(c),
        }
    }

    pub fn push<T: Any>(&mut self, e: T) {
        self.push_box(Box::new(e))
    }

    pub fn push_box(&mut self, boxed: Box<dyn Any>) {
        self.raw.push(boxed)
    }

    pub fn pop<T: Any>(&mut self) -> T {
        let boxed = self.raw.pop().unwrap();
        *boxed.downcast::<T>().unwrap()
    }

    pub fn get<T: 'static>(&self, i: usize) -> &T {
        let boxed = self.raw.get(i).unwrap();
        println!("Getting index {}", i);
        println!("{:?}", type_name::<T>());

        boxed.downcast_ref::<T>().unwrap()
    }
}
