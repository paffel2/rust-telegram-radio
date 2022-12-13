use ureq;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};
use id3::{Tag, TagLike};



pub fn send_message(token: &str, chat_id: i64, text: &str) -> () {
    let requset_string: &str = &(format!(
        "{0}{1}{2}",
        "https://api.telegram.org/bot", token, "/sendMessage"
    ));
    let chat_id_str: &str = &(format!("{}", chat_id));
    let send_help_message =
        ureq::get(requset_string).send_form(&[("chat_id", chat_id_str), ("text", text)]);
    if send_help_message.is_ok() {
        println!("message send")
    } else {
        println!("message not send")
    }
}

pub fn play_file(path:&str) -> ()
    {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let file = BufReader::new(File::open(path).unwrap());
        let source = Decoder::new(file).unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        sink.append(source);
        sink.sleep_until_end();
    }


pub fn read_info(path:&str) -> String {

    let tag = Tag::read_from_path(path).unwrap();

    let artist = tag.artist().unwrap_or("unknown");
    
    let title = tag.title().unwrap_or("unknown");
    format!("{} - {}",artist, title)


}