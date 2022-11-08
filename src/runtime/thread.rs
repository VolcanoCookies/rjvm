use crate::info::class_file::ClassFile;
use crate::info::method_info::MethodInfo;
use crate::info::pool::constant_utf8_info::ConstantUtf8Info;
use crate::runtime::interpreter::Interpreter;
use crate::runtime::stack_frame::StackFrame;
use std::any::Any;

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
            let utf8_info: &ConstantUtf8Info = class_file.constant_pool.get(method_info.name_index);
            if utf8_info.str() == "<init>" {
                return Some(method_info);
            }
        }
        return None;
    }

    pub fn run(&mut self, main_class_file: &ClassFile) {
        let opt_main_method = Thread::get_init_method(main_class_file);
        if opt_main_method.is_none() {
            panic!("Main class does not have a <init> method");
        }
        let main_method = opt_main_method.unwrap();
        Interpreter::invoke_method(&mut self, main_method, main_class_file);
    }
}
