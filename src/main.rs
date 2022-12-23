use rand::Rng;
use rust_telegram_radio::music_methods::*;
use rust_telegram_radio::telegram::telegram_methods::*;
use std::env;
use std::fs;
mod telegram;

fn main() {
    let args: Vec<String> = env::args().collect();
    let token = &args[1];
    let chat_id = args[2].parse::<i64>().unwrap();
    let paths: Vec<Result<fs::DirEntry, std::io::Error>> =
        fs::read_dir("./music_lib").unwrap().collect();

    let nums = paths.len();
    let mut rng = rand::thread_rng();

    loop {
        let i = rng.gen_range(0..nums);
        let next_track_path = format!("{}", paths[i].as_ref().unwrap().path().display());
        let next_track = read_info(&next_track_path);
        send_message(token, chat_id, &next_track);
        play_file(&next_track_path)
    }
}
