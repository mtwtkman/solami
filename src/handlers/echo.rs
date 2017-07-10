extern crate slack_api;

use slack_api::MessageStandard;

pub fn handle(message_standard: MessageStandard) {
    println!("{:?}", message_standard);
    let text = message_standard.text.unwrap();
    let user = message_standard.user.unwrap();
    println!("{} said {}", user, text);
}
