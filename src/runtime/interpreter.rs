use std::borrow::Borrow;
use std::rc::Rc;

use crate::info::attributes::attribute_info::AttributeInfo;
use crate::info::attributes::code_attribute::CodeAttribute;
use crate::info::class_file::ClassFile;
use crate::info::method_info::MethodInfo;
use crate::r#type::reference_type::ReferenceType;
use crate::r#type::reference_type::ReferenceType::Class;
use crate::r#type::type_value::TypeValue;
use crate::runtime::op_stack::OpStack;
use crate::runtime::stack_frame::StackFrame;
use crate::runtime::thread::Thread;

pub struct Interpreter {}

pub struct OpCode;

impl OpCode {
    pub const aload_0: u8 = 0x2a;
    pub const aload_1: u8 = 0x2b;
    pub const aload_2: u8 = 0x2c;
    pub const aload_3: u8 = 0x2d;
    pub const invokespecial: u8 = 0xb7;
    pub const return_: u8 = 0xb1;
}

impl Interpreter {
    fn get_code_attribute(method_info: &MethodInfo) -> &CodeAttribute {
        for attribute_info in &method_info.attribute_info {
            match attribute_info {
                AttributeInfo::Code(code_attribute) => return code_attribute,
                _ => {}
            }
        }
        panic!("method has no code attribute");
    }

    pub fn invoke_method(thread: &mut Thread, method_info: &MethodInfo, class_file: &ClassFile) {
        let pc = &mut thread.program_counter;
        let mut stack = &thread.stack;
        let mut heap = &mut thread.heap;

        let mut frame = stack.last().unwrap();
        let mut local_variables = &frame.local_variables;
        let mut op_stack = &frame.op_stack;

        let code_attribute = &method_info.code;

        local_variables.push(Rc::new(TypeValue::Reference(Class(class_file.this_class))));

        let mut op_stack = OpStack::new();

        while *pc < code_attribute.code_length {
            let op = code_attribute.code.get(*pc);
            *pc += 1;

            match op {
                OpCode::aload_0 => {
                    Self::a_load(0, &local_variables, &mut op_stack);
                }
                OpCode::aload_1 => {
                    Self::a_load(1, &local_variables, &mut op_stack);
                }
                OpCode::aload_2 => {
                    Self::a_load(2, &local_variables, &mut op_stack);
                }
                OpCode::aload_3 => {
                    Self::a_load(3, &local_variables, &mut op_stack);
                }
                OpCode::invokespecial => {
                    let index = code_attribute.code.get_2(*pc);

                    *pc += 2;
                }
                OpCode::return_ => {
                    // Check that the return type of the method is void
                    return;
                }
                _ => {
                    panic!("Unknown instruction {:#04x}", op);
                }
            }
        }
    }

    #[inline]
    fn a_load(index: usize, local_variables: &Vec<Rc<TypeValue>>, op_stack: &mut OpStack) {
        let obj = local_variables[index].clone();

        match obj.borrow() {
            TypeValue::Reference(_) => {
                op_stack.push(obj.clone());
            }
            _ => {
                panic!("aload_0 tried to load a local variable which is not a reference")
            }
        }
    }
}
