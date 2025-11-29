use std::io::stdin;
use crate::modules::{Edgework, Module};
use crate::modules::button::ButtonColour::{Blue, White, Red, Yellow};
use crate::modules::button::ButtonText::{Abort, Detonate, Hold, Press};

enum ButtonColour {
    Blue,
    White,
    Red,
    Yellow,
}

enum ButtonText {
    Abort,
    Detonate,
    Hold,
    Press,
}

pub struct ButtonModule {
    colour: ButtonColour,
    text: ButtonText,
}
impl TryFrom<&str> for ButtonModule {
    type Error = String;

    fn try_from(button_info: &str) -> Result<Self, Self::Error> {
        let button_info = button_info.trim().to_lowercase();
        let mut split = button_info.split(' ');
        let colour = split.next();
        let text = split.next();
        let colour = match colour.expect("Should have colour").chars().next() {
            Some(c) => match c {
                'b' => Ok(Blue),
                'w' => Ok(White),
                'r' => Ok(Red),
                'y' => Ok(Yellow),
                _ => Err("Invalid Colour")
            },
            _ => Err("No Colour")
        }?;
        let text = match text.expect("Should have text").chars().next() {
            Some(c) => match c {
                'a' => Ok(Abort),
                'd' => Ok(Detonate),
                'h' => Ok(Hold),
                'p' => Ok(Press),
                _ => Err("Invalid text")
            },
            _ => Err("No Text")
        }?;
        Ok(ButtonModule {colour, text})
    }
}

impl Module for ButtonModule {
    type Output = String;
    fn get_info() -> Self {
        loop {
            let mut info = String::new();
            stdin().read_line(&mut info).expect("Valid String");
            match ButtonModule::try_from(info.as_str()) {
                Ok(b) => return b,
                Err(e) => println!("{e}")
            }
        }
    }

    fn solve(&self, edgework: Edgework) -> Self::Output {
        let hold= if matches!(self.colour, Blue) && matches!(self.text, Abort) {
            true
        } else if edgework.batteries > 1 && matches!(self.text, Detonate) {
            false
        } else if matches!(self.colour, White) && edgework.indicators.car {
            true
        } else if edgework.batteries > 2 && edgework.indicators.frk {
            false
        } else if matches!(self.colour, Yellow) {
            true
        } else {
            !(matches!(self.colour, Red) && matches!(self.text, Hold))
        };
        if hold {
            "Hold - Blue = 4, Yellow = 5, Other = 1"
        } else {
            "Release"
        }.to_string()
    }
}