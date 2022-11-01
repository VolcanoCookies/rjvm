use crate::ReadBuffer;

pub trait Type {
    const value_type: u8;
    fn parse(r: &mut ReadBuffer) -> Self;
}
