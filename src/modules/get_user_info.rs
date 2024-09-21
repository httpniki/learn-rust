use std::io;
use regex::Regex;

pub fn get_name() -> String {
    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Error reading the name");

    let name = name.trim().to_string();

    return name
}

pub fn get_lastname() -> String {
    let mut lastname = String::new();

    io::stdin()
        .read_line(&mut lastname)
        .expect("Error reading the lastname");
    
    let lastname = lastname.trim().to_string();

    return lastname
}

pub fn get_edad() -> u32 {
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading the age");

    let input = input.trim();

    let age: u32 = match input.parse::<u32>() {
        Ok(num) => num,
        Err(err) => {
            println!("Error: {}", err);
            return 0
        }
    };
    
    return age
}

pub fn get_games() -> [String; 3] {
    let mut input = String::new();
    let regex_with_space = Regex::new(r"^(\w+[\s]*){3}$").unwrap();
    let regex_with_comma = Regex::new(r"^(\w+,\s*){2}\w+$").unwrap();

    const GAMES_INITIAL_VALUE: String = String::new();
    let mut games: [String; 3] = [GAMES_INITIAL_VALUE; 3];

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading the games");

    if regex_with_space.is_match(&input) {
        for (i, v) in input.trim().split(" ").enumerate() {
            let value = v.trim().to_string();
            games[i] = value;
        }       
    }
    
    if regex_with_comma.is_match(&input) {
        for (i, v) in input.trim().split(",").enumerate() {
            let value = v.trim().to_string();
            games[i] = value;
        }
    }

    if !(regex_with_space.is_match(&input)) && !(regex_with_comma.is_match(&input)) {
        panic!("Incorrect games format");
    }

    if games.iter().any(|x| x.len() == 0) {
        panic!("Three games are required, but {} games were obtained", games.len());
    }

    return games
}
