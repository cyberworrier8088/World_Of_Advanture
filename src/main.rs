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