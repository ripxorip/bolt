extern crate core;
extern crate libc;

use core::bolt::Bolt;
use libc::{c_char, int32_t, size_t};
use std::ffi::CString;
use std::slice;

// Helpers
#[no_mangle]
pub extern fn bolt_free_string (s: *mut c_char) {
    unsafe {
        if s.is_null() { return }
        CString::from_raw(s)
    };
}

// Class implementation
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
pub extern fn bolt_get_listing(ptr: *const Bolt, id: int32_t, dest: *mut int32_t, len: size_t) {
    let bolt = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };
    let dest_slice = unsafe {
        assert!(!dest.is_null());
        slice::from_raw_parts_mut(dest, len as usize)
    };

    let v = bolt.get_listing(id);
    let mut i = 0;
    while i < len && i < v.len(){
        dest_slice[i] = v[i].id;
        i += 1;
    }
}

#[no_mangle]
pub extern fn bolt_get_num_entries(ptr: *const Bolt, id: int32_t) -> int32_t {
    let bolt = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };
    bolt.get_num_entries(id)
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
