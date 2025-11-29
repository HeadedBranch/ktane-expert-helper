mod modules;
use crate::modules::{Edgework, Module};
use modules::wires::WireModule;

fn main() {
    let edgework = Edgework::get_info();
    let wires = WireModule::get_info();
    println!("{}", wires.solve(edgework))
}
