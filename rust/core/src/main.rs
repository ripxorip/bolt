/* Unit-test file */
extern crate core;
use core::bolt::Bolt;
use std::env;

fn main() {
    /* There will always be a cwd */
    let cwd = env::current_dir().unwrap();
    let mut bolt = Bolt::new(cwd);
    let r1 = &mut bolt;
    println!("--- Exp1 cwd ---");
    println!("{}", r1.get_cwd(0));
    println!("--- Listing exp1 ---");
    let files = r1.get_listing(0);
    for f in files {
        println!("{}", f);
    }
    println!("--- Exp2 cwd ---");
    r1.cd(1, -1);
    println!("{}", bolt.get_cwd(1));
    println!("--- Listing exp2 ---");
    let files = bolt.get_listing(1);
    for f in files {
        println!("{}", f);
    }
}
