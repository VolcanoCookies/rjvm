/*
ConstantValue_attribute {
u2 attribute_name_index;
u4 attribute_length;
u2 constantvalue_index;
}
*/

use crate::info::attributes::attribute_info_name::AttributeInfoName;
use crate::info::attributes::attribute_trait::AttributeTrait;
use crate::{info::constant_pool::ConstantPool, util::read_buffer::ReadBuffer};

pub struct ConstantValueAttribute {
    attribute_name_index: usize,
    attribute_length: usize,
    constant_value_index: usize,
}

impl AttributeTrait for ConstantValueAttribute {
    const name: &'static str = "undefined";

    fn parse(
        attribute_name_index: usize,
        attribute_length: usize,
        r: &mut ReadBuffer,
        _constant_pool: &ConstantPool,
    ) -> Self {
        let constant_value_index = r.read_u2() as usize;

        Self {
            attribute_name_index,
            attribute_length,
            constant_value_index,
        }
    }
}
