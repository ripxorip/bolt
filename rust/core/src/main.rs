/* Unit-test file */
extern crate core;
use core::bolt::Bolt;

fn main() {
    /* There will always be a cwd */
    let mut bolt = Bolt::new();
    println!("--- Exp1 cwd ---");
    println!("{}", bolt.get_cwd(0));
    println!("--- Listing exp1 ---");
    // https://doc.rust-lang.org/1.8.0/book/references-and-borrowing.html#the-rules
    {
        let listing = bolt.get_raw_listing(0);
        for l in listing {
            println!("{}", l.name);
        }
    }
    println!("--- Exp2 cwd ---");
    bolt.cd(1, 2);
    println!("{}", bolt.get_cwd(1));
    println!("--- Listing exp2 ---");
    let listing = bolt.get_raw_listing(1);
    for l in listing {
        println!("{}", l.name);
    }
}
