use std::any::{Any, TypeId};
use std::str::from_utf8;

use crate::cast;
use crate::info::attributes::attribute_info::AttributeInfo::{Code, LineNumberTable, SourceFile};
use crate::info::attributes::attribute_info_name::AttributeInfoName;
use crate::info::attributes::attribute_trait::AttributeTrait;
use crate::info::attributes::code_attribute::CodeAttribute;
use crate::info::attributes::constant_value_attribute::ConstantValueAttribute;
use crate::info::attributes::line_number_table_attribute::LineNumberTableAttribute;
use crate::info::attributes::source_file_attribute::SourceFileAttribute;
use crate::info::constant_pool::ConstantPool;
use crate::info::pool::constant_utf8_info::ConstantUtf8Info;
use crate::util::read_buffer::ReadBuffer;

pub enum AttributeInfo {
    Code(CodeAttribute),
    LineNumberTable(LineNumberTableAttribute),
    SourceFile(SourceFileAttribute),
    ConstantValue(ConstantValueAttribute),
}

impl AttributeInfo {
    pub fn parse(r: &mut ReadBuffer, constant_pool: &ConstantPool) -> AttributeInfo {
        let attribute_name_index = r.read_u2() as usize;
        let attribute_length = r.read_u4() as usize;

        let utf8_info: &ConstantUtf8Info = constant_pool.get(attribute_name_index);
        let name = from_utf8(&*utf8_info.bytes).unwrap();

        println!("{}", name);

        match name {
            AttributeInfoName::CODE_ATTRIBUTE => Code(CodeAttribute::parse(
                attribute_name_index,
                attribute_length,
                r,
                constant_pool,
            )),
            AttributeInfoName::LINE_NUMBER_TABLE_ATTRIBUTE => {
                LineNumberTable(LineNumberTableAttribute::parse(
                    attribute_name_index,
                    attribute_length,
                    r,
                    constant_pool,
                ))
            }
            AttributeInfoName::SOURCE_FILE_ATTRIBUTE => SourceFile(SourceFileAttribute::parse(
                attribute_name_index,
                attribute_length,
                r,
                constant_pool,
            )),
            _ => {
                panic!("Tried parsing unknown attribute info with name {}", name);
            }
        }
    }
}
