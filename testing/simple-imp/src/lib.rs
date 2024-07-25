use core::ffi::c_void;

#[no_mangle]
pub unsafe extern "C" fn simple_thing_create(_this: *mut *mut c_void) {
    assert!(!_this.is_null());
    assert!(_this.is_aligned());
    _this.write(12345 as *mut c_void);
    println!("simple_thing_create");
}

#[no_mangle]
pub unsafe extern "C" fn simple_thing_destroy(_this: *mut *mut c_void) {
    assert!(!_this.is_null());
    assert!(_this.is_aligned());
    assert_eq!(_this.read() as usize, 12345);
    println!("simple_thing_destroy");
}