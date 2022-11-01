use crate::info::attributes::code_attribute::CodeAttribute;
use crate::info::attributes::line_number_table_attribute::LineNumberTableAttribute;
use crate::info::attributes::source_file_attribute::SourceFileAttribute;

pub struct AttributeInfoName;

impl AttributeInfoName {
    pub const CODE_ATTRIBUTE: &'static str = "Code";
    pub const LINE_NUMBER_TABLE_ATTRIBUTE: &'static str = "LineNumberTable";
    pub const SOURCE_FILE_ATTRIBUTE: &'static str = "SourceFile";
}
