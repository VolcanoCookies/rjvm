use crate::r#type::type_value::TypeValue;
use crate::runtime::op_stack::OpStack;
use std::rc::Rc;

pub struct StackFrame {
    // Index into constant pool for the class containing the method this stack is for
    pub this_class: usize,
    pub local_variables: Vec<Rc<TypeValue>>,
    // LI-FI operand stack
    pub op_stack: OpStack,
}

impl StackFrame {
    pub fn new(this_class: usize, local_variables: Vec<Rc<TypeValue>>) -> Self {
        Self {
            this_class,
            local_variables,
            op_stack: OpStack::new(),
        }
    }
}
