use std::io;

use crate::prompts;
use crate::scene::Scene;


pub fn wait() {
    println!("Press Enter to continue...");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

pub fn intro() {
    println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
    println!("         WORLD OF ADVENTURE     ");
    println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
    println!();
    println!("The rains is falling...");
    println!("You slowly open your eyes.");
    println!("You are lying in a dark forest.");
    println!("You don't remember who you are.");
    println!();
}

pub fn first_choice() {
    println!("What will you do?");
    println!();
    println!("1. Look Around");
    println!("2. Check Pocket");
    println!("3. Walk North");
    println!();
}

pub fn choose() {
    println!();
    println!("Enter your choice");

    let choice = prompts::prompt();

    match choice.as_str() {
        "1" => {
            println!();
            println!("You look around.");
            println!("The forest is quiet.");
            println!("You hear birds singing.");
        }
        "2" => {
            println!();
            println!("You check your pocket.");
            println!("You find a strange silver key.");
            println!("It has a dragon symbol carved into it.");
            println!();
            second_scene();
        }
        "3" => {
            println!();
            println!("You begin walking north.");
            println!("A narrow path appears between the trees.");
        }

        _ => {
            println!("Invalid choice.");
        }
    }
}


// this for second scene
pub fn second_scene() {
    println!("================================");
    println!("        FOREST PATH"             );
    println!("================================");
    println!();
    println!("Holding the key, you walk deeper into the forest.");
    println!("The trees become taller.");
    println!("A strange fog covers the ground.");
    println!();
    println!("In front of you, you see:");
    println!();
    println!("1. A small wooden house");
    println!("2. A dark cave");
    println!("3. Follow the forest path");
}


pub fn create_scene() -> Scene {
    Scene {
        title: String::from("Dark Forest"),
        story: String::from(
            "The rain is falling.\nYou slowly open your eyes.\nYou don't remember who you are."
        ),
        choices: vec![
            String::from("Look Around"),
            String::from("Check Pocket"),
            String::from("Walk North"),
        ],
    }
}


pub fn scene1() {
    intro();
    wait();
    first_choice();
    choose();
}