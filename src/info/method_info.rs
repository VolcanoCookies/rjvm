/*
method_info {
u2             access_flags;
u2             name_index;
u2             descriptor_index;
u2             attributes_count;
attribute_info attributes[attributes_count];
}
*/

use crate::info::attributes::attribute_info::AttributeInfo;
use crate::util::read_buffer::ReadBuffer;

use super::constant_pool::ConstantPool;

pub struct MethodInfo {
    pub access_flags: u16,
    pub name_index: usize,
    pub descriptor_index: usize,
    pub attributes_count: usize,
    pub attribute_info: Vec<AttributeInfo>,
}

impl MethodInfo {
    pub fn parse(r: &mut ReadBuffer, constant_pool: &ConstantPool) -> Self {
        let access_flags = r.read_u2();
        let name_index = r.read_u2() as usize;
        let descriptor_index = r.read_u2() as usize;
        let attributes_count = r.read_u2() as usize;
        let mut attribute_info = Vec::<AttributeInfo>::with_capacity(attributes_count);
        for _ in 0..attributes_count {
            let atr = AttributeInfo::parse(r, constant_pool);
            attribute_info.push(atr);
        }

        Self {
            access_flags,
            name_index,
            descriptor_index,
            attributes_count,
            attribute_info,
        }
    }
}
