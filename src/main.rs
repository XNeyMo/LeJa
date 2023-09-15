use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Read};
use rand::seq::SliceRandom;
use std::process;

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
    fn run() -> Option<Self> {
        let mut file = File::open("data/words.json").ok()?;

        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_err() {
            return None;
        }

        serde_json::from_str(&contents).ok()
    }
}

fn main() {
    if let Some(data) = LanguageData::run() {
        let (hiragana_groups, katakana_groups, kanji_groups) = (data.hiragana, data.katakana, data.kanji);
        
        loop {
            println!("\n= = = = = = = = Welcome to LeJa = = = = = = = =");

            println!("\n1. Hiragana");
            println!("2. Katakana");
            println!("3. Kanji");
            println!("4. Exit.");

            println!("\nEnter the option number of what you want to practice:");
            let option: usize = match read_user_input() {
                Ok(option) => option,
                Err(_) => {
                    println!("\nInvalid option. Try again");
                    continue;
                }
            };

            match option {
                1 => practice(&hiragana_groups, "Hiragana"),
                2 => practice(&katakana_groups, "Katakana"),
                3 => practice(&kanji_groups, "Kanji"),
                4 => process::exit(0),

                _ => {
                    println!("\nInvalid option. Try again");
                    continue;
                }
            }
        }
    } 
    
    else {
        eprintln!("Error: Unable to read data.");
        process::exit(1);
    }
}

fn practice(groups: &[JapaneseGroup], group_name: &str) {
    let group_names: Vec<&str> = groups.iter().map(|group| group.group.as_str()).collect();

    loop {
        println!("\n= = = = = = = = Practice {} = = = = = = = =", group_name);

        println!("\n1. Latin word");
        println!("2. Symbol word");
        println!("3. Previous menu");
        println!("4. Exit.");

        println!("\nEnter the option number of how you want the word to be given to you:");
        let option_word: usize = match read_user_input() {
            Ok(option_word) => option_word,
            Err(_) => {
                println!("\nInvalid option. Try again");
                continue;
            }
        };

        match option_word {
            1 | 2 => {
                let selected_group = select_group(&group_names, group_name);

                loop {
                    println!("\n= = = = = = = = {} Word Option = = = = = = = = =", group_name);

                    println!("\n1. Get word");
                    println!("2. Select Group");
                    println!("3. {} Menu", group_name);
                    println!("4. Exit.");

                    println!("\nEnter the option number of what you want:");
                    let option: usize = match read_user_input() {
                        Ok(option) => option,
                        Err(_) => {
                            println!("\nInvalid option");
                            continue;
                        }
                    };

                    match option {
                        1 => {
                            if let Some(word) = select_random_word(groups, selected_group) {
                                if option_word == 1 {
                                    println!("\nWhat's the symbol of '{}' word", word.latin);
                                } else {
                                    println!("\nWhat's the Latin of '{}' symbol", word.symbol);
                                }

                                if option_word == 1 {
                                    println!("\nPress 'Enter' when you're done:");
                                } else {
                                    println!("\nEnter the word Latin:");
                                }

                                let mut input = String::new();
                                io::stdin().read_line(&mut input).unwrap();
                                let input = input.trim().to_string();

                                if option_word == 1 {
                                    println!("\nSymbol: {}", word.symbol);
                                    println!("Meaning: {}", word.meaning);
                                } else {
                                    if input == word.latin {
                                        println!("\nCorrect. The meaning: {}", word.meaning);
                                    } else {
                                        println!("\nIncorrect answer. The correct is: {}", word.latin);
                                        println!("Meaning: {}", word.meaning);
                                    }
                                }
                            } else {
                                println!("\nNo words in the selected group.");
                            }
                        }

                        2 => {
                            select_group(&group_names, group_name);
                        }

                        3 => {
                            practice(groups, group_name);
                        }

                        4 => process::exit(0),
                        _ => {
                            println!("\nInvalid option");
                            continue;
                        }
                    }
                }
            }

            3 => {
                main();
            }

            4 => process::exit(0),
            _ => println!("\nInvalid option. Try again"),
        }
    }
}

fn select_group<'a>(group_names: &'a Vec<&'a str>, name: &'a str) -> &'a str {
    loop {
        println!("\n= = = = = = = = {} Groups = = = = = = = =\n", name);

        for (index, group) in group_names.iter().enumerate() {
            println!("{}. {}", index + 1, group);
        }

        println!("\nEnter the number of the group you want to practice:");
        let option: usize = match read_user_input() {
            Ok(option) => option,
            Err(_) => {
                println!("\nInvalid option");
                continue;
            }
        };

        if option >= 1 && option <= group_names.len() {
            return group_names[option - 1];
        } else {
            println!("\nInvalid option. Try again");
        }
    }
}

fn select_random_word<'a>(groups: &'a [JapaneseGroup], group_name: &'a str) -> Option<&'a Word> {
    let group = groups.iter().find(|group| group.group == group_name)?;

    let words = &group.words;
    if words.is_empty() {
        return None;
    }

    let random_word = words.choose(&mut rand::thread_rng()).unwrap();
    Some(random_word)
}

fn read_user_input() -> Result<usize, std::num::ParseIntError> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse()
}