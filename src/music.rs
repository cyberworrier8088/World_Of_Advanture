use rodio::{source::Source, Decoder, OutputStream, Sink};
use std::{
    fs::File,
    io::BufReader,
};

pub fn play_music() -> Option<(OutputStream, Sink)> {
    println!("Loading music...");

    let file = match File::open("music.mp3") {
        Ok(f) => f,
        Err(_) => {
            println!("Warning: music.mp3 not found. Running without background music.");
            return None;
        }
    };

    let (stream, handle) = match OutputStream::try_default() {
        Ok(s) => s,
        Err(e) => {
            println!("Warning: Could not open default audio output: {}. Running without background music.", e);
            return None;
        }
    };

    let sink = match Sink::try_new(&handle) {
        Ok(s) => s,
        Err(e) => {
            println!("Warning: Could not create audio sink: {}. Running without background music.", e);
            return None;
        }
    };

    match Decoder::new(BufReader::new(file)) {
        Ok(source) => {
            sink.append(source.repeat_infinite());
            println!("Playing background music...");
            Some((stream, sink))
        }
        Err(e) => {
            println!("Warning: Failed to decode music: {}. Running without background music.", e);
            None
        }
    }
}