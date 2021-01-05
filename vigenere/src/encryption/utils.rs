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

pub fn get_letter_of_string(string: &String, index: usize) -> char {
    let mut i = 0;
    for letter in string.chars() {
        if i == index {
            return letter;
        }

        i += 1;
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
