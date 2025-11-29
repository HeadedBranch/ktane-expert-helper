use std::io::stdin;
use crate::modules::{Edgework, Module};

const WORDS: [&str; 35] = [
    "about", "after", "again", "below", "could", "every", "first", "found", "great", "house",
    "large", "learn", "never", "other", "place", "plant", "point", "right", "small", "sound",
    "spell", "still", "study", "their", "there", "these", "thing", "think", "three", "water",
    "where", "which", "world", "would", "write",
];

pub struct PasswordModule {
    col_1: Option<Vec<char>>,
    col_2: Option<Vec<char>>,
    col_3: Option<Vec<char>>,
    col_4: Option<Vec<char>>,
    col_5: Option<Vec<char>>,
}

macro_rules! get_inputs_as_chars {
    ($($name: ident),*) => {
        let mut input = String::new();
        $(
            stdin().read_line(&mut input).expect("Valid String");
            let $name: Vec<char> = input.trim().to_lowercase().chars().collect();
            let $name = if $name.is_empty() {
                None
            } else { Some($name) };
            input.clear();
        )*
    };
}

macro_rules! get_word_chars {
    ($chars:expr,$($name:ident),*) => {
        $(
            let $name = $chars.next().expect("Word is missing characters");
        )*
    };
}

impl Module for PasswordModule {
    type Output = String;

    fn get_info() -> Self {
        get_inputs_as_chars!(col_1,col_2,col_3,col_4,col_5);
        PasswordModule {col_1, col_2, col_3, col_4, col_5}
    }

    fn solve(&self, _: Edgework) -> Self::Output {
        let possible_words = WORDS.iter();
        let possible_words = possible_words.filter(
            |w| {
                let mut chars = w.chars();
                get_word_chars!(chars,c1,c2,c3,c4,c5);
                self.col_1.as_ref().is_none_or(|c| c.contains(&c1)) &&
                self.col_2.as_ref().is_none_or(|c| c.contains(&c2)) &&
                self.col_3.as_ref().is_none_or(|c| c.contains(&c3)) &&
                self.col_4.as_ref().is_none_or(|c| c.contains(&c4)) &&
                self.col_5.as_ref().is_none_or(|c| c.contains(&c5))
            });

            possible_words
            .map(|s| &**s)
            .collect::<Vec<&str>>().join(", ")
    }
}