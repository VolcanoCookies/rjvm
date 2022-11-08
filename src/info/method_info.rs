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
use crate::info::attributes::code_attribute::CodeAttribute;
use crate::util::read_buffer::ReadBuffer;

use super::constant_pool::ConstantPool;

#[derive(Clone)]
pub struct MethodInfo {
    pub access_flags: u16,
    pub name_index: usize,
    pub descriptor_index: usize,
    pub attributes_count: usize,
    pub attribute_info: Vec<AttributeInfo>,
    pub code: CodeAttribute,
}

impl MethodInfo {
    pub fn parse(r: &mut ReadBuffer, constant_pool: &ConstantPool) -> Self {
        let access_flags = r.read_u2();
        let name_index = r.read_u2() as usize;
        let descriptor_index = r.read_u2() as usize;
        let attributes_count = r.read_u2() as usize;

        let mut opt_code_attribute: Option<CodeAttribute> = None;

        let mut attribute_info = Vec::<AttributeInfo>::with_capacity(attributes_count);
        for _ in 0..attributes_count {
            let atr = AttributeInfo::parse(r, constant_pool);

            match atr {
                AttributeInfo::Code(code_attribute) => opt_code_attribute = Some(code_attribute),
                _ => attribute_info.push(atr),
            }
        }

        Self {
            access_flags,
            name_index,
            descriptor_index,
            attributes_count,
            attribute_info,
            code: opt_code_attribute.unwrap(),
        }
    }
}
