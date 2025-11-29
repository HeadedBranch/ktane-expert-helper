pub mod password;
pub mod wires;
pub mod button;

pub trait Module {
    type Output;
    fn get_info() -> Self;
    fn solve(&self, edgework: Edgework) -> Self::Output;
}

pub struct Edgework {
    pub serial_vowel: bool,
    pub serial_even: bool,
    pub parallel_port: bool,
    pub batteries: u8,
    pub indicators: Indicators
}

pub struct Indicators {
    pub car: bool,
    pub frk: bool
}

impl Edgework {
    pub fn get_info() -> Edgework {
        use std::io;
        let mut input = String::new();
        println!("Is there a vowel in the serial number? (y/n)");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let serial_vowel = input.trim().to_lowercase() == "y";
        input.clear();

        println!("Is the last digit of the serial even? (y/n)");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let serial_even = input.trim().to_lowercase() == "y";
        input.clear();

        println!("Is there a Serial Port? (y/n)");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let parallel_port = input.trim().to_lowercase() == "y";
        input.clear();

        println!("Is there a CAR indicator (y/n)");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let car = input.trim().to_lowercase() == "y";
        input.clear();

        println!("Is there a FRK indicator (y/n)");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let frk = input.trim().to_lowercase() == "y";
        input.clear();

        println!("Enter Battery count");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let batteries: u8 = input.trim().parse().unwrap_or(0);
        input.clear();

        Edgework {
            serial_vowel,
            serial_even,
            parallel_port,
            batteries,
            indicators: Indicators {
                car,
                frk
            }
        }
    }
}
#[macro_export]
macro_rules! compare_count {
    ($operator:tt, $count:literal, $colour:path, $vector:expr) => {$vector.iter().filter(|w| matches!(w,$colour)).count() $operator $count};
}