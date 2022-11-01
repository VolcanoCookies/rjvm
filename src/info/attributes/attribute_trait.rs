use crate::info::constant_pool::ConstantPool;
use crate::util::read_buffer::ReadBuffer;

pub trait AttributeTrait {
    const name: &'static str;
    fn parse(
        attribute_name_index: usize,
        attribute_length: usize,
        r: &mut ReadBuffer,
        constant_pool: &ConstantPool,
    ) -> Self;
}
