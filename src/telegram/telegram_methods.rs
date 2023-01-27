use ureq;

use crate::telegram::telegram_structures::*;

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
        println!("message not send, {:#?}", send_help_message.unwrap_err())
    }
}

pub fn get_updates(token: &str, update_id: &u64) -> Result<TgResponse<Vec<TgUpdate>>, ureq::Error> {
    let requset_string: &str = &(format!(
        "{0}{1}{2}",
        "https://api.telegram.org/bot", token, "/getUpdates"
    ));
    let update_id: &str = &format!("{}", update_id);
    let messages = ureq::get(requset_string)
        .send_form(&[("offset", update_id), ("timeout", "10")])?
        .into_json()?;
    Ok(messages)
}

pub fn find_owner_control_message(owner_id: u64, updates: Vec<TgUpdate>) -> (u64, Control) {
    let mut next_update: u64 = 0;
    let mut last_update: TgUpdate = TgUpdate {
        update_id: 0,
        message: None,
    };
    for update in updates {
        if update.update_id > next_update {
            next_update = update.update_id;
            last_update = update;
        }
    }

    match last_update.message {
        Some(mess) => {
            if mess.from == Some(TgUser { id: owner_id }) {
                let checking_text = mess.text.map(to_lowercase1);
                if checking_text == Some("play".to_string()) {
                    return (next_update + 1, Control::Play);
                } else if checking_text == Some("stop".to_string()) {
                    return (next_update + 1, Control::Stop);
                } else {
                    return (next_update + 1, Control::Nothing);
                }
            }
        }
        None => return (next_update + 1, Control::Nothing),
    }

    (next_update + 1, Control::Nothing)
}

fn to_lowercase1(prev: String) -> String {
    prev.to_lowercase()
}
