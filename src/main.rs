use std::io;
use rand::Rng;

fn input_size(message:&str) -> usize {
    loop {
        let mut size = String::new();
        println!("{message}");
        io::stdin().read_line(&mut size).expect("Error of reading...");
        match size.trim().parse::<usize>() {
            Ok(n) => return n,
            Err(_) => println!("Error : Invalid size, please try again.")
        }
    }
}

fn ask_yes_or_no(message:&str) -> bool {
    loop {
        let mut choice = String::new();
        println!("{message}");
        io::stdin().read_line(&mut choice).expect("Error of reading...");
        let choice = choice.trim().to_lowercase();
        match choice.as_str() {
            "y" => return true,
            "n" => return false,
            _ => println!("Error : Invalid")
        }
    }
}

fn generate_password(length:usize, use_capital:bool, use_digits:bool, use_symbols:bool) -> String {
    let mut rng = rand::rng();

    let lowercase = "abcdefghijklmnopqrstuvwxyz";
    let capital = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let digits = "0123456789";
    let symbols = "!@#$%^&*()_-+={}[]|:;'<>,.?/";

    let mut charset = String::from(lowercase);

    if use_capital {
        charset.push_str(capital)
    }
    if use_digits {
        charset.push_str(digits)
    }
    if use_symbols {
        charset.push_str(symbols)
    }
    if charset.is_empty() {
        panic!("Error: No character set selected for password generation.");
    }
    (0..length)
        .map(|_| {
            let idx = rng.random_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect()
}

fn generate_and_display_passwords(length: usize, number_of_passwords: usize, use_capital: bool, use_digits: bool, use_symbols: bool) {
    println!("\n🔑 Generated Passwords:");
    for _ in 0..number_of_passwords {
        let password = generate_password(length, use_capital, use_digits, use_symbols);
        println!("{}", password);
    }
    println!()
}

fn display_status(capital:bool, digits:bool, symbols:bool, length:usize, number_passwords:usize) {
    println!("Generator Status :");

    if capital == true {
        println!("✅ Capital (A-Z)")
    } else {
        println!("❌ Capital (A-Z)")
    }

    if digits == true {
        println!("✅ Digits (0-9)")
    } else {
        println!("❌ Digits (0-9)")
    }

    if symbols == true {
        println!("✅ Symbols (@!&$?*)")
    } else {
        println!("❌ Symbols (@!&$?*)")
    }
    println!("Length of passwords : {length}");
    println!("Number of passwords generated : {number_passwords}");
    println!()
}

fn menu() {
    let mut length: usize = 30;
    let mut number_of_passwords: usize = 10;
    let mut use_capital: bool = true;
    let mut use_digits: bool = true;
    let mut use_symbols: bool = true;

    println!("Welcome to my generator of password! - Made by Claipousse\n");
    display_status(use_capital, use_digits, use_symbols, length, number_of_passwords);
    loop {
        let mut choice = String::new();
        println!("1 - Generate password(s)");
        println!("2 - Generator Status");
        println!("3 - Change characters used");
        println!("4 - Change password(s) length");
        println!("5 - Change number of passwords generated");
        println!("6 - Exit");
        io::stdin().read_line(&mut choice).expect("Error when reading...");
        let choice = choice.trim();
        println!();
        match choice {
            "1" => generate_and_display_passwords(length, number_of_passwords, use_capital, use_digits, use_symbols),
            "2" => display_status(use_capital, use_digits, use_symbols, length, number_of_passwords),
            "3" => {
                use_capital = ask_yes_or_no("Do you want capital letters (A-Z) in your password? (y/n)");
                use_digits = ask_yes_or_no("Do you want digits (0-9) in your password? (y/n)");
                use_symbols = ask_yes_or_no("Do you want symbols (@!&$?*) in your password? (y/n)");
            }
            "4" => length = input_size("Enter new password length:"),
            "5" => number_of_passwords = input_size("Enter number of passwords to generate:"),
            "6" => {
                println!("Goodbye! 👋");
                break;
            }
            _ => println!("Error: Invalid choice, please enter a number between 1 and 6."),
        }
    }
}

fn main() {
    menu()
}