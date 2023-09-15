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
        loop {
            println!("\n= = = = = = = = Welcome to LeJa = = = = = = = =");

            println!("\n1. Hiragana");
            println!("2. Katakana");
            println!("3. Kanji");
            println!("4. Exit.");

            println!("\nEnter the option number of what you want to practice:");
            let mut option = String::new();
            io::stdin().read_line(&mut option).unwrap();

            let option: u32 = match option.trim().parse() {
                Ok(option) => option,
                Err(_) => {
                    println!("\nInvalid option. Try again");
                    continue;
                }
            };

            match option {
                1 => {
                    let practice_option = select_practice_option("Hiragana");

                    if practice_option == 4 {
                        break;
                    }

                    practice_hiragana(&hiragana_groups, practice_option);
                }

                2 => {
                    let practice_option = select_practice_option("Katakana");
                    
                    if practice_option == 4 {
                        break;
                    }

                    practice_katakana(&katakana_groups, practice_option);
                }

                3 => {
                    let practice_option = select_practice_option("Kanji");
                    
                    if practice_option == 4 {
                        break;
                    }

                    practice_kanji(&kanji_groups, practice_option);
                }

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
        std::process::exit(1);
    }
}

fn select_practice_option(group_name: &str) -> u32 {
    loop {
        println!("\n= = = = = = = = Practice {} = = = = = = = =", group_name);

        println!("\n1. Latin word");
        println!("2. Symbol word");
        println!("3. Previous menu");
        println!("4. Exit.");

        println!("\nEnter the option number of how you want the word to be given to you:");
        let mut option = String::new();
        io::stdin().read_line(&mut option).unwrap();

        let option: u32 = match option.trim().parse() {
            Ok(option) => option,
            Err(_) => {
                println!("\nInvalid option. Try again");
                continue;
            }
        };

        if option >= 1 && option <= 4 {
            return option;
        } 
        
        else {
            println!("\nInvalid option. Try again");
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
        let mut option = String::new();
        io::stdin().read_line(&mut option).unwrap();

        let option: usize = match option.trim().parse() {
            Ok(option) => option,
            Err(_) => {
                println!("\nInvalid option. Try again");
                continue;
            }
        };

        if option >= 1 && option <= group_names.len() {
            return group_names[option - 1];
        } 
        
        else {
            println!("\nInvalid option. Try again");
        }
    }
}

fn select_random_word<'a>(groups: &'a Vec<JapaneseGroup>, group_name: &'a str) -> Option<&'a Word> {
    let group = groups.iter().find(|group| group.group == group_name)?;

    let words = &group.words;
    if words.is_empty() {
        return None;
    }

    let random_word = words.choose(&mut rand::thread_rng()).unwrap();
    Some(random_word)
}

fn practice_hiragana(hiragana_groups: &Vec<JapaneseGroup>, practice_option: u32) {
    let group_names: Vec<&str> = hiragana_groups.iter().map(|group| group.group.as_str()).collect();

    match practice_option {
        1 | 2 => {
            let selected_group = select_group(&group_names, "Hiragana");

            loop {
                println!("\n= = = = = = = = Hiragana Word Option = = = = = = = =");

                println!("\n1. Get word");
                println!("2. Select Group");
                println!("3. Hiragana Menu");

                println!("\nEnter the option number of what you want:");
                let mut option = String::new();
                io::stdin().read_line(&mut option).unwrap();

                let option: u32 = match option.trim().parse() {
                    Ok(option) => option,
                    Err(_) => {
                        println!("\nInvalid option");
                        continue;
                    }
                };

                match option {
                    1 => {
                        if let Some(word) = select_random_word(hiragana_groups, selected_group) {
                            if practice_option == 1 {
                                println!("\nWhat's the symbol of '{}' word", word.latin);
                            } 
                                
                            else {
                                println!("\nWhat's the Latin of '{}' symbol", word.symbol);
                            }
            
                            println!("\nPress 'Enter' when you're done:");
                            let mut input = String::new();
                            io::stdin().read_line(&mut input).unwrap();
            
                            if practice_option == 1 {
                                println!("Symbol: {}", word.symbol);
                                println!("Meaning: {}", word.meaning);
                            } 
                            
                            else {
                                println!("Latin: {}", word.latin);
                                println!("Meaning: {}", word.meaning);
                            }
                        }
                            
                        else {
                            println!("\nNo words in the selected group.");
                        }
                    }

                    2 => {
                        select_group(&group_names, "Hiragana");
                    }

                    3 => {
                        let practice_option = select_practice_option("Hiragana");
                        practice_hiragana(&hiragana_groups, practice_option);
                    }

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

        4 => {}
        _ => println!("\nInvalid option. Try again"),
    }
}

fn practice_katakana(katakana_groups: &Vec<JapaneseGroup>, practice_option: u32) {
    println!("\nKatakana");
}

fn practice_kanji(kanji_groups: &Vec<JapaneseGroup>, practice_option: u32) {
    println!("\nKanji");
}