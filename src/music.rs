use rodio::{Decoder, OutputStream, Sink};
use std::{
    fs::File,
    io::BufReader,
};

pub fn play_music() {
    let (_stream, handle) = OutputStream::try_default().unwrap();

    let sink = Sink::try_new(&handle).unwrap();

    println!("Loading music...");

    let file = File::open("music.mp3")
        .expect("Cannot find music.mp3");

    let source = Decoder::new(BufReader::new(file)).unwrap();

    sink.append(source);

    println!("Playing...");

    sink.sleep_until_end();
}