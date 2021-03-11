use std::io::Write;
use std::io::stdin;
use std::io::stdout;

pub fn input (prompt: &str) -> String {
    let mut input_str = String::new();

    print!("{}", prompt);

    let _ = stdout().flush();

    stdin().read_line(&mut input_str).expect("Did not enter a correct string");

    if let Some('\n') = input_str.chars().next_back() {
        input_str.pop();
    }
    if let Some('\r') = input_str.chars().next_back() {
        input_str.pop();
    }

    return input_str;
}

pub fn get_letter_of_alphabet(letter: char, alphabet: &str) -> usize {
    let mut i = 0;

    for x in alphabet.chars() {
        if x.eq(&letter) {
            return i;
        }

        i += 1;
    }

    return 69;
}

pub fn get_letter_of_string(string: &String, index: i32) -> char {
    let mut i = 0;
    if index >= 0 {
        for letter in string.chars() {
            if i == index {
                return letter;
            }
            i += 1;
        }
    } else {
        for letter in string.chars() {
            if i == index + string.chars().count() as i32 {
                return letter;
            }
            i += 1;
        }
    }

    return '*';
}

pub fn get_letter_of_str(string: &str, index: usize) -> char {
    let mut i = 0;
    for letter in string.chars() {
        if i == index {
            return letter;
        }

        i += 1;
    }

    return '*';
}
