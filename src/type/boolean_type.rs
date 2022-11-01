use crate::r#type::r#type::Type;
use crate::r#type::type_value::TypeValue;
use crate::ReadBuffer;

pub struct BooleanType {
    pub value: bool,
}

impl Type for BooleanType {
    const value_type: u8 = TypeValue::CONST_PRIMITIVE_Boolean;

    fn parse(r: &mut ReadBuffer) -> Self {
        let i = r.read_u4() as i32;
        let value = match i {
            0 => false,
            1 => true,
            _ => {
                panic!("Invalid serialized value ({}) for boolean type", i);
            }
        };

        Self { value }
    }
}
