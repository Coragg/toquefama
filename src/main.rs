use std::io::{self, Write};


fn info_game() -> () {
    println!("The objective of the game is to randomly generate 3 to 8 numbers");
    println!("from zero to nine, your duty is to find them with the following clues");
    println!("if you have 1 touch you found 1 number but not in the right place");
    println!("if you found 1 fame you found 1 number and in the right place");
    println!("if you achieve all the fames, congratulations you won");
    println!("Note: if you enter a number with more digits than indicated or repeated numbers, you lose your game")
}

fn input_string(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut string_variable: String = String::new();
    io::stdin().read_line(&mut string_variable).expect("Failed to read line");
    let new_variable: String = string_variable.trim().to_string();
    new_variable
}








fn main() {
    let user_name = input_string("Enter your name: ");   
    println!("{} welcome to the game: Toque Fama!!!", user_name);
    info_game();
}
