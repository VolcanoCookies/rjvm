#[non_exhaustive]
pub struct ConstantInfoTag;

impl ConstantInfoTag {
    pub const CONSTANT_Class: u8 = 7;
    pub const CONSTANT_Fieldref: u8 = 9;
    pub const CONSTANT_Methodref: u8 = 10;
    pub const CONSTANT_InterfaceMethodRef: u8 = 11;
    pub const CONSTANT_String: u8 = 8;
    pub const CONSTANT_Integer: u8 = 3;
    pub const CONSTANT_Float: u8 = 4;
    pub const CONSTANT_Long: u8 = 5;
    pub const CONSTANT_Double: u8 = 6;
    pub const CONSTANT_NameAndType: u8 = 12;
    pub const CONSTANT_Utf8: u8 = 1;
    pub const CONSTANT_MethodHandle: u8 = 15;
    pub const CONSTANT_MethodType: u8 = 16;
    pub const CONSTANT_Dynamic: u8 = 17;
    pub const CONSTANT_InvokeDynamic: u8 = 18;
    pub const CONSTANT_Module: u8 = 19;
    pub const CONSTANT_Package: u8 = 20;
}
