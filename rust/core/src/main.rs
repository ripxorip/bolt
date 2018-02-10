/* Unit-test file */
extern crate core;
use core::bolt::Bolt;
use std::env;

fn main() {
    /* There will always be a cwd */
    let cwd = env::current_dir().unwrap();
    let bolt = Bolt::new(cwd);
    println!("{}", bolt.get_cwd_exp1());
}
