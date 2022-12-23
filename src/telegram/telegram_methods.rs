use ureq;

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
