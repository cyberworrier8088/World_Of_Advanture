// this is the main.rs
// its run and play the game!
// this can play music and game and etc :)


mod music;
mod prompts;
mod text;
mod story;
mod scene;

fn main() {
    let _music = music::play_music();
    let _game = text::game();

    story::scene1();
}