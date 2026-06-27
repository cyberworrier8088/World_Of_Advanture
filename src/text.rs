use crate::prompts;
use std::io::{self, Write};

pub fn game() {
    println!("Welcome to World of Adventure!");
    println!("--------------------------------");
    println!("Choose your difficulty:");
    println!("1. Easy");
    println!("2. Medium");
    println!("3. Hard");
    println!("--------------------------------");
    print!("Enter your choice (1-3): ");
    let _ = io::stdout().flush();
    
    let answer = prompts::prompt();
    match answer.as_str() {
        "1" => println!("\nYou chose Easy difficulty. Enjoy the journey!"),
        "2" => println!("\nYou chose Medium difficulty. Stay alert!"),
        "3" => println!("\nYou chose Hard difficulty. Prepare for a true challenge!"),
        _ => println!("\nInvalid choice. Defaulting to Medium difficulty."),
    }

    println!("\nYour adventure begins now...");
    let _ = io::stdout().flush();

    let choice = prompts::prompt();
    if choice.to_lowercase().starts_with('y') {
        
    } else {
        
    }
}