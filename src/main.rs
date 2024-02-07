use rand::Rng;
use std::process;   
use std::io::{self, Write};


struct Famas {
    toque: u8,
    fama: u8
}

// creation of the opstion for copy and clone a struct #[derive(Copy, Clone)]
struct DataParty{
    win: u8,
    lose: u8,
    parties: u8,
    best_party: i8
}

fn info_game() -> () {
    println!("========================================================================================================");
    println!("The objective of the game is to randomly generate 3 to 8 numbers");
    println!("from zero to nine, your duty is to find them with the following clues");
    println!("if you have 1 touch you found 1 number but not in the right place");
    println!("if you found 1 fame you found 1 number and in the right place");
    println!("if you achieve all the fames, congratulations you won");
    println!("Note: if you enter a number with more digits than indicated or repeated numbers, you lose your game");
    println!("========================================================================================================\n");
}


fn show_stadistics(parties: DataParty) -> () {
    if parties.parties <= 1{
        println!("\nYou have played {} game", parties.parties);
    } else {
        println!("\nYou have played {} games ", parties.parties); 
        println!("You won {} games", parties.win);  
        println!("You have lost {} games", parties.lose);
        println!("And your best party was with {} try", parties.best_party);
    }
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
            println!("Please, enter a valid term.");
            input_i8(message)
        },
    }
}


fn count_toque_fama(random_numbers: String, user_answer: String) -> Famas {
    let mut toques_famas: Famas = Famas{toque: 0, fama: 0};
    for (index, num) in user_answer.chars().enumerate(){
        if Some(num) == random_numbers.chars().nth(index) {
            toques_famas.fama += 1;
        } else if random_numbers.contains(num) {
            toques_famas.toque += 1;
        }
    }
    toques_famas     
}


fn check_user_response(random_numbers: String, user_answer: String, quantity: i8) -> i8{
    // as usize is for the conflicts of types.
    if user_answer.len() != quantity as usize {
        println!("Your entry does not meet the number of allowed characters, which were {} and you entered {}, so you lose.", quantity, user_answer.len());
        return -1;
    } else if repeated_number(user_answer.clone()){
        println!("You entry repiet numbers, for that reason, you lose. ");
        return  -1;
    } else if random_numbers.clone() == user_answer.clone(){
        return 1;
    } else {
        let party_data: Famas = count_toque_fama(random_numbers, user_answer);
        println!("\nToques: {} \nFamas: {} \n", party_data.toque, party_data.fama);
        return 0;
    }
}


fn generation_of_turns_in_the_game(games: i8, random_numbers: String, quantity: i8) -> (i8, i8) { 
    let mut party: i8 = 0;
    let mut win_or_lose: i8 = 0;
    loop {
        party += 1;
        let message: String = format!("{} ) Enter the number you think it is: ", party);
        let ask_to_user: String = input_string(&message);   
        let result_party: i8 = check_user_response(random_numbers.clone(), ask_to_user, quantity.clone());
        
        if games == party || result_party == -1{
            win_or_lose -= 1;
             println!("This is the random number: {}", random_numbers.clone());
            break;
        } else if result_party == 1 {
            println!("Excelent, you won");
            win_or_lose = 1;
            break;
        }
    }
    (win_or_lose, party)
}


fn check_the_number(message_to_user: &str, message_in_case_of_error: &str, first_conditional: i8, second_conditional: i8) -> (i8){
    let mut ask_number:i8 = input_i8(message_to_user);
    loop {
        if ask_number >= first_conditional && ask_number <= second_conditional {
            break;
        }
        println!("\nPlease, enter a valid value");
        ask_number = input_i8(message_in_case_of_error);  
    }

    ask_number
}


fn game() -> (i8, i8) {
    let quantity_numbers: i8 = check_the_number("\nHow many digits you want to play (3 to 9)? ", "Enter again, how many digits do you want to play(3 to 9)? ", 3, 9);

    let random_number_for_user: String = create_random_string_number(quantity_numbers);
    
    let how_many_games: i8 = check_the_number("How many games do you want to play(2 to 30)? ", "How many games do you want to play (2 to 30)? ", 2, 30); 

    let get_resul_party: (i8, i8) = generation_of_turns_in_the_game(how_many_games, random_number_for_user, quantity_numbers);
    get_resul_party
}


fn menu(user_name: String) -> () {
    let mut games_play: DataParty = DataParty{win: 0, lose: 0, parties: 0, best_party: 30};
    
    loop {
        let choose_play: String = input_string("Do you want to play? y/n: ");
        if choose_play.to_lowercase() == "y" || choose_play.to_lowercase() == "yes" {
            let start_game: (i8, i8) = game();
            if start_game.0 == -1 {
                games_play.lose += 1;
            } else if start_game.0 == 1{
                games_play.win += 1;
                if games_play.best_party > start_game.1 {
                    games_play.best_party = start_game.1;
                }        
            }
            
            games_play.parties += 1;
        } else if choose_play.to_lowercase() == "n" || choose_play.to_lowercase() == "no" {
            show_stadistics(games_play);
            println!("Good bye {}, see you later", user_name);
            process::exit(0);
        } else {
            println!("Please, you only have to write yes or no.")
        } 
    }
}


fn main() {
    let user_name: String = input_string("Enter your name: ");   
    println!("{} welcome to the game: Toque Fama!!!", user_name);
    info_game();
    menu(user_name);
}
