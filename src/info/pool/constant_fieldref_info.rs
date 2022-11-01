/*
CONSTANT_Fieldref_info {
u1 tag;
u2 class_index;
u2 name_and_type_index;
}
*/

use crate::info::pool::constant_info_trait::ConstantInfoTrait;
use crate::util::read_buffer::ReadBuffer;

use super::constant_info_tag::ConstantInfoTag;

pub struct ConstantFieldrefInfo {
    class_index: usize,
    name_and_type_index: usize,
}

impl ConstantInfoTrait for ConstantFieldrefInfo {
    const tag: u8 = ConstantInfoTag::CONSTANT_Fieldref;

    fn parse(r: &mut ReadBuffer) -> Self {
        let class_index = r.read_u2() as usize;
        let name_and_type_index = r.read_u2() as usize;

        Self {
            class_index,
            name_and_type_index,
        }
    }
}
