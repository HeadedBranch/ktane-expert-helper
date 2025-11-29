mod modules;
use crate::modules::{Edgework, Module};
use crate::modules::button::ButtonModule;

fn main() {
    let edgework = Edgework::get_info();
    let module = ButtonModule::get_info();
    println!("{}", module.solve(edgework))
}
