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

impl LanguageData {
    fn run() -> Option<(Vec<JapaneseGroup>, Vec<JapaneseGroup>, Vec<JapaneseGroup>)> {
        let mut file = File::open("data/words.json").ok()?;
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_err() {
            return None;
        }

        let data: LanguageData = serde_json::from_str(&contents).ok()?;

        let hiragana_groups = data.hiragana;
        let katakana_groups = data.katakana;
        let kanji_groups = data.kanji;

        Some((hiragana_groups, katakana_groups, kanji_groups))
    }
}

fn main() {
    if let Some((hiragana_groups, katakana_groups, kanji_groups)) = LanguageData::run() {
        println!("{:?}", hiragana_groups);
        println!("{:?}", katakana_groups);
        println!("{:?}", kanji_groups);
    } 
    
    else {
        eprintln!("Error: Unable to read data.");
        std::process::exit(1);
    }
}