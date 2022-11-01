use std::any::Any;

pub struct Heap {
    boxed: Vec<Box<dyn Any>>,
}

impl Heap {
    pub fn unbox<T: Any>(&self, index: usize) -> &T {
        self.boxed[index].downcast_ref::<T>().unwrap()
    }
}
