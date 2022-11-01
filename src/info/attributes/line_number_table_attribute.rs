/*
LineNumberTable_attribute {
u2 attribute_name_index;
u4 attribute_length;
u2 line_number_table_length;
{   u2 start_pc;
u2 line_number;
} line_number_table[line_number_table_length];
}
*/

use crate::info::attributes::attribute_info_name::AttributeInfoName;
use crate::info::attributes::attribute_trait::AttributeTrait;
use crate::{info::constant_pool::ConstantPool, util::read_buffer::ReadBuffer};

pub struct LineNumberTableEntry {
    pub start_pc: usize,
    pub line_number: usize,
}

pub struct LineNumberTableAttribute {
    pub attribute_name_index: usize,
    pub attribute_length: usize,
    pub line_number_table_length: usize,
    pub line_number_table: Vec<LineNumberTableEntry>,
}

impl AttributeTrait for LineNumberTableAttribute {
    const name: &'static str = AttributeInfoName::LINE_NUMBER_TABLE_ATTRIBUTE;

    fn parse(
        attribute_name_index: usize,
        attribute_length: usize,
        r: &mut ReadBuffer,
        _constant_pool: &ConstantPool,
    ) -> Self {
        let line_number_table_length = r.read_u2() as usize;
        let mut line_number_table =
            Vec::<LineNumberTableEntry>::with_capacity(line_number_table_length);
        for _ in 0..line_number_table_length {
            let start_pc = r.read_u2() as usize;
            let line_number = r.read_u2() as usize;

            let entry = LineNumberTableEntry {
                start_pc,
                line_number,
            };
            line_number_table.push(entry);
        }

        Self {
            attribute_name_index,
            attribute_length,
            line_number_table_length,
            line_number_table,
        }
    }
}
