use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::rng().random_range(1..=100);
    //1..=100 means from 1 to 100 and both 1 and 100 are included

    println!("The secret number is: {secret_number}");
    loop{

         println!("Please enter your guess!");
    
    
    let mut guess = String::new();
    
    
    io::stdin().read_line(&mut guess).expect("Failed to read line!");
    let guess: u32 = guess.trim().parse().expect("Please enter a number");
    
    println!("You guessed: {guess}");
    
    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => {
            println!("You win!");
            break;
        },
    }
    
}
   
}
