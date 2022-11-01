/*
CONSTANT_String_info {
u1 tag;
u2 string_index;
}
*/

use crate::info::pool::constant_info_trait::ConstantInfoTrait;

use super::constant_info_tag::ConstantInfoTag;

pub struct ConstantStringInfo {
    string_index: usize,
}

impl ConstantInfoTrait for ConstantStringInfo {
    const tag: u8 = ConstantInfoTag::CONSTANT_String;

    fn parse(r: &mut crate::util::read_buffer::ReadBuffer) -> Self {
        let string_index = r.read_u2() as usize;

        Self { string_index }
    }
}
