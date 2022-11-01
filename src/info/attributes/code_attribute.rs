/*
Code_attribute {
u2 attribute_name_index;
u4 attribute_length;
u2 max_stack;
u2 max_locals;
u4 code_length;
u1 code[code_length];
u2 exception_table_length;
{   u2 start_pc;
u2 end_pc;
u2 handler_pc;
u2 catch_type;
} exception_table[exception_table_length];
u2 attributes_count;
attribute_info attributes[attributes_count];
}
*/

use crate::info::attributes::attribute_info::AttributeInfo;
use crate::info::attributes::attribute_info_name::AttributeInfoName;
use crate::info::attributes::attribute_trait::AttributeTrait;
use crate::util::bytes::Bytes;
use crate::{info::constant_pool::ConstantPool, util::read_buffer::ReadBuffer};

pub struct ExceptionTableEntry {
    pub start_pc: usize,
    pub end_pc: usize,
    pub handler_pc: usize,
    pub catch_type: usize,
}

pub struct CodeAttribute {
    pub attribute_name_index: usize,
    pub attribute_length: usize,
    pub max_stack: usize,
    pub max_locals: usize,
    pub code_length: usize,
    pub code: Bytes,
    pub exception_table_length: usize,
    pub exception_table: Vec<ExceptionTableEntry>,
    pub attributes_count: usize,
    pub attributes: Vec<AttributeInfo>,
}

impl AttributeTrait for CodeAttribute {
    const name: &'static str = AttributeInfoName::CODE_ATTRIBUTE;

    fn parse(
        attribute_name_index: usize,
        attribute_length: usize,
        r: &mut ReadBuffer,
        constant_pool: &ConstantPool,
    ) -> Self {
        let max_stack = r.read_u2() as usize;
        let max_locals = r.read_u2() as usize;
        let code_length = r.read_u4() as usize;
        let code = Bytes::new(r.read_n(code_length));
        let exception_table_length = r.read_u2() as usize;
        let mut exception_table = Vec::<ExceptionTableEntry>::with_capacity(exception_table_length);
        for _ in 0..exception_table_length {
            let start_pc = r.read_u2() as usize;
            let end_pc = r.read_u2() as usize;
            let handler_pc = r.read_u2() as usize;
            let catch_type = r.read_u2() as usize;
            let exception_table_entry = ExceptionTableEntry {
                start_pc,
                end_pc,
                handler_pc,
                catch_type,
            };

            exception_table.push(exception_table_entry);
        }
        let attributes_count = r.read_u2() as usize;
        let mut attributes = Vec::<AttributeInfo>::with_capacity(attributes_count);
        for _ in 0..attributes_count {
            let atr = AttributeInfo::parse(r, constant_pool);
            attributes.push(atr);
        }

        Self {
            attribute_name_index,
            attribute_length,
            max_stack,
            max_locals,
            code_length,
            code,
            exception_table_length,
            exception_table,
            attributes_count,
            attributes,
        }
    }
}
