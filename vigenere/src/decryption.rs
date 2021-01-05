mod utils;

pub fn decrypt (encrypted_message: String, message_key: &String, alphabets: [&str; 78]) -> String {
    let mut decrypted_message = String::new();

    let mut i = 0;

    for letter in encrypted_message.chars() {
        let key_letter = utils::get_letter_of_string(message_key, i % message_key.len());
        decrypted_message.push(get_decrypted_letter(letter, key_letter, alphabets));
        i += 1;
    }

    return decrypted_message;
}

fn get_decrypted_letter(encrypted: char, key_letter: char, alphabets: [&str; 78]) -> char {
    let key_letter_of_alphabet: usize = get_letter_of_alphabet(key_letter, alphabets[0]);
    let alphabet = alphabets[key_letter_of_alphabet];

    let decrypted_letter = utils::get_letter_of_str(alphabets[0], get_letter_of_alphabet(encrypted, alphabet));

    return decrypted_letter;
}
