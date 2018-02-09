/* Unit-test file */
extern crate core;
use core::bolt::Bolt;

fn main() {
    let bolt = Bolt::new("./bolt/is/awsome/".to_string());
    println!("{}", bolt.get_cwd_exp1());
}
