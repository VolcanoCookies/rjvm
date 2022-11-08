use crate::info::pool::constant_info_trait::ConstantInfoTrait;
use crate::util::read_buffer::ReadBuffer;
use std::any::{Any, TypeId};

use super::constant_info_tag::ConstantInfoTag;

pub struct ConstantUtf8Info {
    pub length: usize,
    pub bytes: Vec<u8>,
}

impl ConstantUtf8Info {
    pub fn str(&self) -> &str {
        &std::str::from_utf8(&self.bytes).unwrap()
    }

    pub fn static_str(&self) -> &'static mut str {
        let string = String::from_utf8(self.bytes.clone()).unwrap();
        Box::leak(string.into_boxed_str())
    }
}

impl ConstantInfoTrait for ConstantUtf8Info {
    const tag: u8 = ConstantInfoTag::CONSTANT_Utf8;

    fn parse(r: &mut ReadBuffer) -> Self {
        let length = r.read_u2() as usize;
        let bytes = r.read_n(length);
        Self { length, bytes }
    }
}
