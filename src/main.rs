use rand::Rng;
use std::io::{self, Write};
use std::process;   


fn info_game() -> () {
    println!("The objective of the game is to randomly generate 3 to 8 numbers");
    println!("from zero to nine, your duty is to find them with the following clues");
    println!("if you have 1 touch you found 1 number but not in the right place");
    println!("if you found 1 fame you found 1 number and in the right place");
    println!("if you achieve all the fames, congratulations you won");
    println!("Note: if you enter a number with more digits than indicated or repeated numbers, you lose your game")
}


fn generate_random_number() -> i8 {
    let random_number:i8 = rand::thread_rng().gen_range(0..=9);
    random_number
}

fn repeated_number(numbers: String) -> bool {
    let mut chars_seen: Vec<char> = vec![];
    
    for characters in numbers.chars() {
        if chars_seen.contains(&characters){
            return true;
        }
        chars_seen.push(characters);
    } 
    false
}


fn create_random_string_number(large_string: i8) -> String {
    let mut save_numbers_in_string: String = String::new();

    for _ in 0..large_string {
        let mut random: i8;
        let mut compare: String;
        loop {
            random = generate_random_number();
            compare = save_numbers_in_string.clone() + &random.to_string();
            if !repeated_number(compare.clone()) {
                break;
            }
        }
        save_numbers_in_string.push_str(&random.to_string());
    }

    save_numbers_in_string
}



fn input_string(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut string_variable: String = String::new();
    io::stdin().read_line(&mut string_variable).expect("Failed to read line");
    let new_variable: String = string_variable.trim().to_string();
    new_variable
}


fn input_i8(message: &str) -> i8 {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut variable: String = String::new();
    io::stdin().read_line(&mut variable).expect("Something is wrong");
    match variable.trim().parse::<i8>() {
        Ok(num) => num,
        Err(_) => {
            println!("Please, enter a good value.");
            input_i8(message)
        },
    }
}


fn game() -> () {
    
}


fn menu(user_name: String) -> () {
    let choose_play: String = input_string("Do you want to play? y/n: ");
    if choose_play.to_lowercase() == "y" || choose_play.to_lowercase() == "yes" {
        println!("Wiii, we're going to play right now, good luck");
    } else if choose_play.to_lowercase() == "n" || choose_play.to_lowercase() == "no" {
        println!("Good bye {}, see you later", user_name);
        process::exit(0);
    } else {
        println!("Please, you only have to write yes or no.")
    } 
}


fn main() {
    let user_name: String = input_string("Enter your name: ");   
    println!("{} welcome to the game: Toque Fama!!!", user_name);
    info_game();
    println!("I wanna see a random number: {}", create_random_string_number(7));
    menu(user_name);
}
