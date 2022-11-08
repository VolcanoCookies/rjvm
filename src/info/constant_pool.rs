use crate::info::pool::constant_info::ConstantInfo;
use crate::util::dyn_vec::DynVec;
use std::any::Any;
use std::ops::Index;

pub struct ConstantPool {
    pool: DynVec,
    count: usize,
}

impl ConstantPool {
    pub fn new(constant_pool_count: usize) -> Self {
        Self {
            pool: DynVec::with_capacity(constant_pool_count - 1),
            count: constant_pool_count,
        }
    }

    pub fn push<T>(&mut self, e: T)
    where
        T: Any,
    {
        self.push_box(Box::new(e))
    }

    pub fn push_box(&mut self, boxed: Box<dyn Any>) {
        self.pool.push_box(boxed)
    }

    pub fn pop<T>(&mut self) -> T
    where
        T: Any,
    {
        self.pool.pop()
    }

    pub fn get<T: 'static>(&self, i: usize) -> &T {
        if i == 0 {
            panic!("Do not the zero index.");
        }
        self.pool.get::<T>(i - 1)
    }
}
