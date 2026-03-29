use std::io::prelude::*;
use std::fs::File;
use std::io;

enum UserChoice {
    Read,
    Write,
}

fn get_user_choice() -> UserChoice {
    loop {
        let mut input = String::new();

        println!("1) Read a file");
        println!("2) Write a file");
        print!("Enter choice: ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => return UserChoice::Read,
            "2" => return UserChoice::Write,
            _ => println!("Invalid input, try again.\n"),
        }
    }
}

fn get_filename() -> String{
    let mut input = String::new();

    print!("Enter filename: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}

fn read_file(filename: &String) -> String{
    let mut file = File::open(filename).expect("File not found.");

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Cannot read file.");

    contents
}

fn write_file(filename: &String, contents: &String){
    let mut file = File::create(filename).expect("Could not create file.");

    file.write_all(contents.as_bytes()).expect("Cannot write to file.");
}

fn get_plaintext() -> String {
    let mut input = String::new();

    print!("Enter plaintext: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}

fn get_shift() -> u8 {
    let mut input = String::new();

    print!("Enter shift value: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();

    input.trim().parse().expect("Please enter a valid number")
}

fn display_texts(plaintext: &str, ciphertext: &str) {
    println!("Plaintext:");
    println!("{}", plaintext);
    println!("Ciphertext:");
    println!("{}", ciphertext);
}

fn encrypt(plaintext: &str, shift: u8) -> String{
    let mut result = String::new();

    for c in plaintext.chars() {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let shifted = ((c as u8 - base + shift) % 26) + base;
            result.push(shifted as char);
        } else {
            result.push(c);
        }
    }

    result
}

fn decrypt(ciphertext: &str, shift: u8) -> String{
    let mut result = String::new();

    for c in ciphertext.chars() {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let shifted = ((c as u8 - base + 26 - shift) % 26) + base; // +26 to keep from going negative before the modulo
            result.push(shifted as char);
        } else {
            result.push(c);
        }
    }

    result
}

fn main() {
    let choice = get_user_choice();

    match choice {
        UserChoice::Read => {
            let filename = get_filename();
            let ciphertext = read_file(&filename);
            let shift = get_shift();
            let plaintext = decrypt(&ciphertext, shift);
            display_texts(&plaintext, &ciphertext);
        }
        UserChoice::Write => {
            let filename = get_filename();
            let plaintext = get_plaintext();
            let shift = get_shift();
            let ciphertext = encrypt(&plaintext, shift);
            display_texts(&plaintext, &ciphertext);
            write_file(&filename, &ciphertext);
        }
    }
}
