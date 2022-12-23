use id3::{Tag, TagLike};
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

pub fn play_file(path: &str) -> () {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open(path).unwrap());
    let source = Decoder::new(file).unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(source);
    sink.sleep_until_end();
}

pub fn read_info(path: &str) -> String {
    let tag = Tag::read_from_path(path).unwrap();

    let artist = tag.artist().unwrap_or("unknown");

    let title = tag.title().unwrap_or("unknown");
    format!("{} - {}", artist, title)
}
