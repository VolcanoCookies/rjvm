use std::borrow::Borrow;
use std::ops::Index;

pub struct Bytes {
    raw: Box<[u8]>,
}

impl Bytes {
    pub fn new(v: Vec<u8>) -> Self {
        Self {
            raw: v.into_boxed_slice(),
        }
    }

    pub fn get(&self, index: usize) -> u8 {
        self.raw[index]
    }

    pub fn get_2(&self, index: usize) -> u16 {
        ((self.raw[index] as u16) << 8) | (self.raw[index + 1] as u16)
    }

    pub fn get_4(&self, index: usize) -> u32 {
        ((self.raw[index] as u32) << 24)
            | ((self.raw[index + 1] as u32) << 16)
            | ((self.raw[index + 2] as u32) << 8)
            | (self.raw[index + 3] as u32)
    }

    pub fn get_n(&self, index: usize, n: usize) -> Bytes {
        let e: &[u8] = self.raw.borrow();
        let v = &e[index..(index + n)];

        Bytes {
            raw: v.to_vec().into_boxed_slice(),
        }
    }
}

impl Index<usize> for Bytes {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.raw[index]
    }
}
