mod utils;
mod encryption;
mod decryption;

fn main() {
    let alphabets = gen_alphabets("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()'\" ,.:");

    // Get the mode
    let mode = utils::input("Do you want to encrypt or decrypt? ");

    let encrypting = mode.eq_ignore_ascii_case("encrypt");

    // Get the text to encrypt or decrypt
    let text;
    if encrypting {
        text = utils::input("Enter some text to encrypt: ");
    } else {
        text = utils::input("Enter some text to decrypt: ");
    }

    // Get the key to either encrypt or decrypt with
    let key = utils::input("Enter a key for the message: ");

    println!("{}ing", mode);

    if encrypting {
        println!("Encrypted text:\n{}", encryption::encrypt(text, &key, &alphabets));
    } else {
        println!("Decrypted text:\n{}", decryption::decrypt(text, &key, &alphabets));
    }
}

fn gen_alphabets(original: &str) -> Vec<String> {
    let mut alphabets = Vec::new();

    for i in 0..original.chars().count() {
        alphabets.push(shift_string(original.to_string(), i as i32));
    }

    alphabets
}

fn shift_string(original: String, shift_amount: i32) -> String {
    let mut shifted = String::new();

    for i in 0..original.chars().count() {
        shifted += &String::from(utils::get_letter_of_string(&original, i as i32 - shift_amount));
    }

    return shifted;
}
