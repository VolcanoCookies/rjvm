use std::ffi::c_void;
use std::ops::Index;
use std::rc::Rc;

use crate::r#type::type_value::TypeValue;

pub struct OpStack {
    stack: Vec<Rc<TypeValue>>,
}

impl OpStack {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push(&mut self, rc_v: Rc<TypeValue>) {
        &mut self.stack.push(rc_v);
    }
    
    pub fn pop(&mut self) -> Rc<TypeValue> {
        self.stack.pop().unwrap()
    }
}

impl Index<usize> for OpStack {
    type Output = TypeValue;

    fn index(&self, index: usize) -> &Self::Output {
        &self.stack[index]
    }
}
