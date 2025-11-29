mod modules;
use crate::modules::{Edgework, Indicators, Module};
use crate::modules::password::PasswordModule;

fn main() {
    let edgework = Edgework {
        serial_even: true,
        serial_vowel: true,
        parallel_port: true,
        batteries: 0,
        indicators: Indicators {
            car: true,
            frk: false
        }
    };
    let module = PasswordModule::get_info();
    println!("{}", module.solve(edgework))
}
