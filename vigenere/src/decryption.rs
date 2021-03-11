use crate::utils;

pub fn decrypt (encrypted_message: String, message_key: &String, alphabets: &Vec<String>) -> String {
    let mut decrypted_message = String::new();

    let mut i = 0;

    // Go through all the letters, and get the decrypted one that corresponds
    for letter in encrypted_message.chars() {
        // Get the letter of the key that was used to encrypt this
        let key_letter = utils::get_letter_of_string(message_key, i % message_key.len() as i32);
        // Decrypt the letter using the letter of the key that we found before, and add that to the final decrypted string
        decrypted_message.push(get_decrypted_letter(letter, key_letter, alphabets));
        i += 1;
    }

    return decrypted_message;
}

fn get_decrypted_letter(encrypted: char, key_letter: char, alphabets: &Vec<String>) -> char {
    let key_letter_of_alphabet: usize = utils::get_letter_of_alphabet(key_letter, &alphabets[0]);
    let alphabet = &alphabets[key_letter_of_alphabet];

    let decrypted_letter = utils::get_letter_of_str(&alphabets[0], utils::get_letter_of_alphabet(encrypted, &alphabet));

    return decrypted_letter;
}
