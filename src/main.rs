extern crate core;

use std::fs;
use std::sync::{Mutex, RwLock};

use crate::info::class_file::ClassFile;
use crate::runtime::class_loader::ClassLoader;
use crate::runtime::thread::Thread;
use crate::util::read_buffer::ReadBuffer;

pub mod info;
pub mod runtime;
pub mod r#type;
pub mod util;

static CLASS_LOADER: Mutex<ClassLoader> = Mutex::new(ClassLoader::new());

#[allow(unused_variables)]
fn main() {
    let class_file_path = "./src/Main.class";
    let class_bytes = fs::read(class_file_path).unwrap();
    let mut read_buf = ReadBuffer::new(class_bytes);

    let class_file = ClassFile::parse(&mut read_buf);

    let mut thread = Thread {
        program_counter: 0,
        stack: Vec::new(),
        heap: Vec::new(),
    };

    thread.run(&class_file);
}
