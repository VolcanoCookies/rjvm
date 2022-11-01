use crate::info::pool::constant_info::ConstantInfo;
use std::ops::Index;

pub struct ConstantPool {
    pool: Vec<ConstantInfo>,
    count: usize,
}

impl ConstantPool {
    pub fn new(constant_pool_count: usize) -> Self {
        let pool = Vec::<ConstantInfo>::with_capacity(constant_pool_count - 1);

        Self {
            pool,
            count: constant_pool_count,
        }
    }

    pub fn push(&mut self, c: ConstantInfo) {
        self.pool.push(c);
    }
}

impl Index<usize> for ConstantPool {
    type Output = ConstantInfo;

    fn index(&self, index: usize) -> &Self::Output {
        if index == 0 {
            panic!("Do not the zero index.");
        }

        &self.pool[index - 1]
    }
}
