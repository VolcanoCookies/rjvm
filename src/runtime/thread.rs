use std::any::Any;
use std::rc::Rc;

use crate::info::attributes::attribute_info::AttributeInfo;
use crate::info::attributes::code_attribute::CodeAttribute;
use crate::info::class_file::ClassFile;
use crate::info::method_info::MethodInfo;
use crate::info::pool::constant_info::ConstantInfo;
use crate::info::pool::constant_info::ConstantInfo::Utf8;
use crate::r#type::reference_type::ReferenceType::{Class, Null};
use crate::r#type::type_value::TypeValue;
use crate::r#type::type_value::TypeValue::Reference;
use crate::runtime::interpreter::Interpreter;
use crate::runtime::stack_frame::StackFrame;
use crate::{cast, main};

pub struct Thread {
    pub program_counter: usize,
    pub stack: Vec<StackFrame>,
    pub heap: Vec<Box<dyn Any>>,
}

impl Thread {
    pub fn new() -> Self {
        Self {
            program_counter: 0,
            stack: Vec::new(),
            heap: Vec::new(),
        }
    }

    fn get_init_method(class_file: &ClassFile) -> Option<&MethodInfo> {
        for method_info in &class_file.method_info {
            match &class_file.constant_pool[method_info.name_index] {
                Utf8(utf8_info) => {
                    if utf8_info.str() == "<init>" {
                        return Some(method_info);
                    }
                }
                _ => {
                    panic!("Method name is not a utf8 constant");
                }
            }
        }
        return None;
    }

    pub fn run(&mut self, main_class_file: &ClassFile) {
        let main_method = Thread::get_init_method(main_class_file)?;
        Interpreter::invoke_method(main_method, main_class_file);
    }
}
