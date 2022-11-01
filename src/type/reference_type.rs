use std::rc::Rc;

use crate::r#type::r#type::Type;
use crate::r#type::type_value::TypeValue;
use crate::util::read_buffer::ReadBuffer;

#[derive(Clone)]
pub enum NonArrayType {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Char,
    Float(f32),
    Double(f64),
    Boolean(bool),
    Reference(NonArrayReferenceType),
}

#[derive(Clone)]
pub enum NonArrayReferenceType {
    Null,
    Class,
    Interface,
}

#[derive(Clone)]
pub struct ArrayReferenceType {
    dimensions: usize,
    element_type: NonArrayType,
}

#[derive(Clone)]
pub enum ReferenceType {
    Null,
    Array(ArrayReferenceType),
    Class(usize),
}

impl Type for ReferenceType {
    const value_type: u8 = TypeValue::CONST_Reference;

    fn parse(r: &mut ReadBuffer) -> Self {
        todo!("Parse reference types")
    }
}
