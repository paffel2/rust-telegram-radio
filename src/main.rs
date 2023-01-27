use rand::Rng;
use rodio::Sink;
use rodio::{Decoder, OutputStream};
use rust_telegram_radio::music_methods::*;
use rust_telegram_radio::telegram::telegram_methods::*;
use rust_telegram_radio::telegram::telegram_structures::Control;
use std::env;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::string::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let chat_id = args[2].parse::<i64>().unwrap();
    let admin_id = args[3].parse::<u64>().unwrap();
    let token = args[1].as_str();
    let paths: Vec<Result<fs::DirEntry, std::io::Error>> =
        fs::read_dir("./music_lib").unwrap().collect();

    let nums = paths.len();
    let mut rng = rand::thread_rng();
    let mut update_id: u64 = 0;

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let mut status = Control::Play;

    loop {
        let updates = get_updates(&token, &update_id);
        match updates {
            Ok(result) => {
                let vec_of_updates = result.result.unwrap_or(vec![]);
                let (next_upd_id, control_message) =
                    find_owner_control_message(admin_id, vec_of_updates);
                update_id = next_upd_id;
                match control_message {
                    Control::Play => {
                        sink.stop();
                        let i = rng.gen_range(0..nums);
                        let next_track_path =
                            format!("{}", paths[i].as_ref().unwrap().path().display());
                        let next_track = read_info(&next_track_path);
                        send_message(&token, chat_id, &next_track);
                        let file = BufReader::new(File::open(next_track_path).unwrap());
                        let source = Decoder::new(file).unwrap();

                        sink.append(source);
                        println!("song started");
                        sink.play();
                        status = Control::Play;
                    }
                    Control::Stop => {
                        println!("song stopped");
                        sink.stop();
                        status = Control::Stop;
                    }
                    _ => match status {
                        Control::Stop => (),
                        _ => {
                            if sink.empty() {
                                let i = rng.gen_range(0..nums);
                                let next_track_path =
                                    format!("{}", paths[i].as_ref().unwrap().path().display());
                                let next_track = read_info(&next_track_path);
                                send_message(&token, chat_id, &next_track);
                                let file = BufReader::new(File::open(next_track_path).unwrap());
                                let source = Decoder::new(file).unwrap();

                                sink.append(source);
                                println!("song started");
                                sink.play();
                            } else {
                                ()
                            }
                        }
                    },
                }
            }
            _ => (),
        }
    }
}
