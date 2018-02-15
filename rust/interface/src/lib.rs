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
pub extern fn bolt_get_listing(ptr: *const Bolt, id: int32_t, offset: int32_t, dest: *mut int32_t, len: size_t) -> i32 {
    let bolt = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };
    let dest_slice = unsafe {
        assert!(!dest.is_null());
        slice::from_raw_parts_mut(dest, len as usize)
    };
    // FIXME below:
    // Get the listing in a vector
    // Copy vector to the slice
    // Return the number of listed elements
    bolt.get_listing(id, offset as usize, dest_slice)
}

#[no_mangle]
pub extern fn bolt_cd(ptr: *mut Bolt, id: int32_t, entry_id: int32_t) {
    let bolt = unsafe {
        assert!(!ptr.is_null());
        &mut*ptr
    };
    bolt.cd(id, entry_id);
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

#[no_mangle]
pub extern fn bolt_get_entry_type(ptr: *const Bolt, id: int32_t, entry_id: int32_t) -> *mut c_char {
    let bolt = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };
    let c_str_ret = CString::new(bolt.get_entry_type(id, entry_id)).unwrap();
    c_str_ret.into_raw()
}

#[no_mangle]
pub extern fn bolt_get_entry_name(ptr: *const Bolt, id: int32_t, entry_id: int32_t) -> *mut c_char {
    let bolt = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };
    let c_str_ret = CString::new(bolt.get_entry_name(id, entry_id)).unwrap();
    c_str_ret.into_raw()
}
