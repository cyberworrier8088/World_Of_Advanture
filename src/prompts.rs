use std::io;

fn prompt() -> bool {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    return input.to_ascii_lowercase().starts_with("y");
    
}