mod ptr;

use core::ffi::c_void;
use std::sync::Arc;

#[repr(C)]
#[derive(Default)]
pub struct Object([u8; 16]);

impl Object {
    pub fn into_arc<T>(self) -> Arc<T> {
        let bytes = self.0[0..core::mem::size_of::<usize>()]
            .try_into()
            .expect("ptrobj storage too small");

        let address = usize::from_le_bytes(bytes);

        let pointer = address as *const T;
        assert!(!pointer.is_null());
        assert!(pointer.is_aligned());

        let arc = unsafe { Arc::from_raw(pointer) };
        arc
    }
}

impl<T> From<Arc<T>> for Object {
    fn from(arc: Arc<T>) -> Self {
        let pointer = Arc::into_raw(arc);
        let address = pointer as usize;
        let bytes = address.to_le_bytes();

        let mut object = Object::default();
        object.0[0..core::mem::size_of::<usize>()].copy_from_slice(&bytes[0..core::mem::size_of::<usize>()]);
        object
    }
}

pub struct Unique<T>(Box<Storage<T>>);

pub struct Shared<T>(Arc<Storage<T>>);

#[repr(C)]
pub struct Storage<T>(Validator, T);

pub struct Validator {
    type_id: std::any::TypeId,
    thread_id: std::thread::ThreadId,
}