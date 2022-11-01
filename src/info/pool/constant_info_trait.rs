use crate::util::read_buffer::ReadBuffer;
use std::task::ready;

pub trait ConstantInfoTrait {
    const tag: u8;
    fn parse(r: &mut ReadBuffer) -> Self;
}
