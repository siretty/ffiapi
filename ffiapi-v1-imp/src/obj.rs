use core::ffi::c_void;
use std::marker::PhantomData;
use std::sync::Arc;

pub trait ObjValueType {
    type Target;
}

pub trait ArcObjValueType {}

impl<T: ArcObjValueType> ObjValueType for T {
    type Target = Arc<Self>;
}

#[repr(C)]
pub struct Obj<T>(*mut c_void, PhantomData<T>);

impl<T> Obj<T> {
    pub fn null() -> Self { Obj(core::ptr::null_mut(), PhantomData) }

    pub fn is_null(&self) -> bool { self.0.is_null() }
}

impl<T> Obj<T>
where
    T: ObjValueType<Target=Arc<T>>,
{
    pub fn from_value(value: T) -> Self {
        let arc = Arc::new(value);
        Self::from_target(arc)
    }

    pub fn from_target(arc: Arc<T>) -> Self {
        let pointer = Arc::into_raw(arc);
        Obj(pointer as *mut c_void, PhantomData)
    }

    pub unsafe fn into_target(self) -> Arc<T> {
        let pointer = self.0 as *const T;
        let arc = Arc::from_raw(pointer);
        arc
    }
}

mod tests {
    #[test]
    fn arc_obj_value() {
        use super::{Obj, ArcObjValueType};

        struct ArcObjValue(i32);

        impl ArcObjValueType for ArcObjValue {}

        let obj = Obj::from_value(ArcObjValue(12345));
        let arc = unsafe { Obj::into_target(obj) };
        assert_eq!(arc.0, 12345);
    }
}