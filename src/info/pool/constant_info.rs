use crate::info::pool::constant_class_info::ConstantClassInfo;
use crate::info::pool::constant_fieldref_info::ConstantFieldrefInfo;
use std::any::Any;

use crate::info::pool::constant_info_tag::ConstantInfoTag;
use crate::info::pool::constant_info_trait::ConstantInfoTrait;
use crate::info::pool::constant_methodref_info::ConstantMethodrefInfo;
use crate::info::pool::constant_name_and_type::ConstantNameAndTypeInfo;
use crate::info::pool::constant_string_info::ConstantStringInfo;
use crate::info::pool::constant_utf8_info::ConstantUtf8Info;
use crate::util::read_buffer::ReadBuffer;

pub struct ConstantInfo;

impl ConstantInfo {
    pub fn parse(r: &mut ReadBuffer) -> Box<dyn Any> {
        let tag = r.read_u1();
        println!("Parsing constant info tag {:#04x}", tag);
        match tag {
            ConstantInfoTag::CONSTANT_Utf8 => Box::new(ConstantUtf8Info::parse(r)),
            ConstantInfoTag::CONSTANT_Methodref => Box::new(ConstantMethodrefInfo::parse(r)),
            ConstantInfoTag::CONSTANT_Class => Box::new(ConstantClassInfo::parse(r)),
            ConstantInfoTag::CONSTANT_NameAndType => Box::new(ConstantNameAndTypeInfo::parse(r)),
            ConstantInfoTag::CONSTANT_Fieldref => Box::new(ConstantFieldrefInfo::parse(r)),
            ConstantInfoTag::CONSTANT_String => Box::new(ConstantStringInfo::parse(r)),
            _ => {
                let [a, b, c, d] = r.read();
                panic!("Invalid constant pool info tag {}, reading at index {}, next {:#02} {:#02} {:#02} {:#02}", tag, r.index, a, b, c, d);
            }
        }
    }
}
