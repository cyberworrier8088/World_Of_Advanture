pub fn game() {
    println!("Welcome to World of Adventure!");
    println!("--------------------------------");
    println!("Choose your difficulty:");
    println!("1. Easy");
    println!("2. Medium");
    println!("3. Hard");
    println!("--------------------------------");
    println!("Enter your choice: ");
    let choice = std::io::stdin().read_line(&mut String::new()).expect("Failed to read line");
    
}