use std::any::Any;

use crate::info::attributes::attribute_info::AttributeInfo;
use crate::info::pool::constant_info::ConstantInfo;
use crate::ReadBuffer;

use super::{constant_pool::ConstantPool, field_info::FieldInfo, method_info::MethodInfo};

pub struct ClassFile {
    pub magic: u32,
    pub major_version: u16,
    pub minor_version: u16,
    pub constant_pool_count: usize,
    pub constant_pool: ConstantPool,
    pub access_flags: u16,
    pub this_class: usize,
    pub super_class: usize,
    pub interface_count: usize,
    pub interfaces: Vec<usize>,
    pub field_count: usize,
    pub field_info: Vec<FieldInfo>,
    pub method_count: usize,
    pub method_info: Vec<MethodInfo>,
    pub attribute_count: usize,
    pub attributes: Vec<AttributeInfo>,
}

impl ClassFile {
    pub fn parse(r: &mut ReadBuffer) -> Self {
        let magic = r.read_u4();
        let major_version = r.read_u2();
        let minor_version = r.read_u2();
        let constant_pool_count = r.read_u2() as usize - 1;

        println!(
            "Magic: {:#04x}, Version: {}.{}, Constants: {}",
            magic, major_version, minor_version, constant_pool_count
        );

        let mut constant_pool = ConstantPool::new(constant_pool_count);
        for _ in 0..constant_pool_count {
            let cpi = ConstantInfo::parse(r);
            constant_pool.push_box(cpi);
        }
        let access_flags = r.read_u2();
        let this_class = r.read_u2() as usize;
        let super_class = r.read_u2() as usize;
        let interface_count = r.read_u2() as usize;
        let mut interfaces = Vec::<usize>::with_capacity(interface_count);
        for _ in 0..interface_count {
            let inf_i = r.read_u2() as usize;
            interfaces.push(inf_i);
        }
        let field_count = r.read_u2() as usize;
        let mut field_info = Vec::<FieldInfo>::with_capacity(field_count);
        for _ in 0..field_count {
            let fld = FieldInfo::parse(r, &constant_pool);
            field_info.push(fld);
        }
        let method_count = r.read_u2() as usize;
        let mut method_info = Vec::<MethodInfo>::with_capacity(method_count);
        for _ in 0..method_count {
            let mth = MethodInfo::parse(r, &constant_pool);
            method_info.push(mth);
        }
        let attribute_count = r.read_u2() as usize;
        let mut attributes = Vec::<AttributeInfo>::with_capacity(attribute_count);
        for _ in 0..attribute_count {
            let atr = AttributeInfo::parse(r, &constant_pool);
            attributes.push(atr);
        }

        Self {
            magic,
            major_version,
            minor_version,
            constant_pool_count,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interface_count,
            interfaces,
            field_count,
            field_info,
            method_count,
            method_info,
            attribute_count,
            attributes,
        }
    }
}
