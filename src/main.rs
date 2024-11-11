mod modules;
use modules::get_user_info;

struct User {
    name: String,
    lastname: String,
    age: u32,
    games: [String; 3]
}

fn main() {
    let mut user = User { 
        name: String::new(), 
        lastname: String::new(),
        age: 0,
        games: [String::new(), String::new(), String::new()]
    };

    println!("What is your name?");
    user.name = get_user_info::get_name();

    println!("What is your lastname?");
    user.lastname = get_user_info::get_lastname();
    
    println!("How old are you?");
    user.age = get_user_info::get_age();

    println!("What are your favorite games? (3)");
    user.games = get_user_info::get_games();
    
    println!("-------------------------");
    println!("Name: {}", user.name);
    println!("Lastname: {}", user.lastname);
    println!("Age: {}", user.age);
    println!("Favorite games: {:?}", user.games);
    println!("-------------------------");
}
