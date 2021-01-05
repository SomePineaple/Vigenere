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
