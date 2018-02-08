extern crate core;

#[no_mangle]
pub extern "C" fn get_dummy_listing() -> String {
    core::test()
}
