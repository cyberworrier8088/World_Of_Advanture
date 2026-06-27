// this is the prompts.rs
// this input in python like more easy for to use made


use std::io;

pub fn prompt() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}