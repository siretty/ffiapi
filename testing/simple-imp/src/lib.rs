use ffiapi_v1_imp::obj::{Obj, ObjValueType};
use std::sync::Arc;

#[derive(Default)]
pub struct Thing {
    pub value: u64,
}

impl Thing {
    pub fn new(value: u64) -> Self { Self { value } }
}

impl ObjValueType for Thing {
    type Target = Arc<Self>;
}

#[no_mangle]
pub unsafe extern "C" fn simple_thing_create(_this: *mut Obj<Thing>) {
    assert!(!_this.is_null());
    assert!(_this.is_aligned());

    let obj_thing = _this.read();
    assert!(obj_thing.is_null());

    let thing = Thing::new(12345);

    let obj_thing = Obj::from_value(thing);
    _this.write(obj_thing);

    eprintln!("simple_thing_create");
}

#[no_mangle]
pub unsafe extern "C" fn simple_thing_destroy(_this: *mut Obj<Thing>) {
    assert!(!_this.is_null());
    assert!(_this.is_aligned());

    let ptr_thing = _this.read();
    assert!(!ptr_thing.is_null());

    _this.write(Obj::null());

    eprintln!("simple_thing_destroy");
}