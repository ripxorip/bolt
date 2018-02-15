/* Unit-test file */
extern crate core;
use core::tui::Tui;

fn main() {
    let mut tui = Tui::new(32, 32);
    println!("{}", tui.draw());
}
