use crate::r#type::reference_type::ReferenceType;

pub enum TypeValue {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Char,
    Float(f32),
    Double(f64),
    Boolean(bool),
    ReturnAddress,
    Reference(ReferenceType),
}

impl TypeValue {
    pub const CONST_PRIMITIVE_Byte: u8 = 0;
    pub const CONST_PRIMITIVE_Short: u8 = 1;
    pub const CONST_PRIMITIVE_Int: u8 = 2;
    pub const CONST_PRIMITIVE_Long: u8 = 3;
    pub const CONST_PRIMITIVE_Char: u8 = 4;
    pub const CONST_PRIMITIVE_Float: u8 = 5;
    pub const CONST_PRIMITIVE_Double: u8 = 6;
    pub const CONST_PRIMITIVE_Boolean: u8 = 7;
    pub const CONST_PRIMITIVE_ReturnAddress: u8 = 8;
    pub const CONST_Reference: u8 = 9;
}
