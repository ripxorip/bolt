extern crate core;
use core::bolt::Bolt;

#[no_mangle]
pub extern fn bolt_new () -> *mut Bolt {
    Box::into_raw(Box::new(Bolt::new()))
}

#[no_mangle]
pub extern fn bolt_free (ptr: *mut Bolt) {
    if ptr.is_null() { return }
    unsafe { Box::from_raw(ptr); }
} 
