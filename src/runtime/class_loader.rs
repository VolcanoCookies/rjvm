use std::collections::HashMap;
use std::fs;

use crate::info::attributes::attribute_info::AttributeInfo;
use crate::info::attributes::code_attribute::CodeAttribute;
use crate::info::class_file::ClassFile;
use crate::info::method_info::MethodInfo;
use crate::info::pool::constant_class_info::ConstantClassInfo;
use crate::info::pool::constant_utf8_info::ConstantUtf8Info;
use crate::util::read_buffer::ReadBuffer;

pub struct ClassLoader {
    classes: HashMap<&'static str, ClassFile>,
    methods: HashMap<&'static str, (CodeAttribute, MethodInfo)>,
}

unsafe impl Sync for ClassLoader {}

impl ClassLoader {
    pub fn new() -> Self {
        Self {
            classes: HashMap::new(),
            methods: HashMap::new(),
        }
    }

    pub fn load(&mut self, path: &str) -> &ClassFile {
        let class_bytes = fs::read(path).unwrap();
        let mut read_buf = ReadBuffer::new(class_bytes);

        let class_file = ClassFile::parse(&mut read_buf);

        for method_info in &class_file.method_info {
            for attribute_info in &method_info.attribute_info {
                match attribute_info {
                    AttributeInfo::Code(code_attribute) => {
                        let utf8_info: &ConstantUtf8Info =
                            class_file.constant_pool.get(method_info.name_index);
                        let method_name = utf8_info.static_str();
                        self.methods
                            .insert(method_name, (code_attribute.clone(), method_info.clone()));
                    }
                    _ => {}
                }
            }
        }

        let class_name = {
            let class_info: &ConstantClassInfo =
                class_file.constant_pool.get(class_file.this_class);
            let utf8_info: &ConstantUtf8Info = class_file.constant_pool.get(class_info.name_index);
            utf8_info.static_str()
        };

        self.classes.insert(class_name, class_file);

        self.classes.get(class_name).unwrap()
    }

    pub fn get_class(&self, name: &str) -> Option<&ClassFile> {
        self.classes.get(name)
    }
}
