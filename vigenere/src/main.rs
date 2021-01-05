mod utils;
mod encryption;

fn main() {
    let alphabets: [&str; 78] = [
             "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()'\" ,.:",
             "BCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()'\" ,.:A",
             "CDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()'\" ,.:AB",
             "DEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()'\" ,.:ABC",
             "EFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()'\" ,.:ABCD",
             "FGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()'\" ,.:ABCDE",
             "GHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()'\" ,.:ABCDEF",
             "HIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()'\" ,.:ABCDEFG",
             "IJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()'\" ,.:ABCDEFGH",
             "JKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()'\" ,.:ABCDEFGHI",
             "KLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()'\" ,.:ABCDEFGHIJ",
             "LMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()'\" ,.:ABCDEFGHIJK",
             "MNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()'\" ,.:ABCDEFGHIJKL",
             "NOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()'\" ,.:ABCDEFGHIJKLM",
             "OPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()'\" ,.:ABCDEFGHIJKLMN",
             "PQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNO",
             "QRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOP",
             "RSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQ",
             "STUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQR",
             "TUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRS",
             "UVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRST",
             "VWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTU",
             "WXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUV",
             "XYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVW",
             "YZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWX",
             "Zabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXY",
             "abcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZ",
             "bcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZa",
             "cdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZab",
             "defghijklmnopqrstuvwxyz1234567890!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabc",
             "efghijklmnopqrstuvwxyz1234567890!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcd",
             "fghijklmnopqrstuvwxyz1234567890!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcde",
             "ghijklmnopqrstuvwxyz1234567890!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef",
             "hijklmnopqrstuvwxyz1234567890!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefg",
             "ijklmnopqrstuvwxyz1234567890!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefgh",
             "jklmnopqrstuvwxyz1234567890!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghi",
             "klmnopqrstuvwxyz1234567890!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghij",
             "lmnopqrstuvwxyz1234567890!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijk",
             "mnopqrstuvwxyz1234567890!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijkl",
             "nopqrstuvwxyz1234567890!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklm",
             "opqrstuvwxyz1234567890!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmn",
             "pqrstuvwxyz1234567890!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmno",
             "qrstuvwxyz1234567890!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnop",
             "rstuvwxyz1234567890!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopq",
             "stuvwxyz1234567890!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqr",
             "tuvwxyz1234567890!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrs",
             "uvwxyz1234567890!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrst",
             "vwxyz1234567890!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstu",
             "wxyz1234567890!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuv",
             "xyz1234567890!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvw",
             "yz1234567890!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwx",
             "z1234567890!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxy",
             "1234567890!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz",
             "234567890!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1",
             "34567890!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz12",
             "4567890!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz123",
             "567890!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234",
             "67890!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz12345",
             "7890!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz123456",
             "890!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567",
             "90!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz12345678",
             "0!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz123456789",
             "!@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890",
             "@#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!",
             "#$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@",
             "$%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#",
             "%^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$",
             "^&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%",
             "&*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^",
             "*()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&",
             "()'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*",
             "'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*(",
             "'\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()",
             "\" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()'",
             " ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()'\"",
             ",.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()'\" ",
             ".:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()'\" ,",
             ":ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()'\" ,."
    ];

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
        println!("Encrypted text:\n{}", encryption::encrypt(text, &key, alphabets));
    } else {
        println!("Decrypted text:\n{}", decrypt(text, &key, alphabets));
    }
}

fn decrypt (encrypted_message: String, message_key: &String, alphabets: [&str; 78]) -> String {
    let mut decrypted_message = String::new();

    let mut i = 0;

    for letter in encrypted_message.chars() {
        let key_letter = get_letter_of_other_string(message_key, i % message_key.len());
        decrypted_message.push(get_decrypted_letter(letter, key_letter, alphabets));
        i += 1;
    }

    return decrypted_message;
}

fn get_decrypted_letter(encrypted: char, key_letter: char, alphabets: [&str; 78]) -> char {
    let key_letter_of_alphabet: usize = get_letter_of_alphabet(key_letter, alphabets[0]);
    let alphabet = alphabets[key_letter_of_alphabet];

    let decrypted_letter = get_letter_of_string(alphabets[0], get_letter_of_alphabet(encrypted, alphabet));

    return decrypted_letter;
}

fn get_letter_of_string(string: &str, index: usize) -> char {
    let mut i = 0;
    for letter in string.chars() {
        if i == index {
            return letter;
        }

        i += 1;
    }

    return '*';
}

fn get_letter_of_other_string(string: &String, index: usize) -> char {
    let mut i = 0;
    for letter in string.chars() {
        if i == index {
            return letter;
        }

        i += 1;
    }

    return '*';
}

fn get_letter_of_alphabet(letter: char, alphabet: &str) -> usize {
    let mut i = 0;

    for x in alphabet.chars() {
        if x.eq(&letter) {
            return i;
        }

        i += 1;
    }

    return 69;
}