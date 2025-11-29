use crate::modules::wires::WireColour::{Black, Blue, Red, White, Yellow};
use crate::modules::wires::WireCount::{Five, Four, Six, Three};
use crate::modules::{Edgework, Module};
use std::io::stdin;
use crate::compare_count;

enum WireColour {
    Red,
    Blue,
    Yellow,
    White,
    Black,
}

enum WireCount {
    Three,
    Four,
    Five,
    Six,
}

pub struct WireModule {
    count: WireCount,
    wires: Vec<WireColour>,
}

impl Module for WireModule {
    type Output = String;
    fn get_info() -> Self {
        loop {
            let mut wires = String::new();
            stdin().read_line(&mut wires).expect("Valid String");
            match WireModule::try_from(wires.as_str()) {
                Ok(m) => return m,
                Err(e) => println!("{e}")
            }
        }
    }
    fn solve(&self, edgework: Edgework) -> Self::Output {
        match self.count {
            Three => {
                if compare_count!(==, 0, Red, self.wires) {
                    "Second"
                } else if matches!(self.wires.last(), Some(White)) {
                    "Last"
                } else if compare_count!( >, 1, Blue, self.wires) {
                    "Last Blue"
                } else {
                    "Last"
                }
            }
            Four => {
                if compare_count!(>, 1, Red, self.wires) && !edgework.serial_even {
                    "Last Red"
                } else if matches!(self.wires.last(), Some(Yellow))
                    && compare_count!(==, 0, Red, self.wires)
                    || compare_count!(==, 1, Blue, self.wires)
                {
                    "First"
                } else if compare_count!(>, 1, Yellow, self.wires) {
                    "Last"
                } else {
                    "Second"
                }
            }
            Five => {
                if matches!(self.wires.last(), Some(Black)) && !edgework.serial_even {
                    "Fourth"
                } else if compare_count!(==, 1, Red, self.wires)
                    && compare_count!(>, 1, Yellow, self.wires)
                {
                    "First"
                } else if compare_count!(==, 0, Black, self.wires) {
                    "Second"
                } else {
                    "First"
                }
            }
            Six => {
                if compare_count!(==, 0, Yellow, self.wires) && !edgework.serial_even {
                    "Third"
                } else if compare_count!(==, 1, Yellow, self.wires) {
                    "Fourth"
                } else if compare_count!(==, 0, Red, self.wires) {
                    "Last"
                } else {
                    "Fourth"
                }
            }
        }.to_string()
    }
}
impl TryFrom<&str> for WireModule {
    type Error = String;
    fn try_from(wires: &str) -> Result<Self, Self::Error> {
        let wires = wires.trim().to_lowercase();
        let count = match wires.len() {
            ..3 => Err("Too Short"),
            3 => Ok(Three),
            4 => Ok(Four),
            5 => Ok(Five),
            6 => Ok(Six),
            7.. => Err("Too long"),
        }?;
        let wirechars = wires.chars();
        let mut wires = vec![];
        for c in wirechars {
            wires.push(match c {
                'r' => Ok(Red),
                'b' => Ok(Blue),
                'y' => Ok(Yellow),
                'w' => Ok(White),
                'l' => Ok(Black),
                _ => Err("Invalid character, Must be one of [R,B,Y,W,L]"),
            }?)
        }
        Ok(WireModule {count, wires})
    }
}

#[cfg(test)]
mod tests {
    use crate::modules::{Edgework, Indicators, Module};
    use crate::modules::wires::WireModule;
    macro_rules! gen_test {
    ($name:ident, $wires:literal,$edgework:path, $result:literal) => {
        #[test]
        fn $name() -> Result<(),String> {
            let wires = WireModule::try_from($wires)?;
            assert_eq!(wires.solve($edgework), $result);
            Ok(())
        }
    };
}

    const EDGEWORK_ODD:Edgework = Edgework { serial_even: false, serial_vowel: true, parallel_port: true, batteries: 0, indicators: Indicators {car:false,frk:false} };
    const EDGEWORK_EVEN:Edgework = Edgework { serial_even: true, serial_vowel: true, parallel_port: true, batteries: 0, indicators: Indicators {car:false,frk:false} };

    // Three Wires tests:
    gen_test!(test_31, "bby", EDGEWORK_ODD, "Second");
    gen_test!(test_32, "rbw", EDGEWORK_ODD, "Last");
    gen_test!(test_33, "rbb", EDGEWORK_ODD, "Last Blue");
    gen_test!(test_34, "rry", EDGEWORK_ODD, "Last");
    // Four Wires tests:
    gen_test!(test_41, "rryy", EDGEWORK_ODD, "Last Red");
    gen_test!(test_42, "lbby", EDGEWORK_EVEN, "First");
    gen_test!(test_43, "rryb", EDGEWORK_EVEN, "First");
    gen_test!(test_44, "rlyy", EDGEWORK_EVEN, "Last");
    gen_test!(test_45, "rwwy", EDGEWORK_EVEN, "Second");
    // Five Wires tests:
    gen_test!(test_51, "rbywl", EDGEWORK_ODD, "Fourth");
    gen_test!(test_52, "rbyyw", EDGEWORK_EVEN, "First");
    gen_test!(test_53, "rywbw", EDGEWORK_ODD, "Second");
    gen_test!(test_54, "ryblw", EDGEWORK_EVEN, "First");
    // Six Wires tests:
    gen_test!(test_61, "wwwwww", EDGEWORK_ODD, "Third");
    gen_test!(test_62, "wwwwwy", EDGEWORK_EVEN, "Fourth");
    gen_test!(test_63, "wwyybl", EDGEWORK_ODD, "Last");
    gen_test!(test_64, "wryybl", EDGEWORK_EVEN, "Fourth");
}