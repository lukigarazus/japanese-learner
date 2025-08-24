pub mod commands;
pub mod parser;

pub struct Word {
    vec: Vec<Char>,
}

pub struct Char {
    char: String,
    reading: String,
}

pub fn parse_word(word: &String) -> Word {
    let mut vec = Vec::new();
    for c in word.chars() {
        vec.push(Char {
            char: c.to_string(),
            reading: String::new(),
        });
    }
    Word { vec }
}
