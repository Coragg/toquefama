use std::io::{self, Write};

pub fn input_string(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut string_variable: String = String::new();
    io::stdin().read_line(&mut string_variable).expect("Failed to read line");
    let new_variable: String = string_variable.trim().to_string();
    new_variable
}


pub fn input_i8(message: &str) -> i8 {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut variable: String = String::new();
    io::stdin().read_line(&mut variable).expect("Something is wrong");
    match variable.trim().parse::<i8>() {
        Ok(num) => num,
        Err(_) => {
            println!("Please, enter a valid term.");
            input_i8(message)
        },
    }
}