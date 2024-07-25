use core::ffi::c_void;

#[no_mangle]
pub unsafe extern "C" fn simple_thing_create(_this: *mut *mut c_void) {
    println!("simple_thing_create");
}

#[no_mangle]
pub unsafe extern "C" fn simple_thing_destroy(_this: *mut *mut c_void) {
    println!("simple_thing_destroy");
}