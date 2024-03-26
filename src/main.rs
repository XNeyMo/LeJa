#![allow(warnings)]

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
struct Char {
    level: u8,
    japan: String,
    latin: String,
}

fn main() {
    let mut file = File::open("./data/chars.json").expect("JSON file not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Cannot read the file");

    let chars: serde_json::Value = serde_json::from_str(&data).expect("Error parsing JSON");

    let hiragana: Vec<Char> =
        serde_json::from_value(chars["hiragana"].clone()).expect("Error Hiragana");
    let katakana: Vec<Char> =
        serde_json::from_value(chars["katakana"].clone()).expect("Error Katakana");
}
