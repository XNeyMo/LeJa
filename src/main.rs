#![allow(warnings)]

use serde::Deserialize;

use std::fs::File;
use std::path::Path;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Char {
    level: u8,
    japan: String,
    latin: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Kana {
    chars: Vec<Char>,
}

fn main() {
    let json_file_path = Path::new("./data/chars.json");
    let file = File::open(json_file_path).expect("file not found");

    let japanese: Vec<Kana> = serde_json::from_reader(file).unwrap();
}
