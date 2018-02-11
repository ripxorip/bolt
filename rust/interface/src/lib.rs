extern crate core;
extern crate libc;

use core::bolt::Bolt;
use libc::{c_char, int32_t};
use std::ffi::CString;

#[no_mangle]
pub extern fn bolt_new () -> *mut Bolt {
    Box::into_raw(Box::new(Bolt::new()))
}

#[no_mangle]
pub extern fn bolt_free (ptr: *mut Bolt) {
    if ptr.is_null() { return }
    unsafe { Box::from_raw(ptr); }
} 

#[no_mangle]
pub extern fn bolt_get_cwd(ptr: *const Bolt, id: int32_t) -> *mut c_char {
    let bolt = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };
    let c_str_ret = CString::new(bolt.get_cwd(id)).unwrap();
    c_str_ret.into_raw()
}
