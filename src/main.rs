use std::io::{self, Read};
use serde::{Deserialize, Serialize};
use std::fs::File;
use rand::seq::SliceRandom;

#[derive(Debug, Deserialize, Serialize)]
struct Word {
    latin: String,
    symbol: String,
    meaning: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct HiraganaGroup {
    vowels: Option<Vec<Word>>,
    k: Option<Vec<Word>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct HiraganaData {
    hiragana: Vec<HiraganaGroup>,
}

fn main() {
    loop {
        println!("\n= = = = = = = = Welcome to LeJa = = = = = = = =");

        println!("\n1. Hiragana");
        println!("2. Katakana");
        println!("3. Kanji");
        println!("4. Exit");

        println!("\nEnter the option number of what you want to learn:");
        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Error Reading");

        let option: u32 = match option.trim().parse() {
            Ok(option) => option,
            Err(_) => {
                println!("\nInvalid option.");
                continue;
            }
        };

        match option {
            1 => learn_hiragana(),
            2 => learn_katakana(),
            3 => learn_kanji(),
            4 => break,

            _ => {
                println!("\nInvalid option.");
                continue;
            }
        }
    }
}

fn learn_hiragana() {
    loop {
        println!("\n= = = = = = = = Hiragana = = = = = = = =");

        println!("\n1. Gojūon");
        println!("2. Dakuon");
        println!("3. Yōon");
        println!("4. Main Menu");

        println!("\nEnter the option number of what you want to learn:");
        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Error Reading");

        let option: u32 = match option.trim().parse() {
            Ok(option) => option,
            Err(_) => {
                println!("\nInvalid option.");
                continue;
            }
        };

        match option {
            1 => h_gojūon(),
            2 => h_dakuon(),
            3 => h_yōon(),
            4 => break,

            _ => {
                println!("\nInvalid option.");
                continue;
            }
        }
    }
}

fn h_gojūon() {
    let file_path = "data/words.json";

    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => {
            println!("Error opening the JSON file");
            return;
        }
    };

    let mut json_data = String::new();
    if file.read_to_string(&mut json_data).is_err() {
        println!("Error reading the JSON file");
        return;
    }


    let hiragana_data: HiraganaData = match serde_json::from_str(&json_data) {
        Ok(data) => data,
        Err(_) => {
            println!("Error parsing JSON data");
            return;
        }
    };

    loop {
        println!("\n= = = = = = = = Hiragana Gojūon = = = = = = = =");

        println!("\n1. Vowels");
        println!("2. Previous and k + (vowels)");
        println!("3. Previous and s + (vowels)");
        println!("4. Previous and t + (vowels)");
        println!("5. Previous and n + (vowels)");
        println!("6. Previous and h + (vowels)");
        println!("7. Previous and m + (vowels)");
        println!("8. Previous and y + (vowels)");
        println!("9. Previous and r + (vowels)");
        println!("10. Previous and w + (vowels)");
        println!("11. Hiragana Menu");


        println!("\nEnter the option number of what you want to learn:");
        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Error Reading");

        let option: u32 = match option.trim().parse() {
            Ok(option) => option,
            Err(_) => {
                println!("\nInvalid option.");
                continue;
            }
        };

        match option {
            1 => {
                if let Some(ref words) = hiragana_data.hiragana[0].vowels.as_ref() {
                    if !words.is_empty() {
                        if let Some(word) = words.choose(&mut rand::thread_rng()) {
                            println!("\nRandom latin: {}", word.latin);
                
                            println!("\nType 'finished' when you're ready to see the result:");
                            let mut input: String = String::new();
                            io::stdin().read_line(&mut input).expect("Error Reading");
                
                            if input.trim() != "finished" {
                                println!("\nInvalid input. Try again");
                                continue;
                            }
                
                            println!("\nSymbol: {}", word.symbol);
                            println!("Meaning: {}", word.meaning);
                        } 
                        
                        else {
                            println!("\nNo words available in the 'vowels' vector");
                        }
                    } 
                    
                    else {
                        println!("\n'vowels' vector is empty");
                    }
                } 
                
                else {
                    println!("\nNo 'vowels' available");
                }
            }

            2 => {
                if let Some(ref words) = hiragana_data.hiragana[1].k.as_ref() {
                    if !words.is_empty() {
                        if let Some(word) = words.choose(&mut rand::thread_rng()) {
                            println!("\nRandom latin: {}", word.latin);
                            
                            println!("\nType 'finished' when you're ready to see the result:");
                            let mut input: String = String::new();
                            io::stdin().read_line(&mut input).expect("Error Reading");
                            
                            if input.trim() != "finished" {
                                println!("\nInvalid input. Try again");
                                continue;
                            }
                            
                            println!("\nSymbol: {}", word.symbol);
                            println!("Meaning: {}", word.meaning);
                        } else {
                            println!("\nNo words available in the 'k' vector");
                        }
                    } else {
                        println!("\n'k' vector is empty");
                    }
                } else {
                    println!("\nNo 'k' available");
                }
            }            

            3 => {}
            4 => {}
            5 => {}
            6 => {}
            7 => {}
            8 => {}
            9 => {}
            10 => {}
            11 => break,

            _ => {
                println!("\nInvalid option.");
                continue;
            }
        }
    }
}

fn h_dakuon() {
    loop {
        println!("\n= = = = = = = = Hiragana Dakuon = = = = = = = =");
    
        println!("\n1. Gojūon and g + (vowels)");
        println!("2. Gojūon and previous and z + (vowels)");
        println!("3. Gojūon and previous and d + (vowels)");
        println!("4. Gojūon and previous and b + (vowels)");
        println!("5. Gojūon and previous and p + (vowels)");
        println!("6. Hiragana Menu");


        println!("\nEnter the option number of what you want to learn:");
        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Error Reading");

        let option: u32 = match option.trim().parse() {
            Ok(option) => option,
            Err(_) => {
                println!("\nInvalid option.");
                continue;
            }
        };

        match option {
            1 => {}
            2 => {}
            3 => {}
            4 => {}
            5 => {}
            6 => break,

            _ => {
                println!("\nInvalid option.");
                continue;
            }
        }
    }
}

fn h_yōon() {
    loop {
        println!("\n= = = = = = = = Hiragana Yōon = = = = = = = =");

        println!("\n1. Gojūon and Dakuon and ky + (vowels)");
        println!("2. Gojūon and Dakuon and previous and gy + (vowels)");
        println!("3. Gojūon and Dakuon and previous and sh + (vowels)");
        println!("4. Gojūon and Dakuon and previous and j + (vowels)");
        println!("5. Gojūon and Dakuon and previous and ch + (vowels)");
        println!("6. Gojūon and Dakuon and previous and ny + (vowels)");
        println!("7. Gojūon and Dakuon and previous and hy + (vowels)");
        println!("8. Gojūon and Dakuon and previous and by + (vowels)");
        println!("9. Gojūon and Dakuon and previous and py + (vowels)");
        println!("10. Gojūon and Dakuon and previous and my + (vowels)");
        println!("11. Gojūon and Dakuon and previous and ry + (vowels)");
        println!("12. Hiragana Menu");


        println!("\nEnter the option number of what you want to learn:");
        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Error Reading");

        let option: u32 = match option.trim().parse() {
            Ok(option) => option,
            Err(_) => {
                println!("\nInvalid option.");
                continue;
            }
        };

        match option {
            1 => {}
            2 => {}
            3 => {}
            4 => {}
            5 => {}
            6 => {}
            7 => {}
            8 => {}
            9 => {}
            10 => {}
            11 => {}
            12 => break,

            _ => {
                println!("\nInvalid option.");
                continue;
            }
        }
    }
}

fn learn_katakana() {
    println!("\n= = = = = = = = Katakana = = = = = = = =");
}

fn learn_kanji() {
    println!("\n= = = = = = = = Kanji = = = = = = = =");
}