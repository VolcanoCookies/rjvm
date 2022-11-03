use std::any::{type_name, type_name_of_val, Any, TypeId};

pub struct DynVec {
    raw: Vec<(TypeId, Box<dyn Any>)>,
}

impl DynVec {
    pub fn new() -> Self {
        Self { raw: Vec::new() }
    }

    pub fn with_capacity(c: usize) -> Self {
        Self {
            raw: Vec::with_capacity(c),
        }
    }

    pub fn push<T>(&mut self, e: T)
    where
        T: Any,
    {
        let type_id = TypeId::of::<T>();
        println!("{:?}", type_name_of_val(&e));
        if e.type_id() == TypeId::of::<Box<dyn Any>>() {
            panic!("Cannot store Box<dyn Any> in DynVec");
        }
        self.raw.push((type_id, Box::new(e)));
    }

    pub fn push_box(&mut self, e: Box<dyn Any>) {
        self.raw.push(((&*e).type_id(), e));
    }

    pub fn pop<T>(&mut self) -> &T
    where
        T: Any,
    {
        let (type_id, boxed) = self.raw.pop().unwrap();
        if type_id != TypeId::of::<T>() {
            panic!("Invalid type");
        }
        *boxed.downcast().unwrap()
    }

    pub fn get<T>(&self, i: usize) -> &T
    where
        T: Any,
    {
        let (type_id, boxed) = self.raw.get(i).unwrap();
        let opt_unwrapped = boxed.downcast_ref::<T>();
        let boxed_type = type_name_of_val(&*boxed);
        if opt_unwrapped.is_none() {
            panic!(
                "Invalid boxed type, tried to unbox {:?} into {:?} at index {}",
                boxed_type,
                type_name::<T>(),
                i
            );
        }
        opt_unwrapped.unwrap()
    }
}
