use rand::Rng;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn get_random_word() -> String {
    let mut file =
        File::open("/home/ramsha/Desktop/RUST/hangman/src/words.txt").expect("Unable to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Failed to read file into string");

    let mut words = vec![];
    let bytes = contents.as_bytes();
    let mut start = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b'\n' {
            words.push(&contents[start..i]);
            start = i + 1;
        }
    }

    let random_word = rand::thread_rng().gen_range(1..17);

    words[random_word].to_string()
}

fn mask(word: &String, mask: &String) -> String {
    let mut final_masked_word = String::new();
    for letter in word.chars() {
        if mask.contains(letter) {
            final_masked_word.push(letter)
        } else {
            final_masked_word.push('-');
        }
    }
    final_masked_word
}

fn main() {
    let word = String::from(get_random_word());
    let mut mask_string = String::new();
    let mut mistakes = 6;
    mask_string.push(' ');
    let mut masked_word = mask(&word, &mask_string);

    println!("\nYou have {} chance(s) ", mistakes);

    println!("\n\nWord to guess : {}\n", masked_word);
    while mistakes != 0 {
        let mut guess = String::new();
        print!("\nEnter your guess: ");
        std::io::stdout()
            .flush()
            .ok()
            .expect("Could not flush stdout");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: char = match guess.trim().parse() {
            Ok(guess) => guess,
            Err(_) => continue,
        };
        if mask_string.contains(guess) {
            println!("\nLetter already guessed! ");
            continue
        }
        mask_string.push(guess);

        if word.contains(guess) {
            masked_word = mask(&word, &mask_string);
            println!("Your Guess: {}", masked_word);
            if masked_word == word {
                println!("You Win!!");
                break
            }
        } else {
            println!("\nIncorrect guess");
            mistakes -= 1;
            match mistakes {
                5 => {
                    println!(" ________    ");
                    println!("|        |   ");
                    println!("|      (`_`) ");
                    println!("|            ");
                    println!("|            ");
                    println!("|            ");
                    println!("|            ");
                    println!("|            ");
                    println!("_            ");
                    println!("You have {} chance(s) remaining ", mistakes);
                }
                4 => {
                    println!(" ________    ");
                    println!("|        |   ");
                    println!("|      (`_`) ");
                    println!("|        |   ");
                    println!("|        |   ");
                    println!("|        |   ");
                    println!("|        |   ");
                    println!("|            ");
                    println!("_            ");
                    println!("You have {} chance(s) remaining ", mistakes);
                }
                3 => {
                    println!(" ________    ");
                    println!("|        |   ");
                    println!("|      (`_`) ");
                    println!("|        |   ");
                    println!("|      / |   ");
                    println!("|     /  |   ");
                    println!("|        |   ");
                    println!("|            ");
                    println!("_            ");
                    println!("You have {} chance(s) remaining ", mistakes);
                }
                2 => {
                    println!(" ________     ");
                    println!("|        |    ");
                    println!("|      (`_`)  ");
                    println!("|        |    ");
                    println!("|      / | \\  ");
                    println!("|     /  |  \\ ");
                    println!("|        |    ");
                    println!("|             ");
                    println!("_             ");
                    println!("You have {} chance(s) remaining ", mistakes);
                }
                1 => {
                    println!(" ________     ");
                    println!("|        |    ");
                    println!("|      (`_`)  ");
                    println!("|        |    ");
                    println!("|      / | \\  ");
                    println!("|     /  |  \\ ");
                    println!("|        |    ");
                    println!("|      /      ");
                    println!("_     /       ");
                    println!("You have {} chance(s) remaining ", mistakes);
                }
                0 => {
                    println!(" ________     ");
                    println!("|        |    ");
                    println!("|      (`_`)  ");
                    println!("|        |    ");
                    println!("|      / | \\  ");
                    println!("|     /  |  \\ ");
                    println!("|        |    ");
                    println!("|      /  \\  ");
                    println!("_     /    \\ ");

                    println!("Game Over")
                }
                _ => {
                    println!("Game Over")
                }
            }
        }
    }
}
