use crate::r#type::r#type::Type;
use crate::r#type::type_value::TypeValue;
use crate::ReadBuffer;

pub struct IntType {
    pub value: i32,
}

impl Type for IntType {
    const value_type: u8 = TypeValue::CONST_PRIMITIVE_Int;

    fn parse(r: &mut ReadBuffer) -> Self {
        let value = r.read_u4() as i32;

        Self { value }
    }
}
