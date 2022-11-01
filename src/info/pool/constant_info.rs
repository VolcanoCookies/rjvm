use crate::info::pool::constant_class_info::ConstantClassInfo;
use crate::info::pool::constant_fieldref_info::ConstantFieldrefInfo;
use crate::info::pool::constant_info::ConstantInfo::{
    Class, Fieldref, Methodref, NameAndType, String, Utf8,
};
use crate::info::pool::constant_info_tag::ConstantInfoTag;
use crate::info::pool::constant_info_trait::ConstantInfoTrait;
use crate::info::pool::constant_methodref_info::ConstantMethodrefInfo;
use crate::info::pool::constant_name_and_type::ConstantNameAndTypeInfo;
use crate::info::pool::constant_string_info::ConstantStringInfo;
use crate::info::pool::constant_utf8_info::ConstantUtf8Info;
use crate::util::read_buffer::ReadBuffer;

pub enum ConstantInfo {
    String(ConstantStringInfo),
    Utf8(ConstantUtf8Info),
    Methodref(ConstantMethodrefInfo),
    NameAndType(ConstantNameAndTypeInfo),
    Fieldref(ConstantFieldrefInfo),
    Class(ConstantClassInfo),
}

impl ConstantInfo {
    pub fn parse(r: &mut ReadBuffer) -> ConstantInfo {
        let tag = r.read_u1();
        println!("Parsing constant info tag {:#04x}", tag);
        match tag {
            ConstantInfoTag::CONSTANT_Utf8 => Utf8(ConstantUtf8Info::parse(r)),
            ConstantInfoTag::CONSTANT_Methodref => Methodref(ConstantMethodrefInfo::parse(r)),
            ConstantInfoTag::CONSTANT_Class => Class(ConstantClassInfo::parse(r)),
            ConstantInfoTag::CONSTANT_NameAndType => NameAndType(ConstantNameAndTypeInfo::parse(r)),
            ConstantInfoTag::CONSTANT_Fieldref => Fieldref(ConstantFieldrefInfo::parse(r)),
            ConstantInfoTag::CONSTANT_String => String(ConstantStringInfo::parse(r)),
            _ => {
                let [a, b, c, d] = r.read();
                panic!("Invalid constant pool info tag {}, reading at index {}, next {:#02} {:#02} {:#02} {:#02}", tag, r.index, a, b, c, d);
            }
        }
    }
}
