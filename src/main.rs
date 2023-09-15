use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Read};
use rand::seq::SliceRandom;

#[derive(Debug, Deserialize, Serialize)]
struct Word {
    latin: String,
    symbol: String,
    meaning: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct JapaneseGroup {
    group: String,
    words: Vec<Word>,
}

#[derive(Debug, Deserialize, Serialize)]
struct LanguageData {
    hiragana: Vec<JapaneseGroup>,
    katakana: Vec<JapaneseGroup>,
    kanji: Vec<JapaneseGroup>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("data/words.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let data: LanguageData = serde_json::from_str(&contents)?;

    let hiragana_groups = &data.hiragana;
    let katakana_groups = &data.katakana;
    let kanji_groups = &data.kanji;

    println!("{:?}", hiragana_groups);
    println!("{:?}", katakana_groups);
    println!("{:?}", kanji_groups);

    Ok(())
}