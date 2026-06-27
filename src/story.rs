// this is the story.rs


use std::collections::HashMap;
use std::io::{self, Write};

use crate::prompts;
use crate::scene::{Choice, Scene};

// ============================================================================
//  HELPER MACROS (For making code shorter ) — make adding new story levels super easy! :)
// ============================================================================
//
//  To add a NEW scene, just write:
//
//      add_scene!(story, "scene_id", "Scene Title", "Story text here...",
//          choice!("Choice text" => "next_scene_id"),
//          choice!("Another choice" => "other_scene_id"),
//      );
//
//  For an ENDING scene (no choices), leave choices empty:
//
//      add_scene!(story, "scene_id", "Title", "Story text...",);
//
// ============================================================================

/// Creates a single Choice { text, next_scene_id } without boilerplate.
macro_rules! choice {
    ($text:expr => $next:expr) => {
        Choice {
            text: $text.to_string(),
            next_scene_id: $next.to_string(),
        }
    };
}

/// Inserts a scene into the story HashMap in one compact call.
macro_rules! add_scene {
    ($map:expr, $id:expr, $title:expr, $story:expr, $( $choice:expr ),* $(,)? ) => {
        $map.insert(
            $id.to_string(),
            Scene {
                title: $title.to_string(),
                story: $story.to_string(),
                choices: vec![ $( $choice ),* ],
            },
        );
    };
}

pub fn wait() {
    println!("Press Enter to continue...");
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
}

pub fn intro() {
    println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
    println!("         WORLD OF ADVENTURE     ");
    println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
    println!();
    println!("The rain is falling...");
    println!("You slowly open your eyes.");
    println!("You are lying in a dark forest.");
    println!("You don't remember who you are.");
    println!();
}

pub fn build_story() -> HashMap<String, Scene> {
    let mut story = HashMap::new();

    // Starting Scene
    add_scene!(story, "start", "Dark Forest Clearing",
        "You are standing in the middle of a dark forest clearing. The rain is cold against your face.",
        choice!("Look Around"       => "look_around"),
        choice!("Check Pockets"     => "check_pocket"),
        choice!("Walk North"        => "walk_north_no_key"),
    );

    // Look Around
    add_scene!(story, "look_around", "Looking Around",
        "The forest is quiet except for the rain. You hear distant birds singing, but see nothing but trees.",
        choice!("Go back to the clearing" => "start"),
    );

    // Check Pockets (gives the key)
    add_scene!(story, "check_pocket", "Pocket Search",
        "You search your pockets and feel something cold. You pull out a strange silver key with a dragon symbol carved into it!",
        choice!("Walk North with the key" => "walk_north_with_key"),
    );

    // Walk North (No Key) 
    add_scene!(story, "walk_north_no_key", "Forest Path",
        "A narrow path appears between the trees. The trees become taller, and a strange fog covers the ground. In front of you, you see a small wooden house and a dark cave.",
        choice!("Try to enter the small wooden house" => "wooden_house_locked"),
        choice!("Enter the dark cave"                 => "dark_cave"),
        choice!("Go back south"                       => "start"),
    );

    // Walk North (With Key)
    add_scene!(story, "walk_north_with_key", "Foggy Forest Path",
        "Holding the dragon key, you walk deeper into the forest. A strange fog covers the ground. In front of you, you see a small wooden house and a dark cave.",
        choice!("Use the key on the wooden house door" => "wooden_house_unlocked"),
        choice!("Enter the dark cave"                  => "dark_cave"),
        choice!("Follow the forest path further"       => "deep_forest"),
    );

    // Locked House
    add_scene!(story, "wooden_house_locked", "Locked Door",
        "You try to turn the doorknob, but the house is locked tight. There is a keyhole shaped like a dragon symbol. You need a key to enter.",
        choice!("Go back to the path" => "walk_north_no_key"),
    );

    // Unlocked House (Win)
    add_scene!(story, "wooden_house_unlocked", "Cozy Wooden House",
        "You insert the dragon silver key into the keyhole. It turns with a satisfying click! The door swings open. Inside, a fireplace crackles warmly, and there is a table laden with food and gold. You are safe! YOU WIN!",
    );

    // Dark Cave (Game Over)
    add_scene!(story, "dark_cave", "Inside the Cave",
        "You step inside the cold, pitch-black cave. Suddenly, a pair of glowing red eyes appear in the darkness. A loud growl echoes, and a giant cave bear leaps at you! GAME OVER.",
    );

    // Deep Forest (Game Over)
    add_scene!(story, "deep_forest", "Lost in the Woods",
        "You ignore the cave and the house and follow the path deeper. The trees grow so close together they block out all light. You quickly lose your bearings and wander the forest forever. GAME OVER.",
    );

    // ════════════════════════════════════════════════════════════════════
    //  ADD YOUR NEW LEVELS BELOW — just copy this template:
    //
    //  add_scene!(story, "your_scene_id", "Your Scene Title",
    //      "Your story text goes here...",
    //      choice!("Choice A text" => "next_scene_id_a"),
    //      choice!("Choice B text" => "next_scene_id_b"),
    //  );
    //
    //  For an ending (no choices):
    //  add_scene!(story, "your_ending_id", "Ending Title",
    //      "Your ending text...",
    //  );
    // ════════════════════════════════════════════════════════════════════

    story
}

pub fn scene1() {
    intro();
    wait();

    let story_map = build_story();
    let mut current_scene_id = "start".to_string();

    loop {
        let scene = match story_map.get(&current_scene_id) {
            Some(s) => s,
            None => {
                println!("Thank you for playing World of Adventure!");
                break;
            }
        };

        println!("========================================");
        println!("        {}", scene.title.to_uppercase());
        println!("========================================");
        println!();
        println!("{}", scene.story);
        println!();

        if scene.choices.is_empty() {
            println!("--- THE END ---");
            break;
        }

        for (i, choice) in scene.choices.iter().enumerate() {
            println!("{}. {}", i + 1, choice.text);
        }
        println!();

        print!("Enter your choice: ");
        let _ = io::stdout().flush();

        let choice = prompts::prompt();
        if let Ok(choice_idx) = choice.parse::<usize>() {
            if choice_idx > 0 && choice_idx <= scene.choices.len() {
                current_scene_id = scene.choices[choice_idx - 1].next_scene_id.clone();
                println!();
                continue;
            }
        }
        println!("\nInvalid choice. Please choose a valid number.\n");
    }
}