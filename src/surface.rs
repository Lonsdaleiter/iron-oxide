use crate::{handle, Object, ObjectPointer};

pub struct CAMetalLayer(ObjectPointer);
handle!(CAMetalLayer);

impl CAMetalLayer {
    // TODO implement
}

impl Object for CAMetalLayer {
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self
    where
        Self: Sized,
    {
        CAMetalLayer(ptr)
    }

    fn get_ptr(&self) -> ObjectPointer {
        self.0
    }
}
