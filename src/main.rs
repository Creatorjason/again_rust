// creating a guessing game in rust;
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
  
    println!("Welcome to guess a number game");
    println!("Guess a number:");
    loop{
    let mut user_guess : String = String::new();
    // get user input
    io::stdin().read_line(&mut user_guess).expect("Expects a string");
    // generate random number
    let random_number = rand::thread_rng().gen_range(1..=2);
    //println!("{}", random_number)
    // compare user guess to random number
    // user_guess stores a string value, convert to u32 before comparing
    // We could also handle invalid inputs, using the variants of the Result type
    let user_guess: u32 = match user_guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    };
    //println!("{}", user_guess + 1) 
    
       
    match user_guess.cmp(&random_number){
        Ordering::Less => println!("Too low, try again"),
        Ordering::Greater => println!("Too high, you can do better"),
        Ordering::Equal =>{ 
           println! ("Hurray! You won");
            break;
        },
        
};
    println!("Almost:");
}

}