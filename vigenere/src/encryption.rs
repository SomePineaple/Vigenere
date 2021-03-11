use crate::utils;

pub fn encrypt (message: String, message_key: &String, alphabets: &Vec<String>) -> String {
    let mut encrypted_message = String::new();

    let mut i = 0;

    for letter in message.chars() {
        let key_letter = utils::get_letter_of_string(message_key, i % message_key.len() as i32);
        encrypted_message.push(get_encrypted_letter(letter, key_letter, &alphabets));
        i += 1;
    }

    return encrypted_message;
}

fn get_encrypted_letter(original: char, key_letter: char, alphabets: &Vec<String>) -> char {
    let key_letter_of_alphabet: usize = utils::get_letter_of_alphabet(key_letter, &alphabets[0]);
    let alphabet = &alphabets[key_letter_of_alphabet];

    let encrypted_letter = utils::get_letter_of_str(&alphabet, utils::get_letter_of_alphabet(original, &alphabets[0]));

    return encrypted_letter;
}
