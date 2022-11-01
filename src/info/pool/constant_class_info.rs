/*
CONSTANT_Class_info {
u1 tag;
u2 name_index;
}
*/

use crate::info::pool::constant_info_trait::ConstantInfoTrait;
use crate::util::read_buffer::ReadBuffer;

use super::constant_info_tag::ConstantInfoTag;

pub struct ConstantClassInfo {
    name_index: usize,
}

impl ConstantInfoTrait for ConstantClassInfo {
    const tag: u8 = ConstantInfoTag::CONSTANT_Class;

    fn parse(r: &mut ReadBuffer) -> Self {
        let name_index = r.read_u2() as usize;
        Self { name_index }
    }
}
