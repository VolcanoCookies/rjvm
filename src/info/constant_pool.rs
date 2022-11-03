use std::any::{Any, TypeId};

use crate::r#type::dyn_vec::DynVec;

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

    pub fn get<T>(&self, i: usize) -> &T
    where
        T: Any,
    {
        self.pool.get::<T>(i)
    }

    pub fn push<T>(&mut self, e: T)
    where
        T: Any,
    {
        self.pool.push(e);
    }

    pub fn push_box(&mut self, e: Box<dyn Any>) {
        self.pool.push_box(e);
    }
}
