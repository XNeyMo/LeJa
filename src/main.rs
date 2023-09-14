use std::io;

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