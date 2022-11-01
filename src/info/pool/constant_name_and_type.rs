/*
CONSTANT_NameAndType_info {
u1 tag;
u2 name_index;
u2 descriptor_index;
}
*/

use crate::info::pool::constant_info_trait::ConstantInfoTrait;

use super::constant_info_tag::ConstantInfoTag;

pub struct ConstantNameAndTypeInfo {
    pub name_index: usize,
    pub descriptor_index: usize,
}

impl ConstantInfoTrait for ConstantNameAndTypeInfo {
    const tag: u8 = ConstantInfoTag::CONSTANT_NameAndType;

    fn parse(r: &mut crate::util::read_buffer::ReadBuffer) -> Self {
        let name_index = r.read_u2() as usize;
        let descriptor_index = r.read_u2() as usize;

        Self {
            name_index,
            descriptor_index,
        }
    }
}
