/*
SourceFile_attribute {
u2 attribute_name_index;
u4 attribute_length;
u2 sourcefile_index;
}
*/

use crate::info::attributes::attribute_info_name::AttributeInfoName;
use crate::info::attributes::attribute_trait::AttributeTrait;
use crate::{info::constant_pool::ConstantPool, util::read_buffer::ReadBuffer};

pub struct SourceFileAttribute {
    pub attribute_name_index: usize,
    pub attribute_length: usize,
    pub sourcefile_index: usize,
}

impl AttributeTrait for SourceFileAttribute {
    const name: &'static str = AttributeInfoName::SOURCE_FILE_ATTRIBUTE;

    fn parse(
        attribute_name_index: usize,
        attribute_length: usize,
        r: &mut ReadBuffer,
        _constant_pool: &ConstantPool,
    ) -> Self {
        let sourcefile_index = r.read_u2() as usize;

        Self {
            attribute_name_index,
            attribute_length,
            sourcefile_index,
        }
    }
}
