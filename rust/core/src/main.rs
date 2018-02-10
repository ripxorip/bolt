/* Unit-test file */
extern crate core;
use core::bolt::Bolt;
use std::env;

fn main() {
    /* There will always be a cwd */
    let cwd = env::current_dir().unwrap();
    let mut bolt = Bolt::new(cwd);
    println!("--- Exp1 cwd ---");
    println!("{}", bolt.get_cwd(0));
    println!("--- Listing exp1 ---");
    // https://doc.rust-lang.org/1.8.0/book/references-and-borrowing.html#the-rules
    {
        let files = bolt.get_listing(0);
        for f in files {
            println!("{}", f);
        }
    }
    println!("--- Exp2 cwd ---");
    bolt.cd(1, -1);
    println!("{}", bolt.get_cwd(1));
    println!("--- Listing exp2 ---");
    let files = bolt.get_listing(1);
    for f in files {
        println!("{}", f);
    }
}
