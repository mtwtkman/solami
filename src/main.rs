extern crate slack;
extern crate slack_api;
extern crate regex;

use std::clone::Clone;
use std::collections::HashMap;
use slack::{Event, RtmClient, Message, User};
use slack_api::users::{list, ListError, ListRequest};
use slack_api::requests::{Client};
use regex::Regex;

mod handlers;
use handlers::{SolamiHandler, echo};

type Users = HashMap<String, User>;

struct MyHandler {
    users: Users,
    me: User,
}

#[allow(unused_variables)]
impl slack::EventHandler for MyHandler {
    fn on_event(&mut self, cli: &RtmClient, event: Event) {
        println!("on_event(event: {:?})", event);
        if let Event::Message(message) = event {
            if let Message::Standard(message_standard) = *message {
                let user = &message_standard.user.as_ref().unwrap();
                if **user == *self.me.id.as_ref().unwrap() {
                    println!("bot saied");
                    return
                }
                let re = Regex::new(r"!(?P<command>\w+)\s+(?P<pattern>\w+)").unwrap();
                re.captures(&message_standard.text.as_ref().unwrap())
                    .map_or_else(|| {}, |ref caps| {
                        let channel_id = &message_standard.channel.as_ref().unwrap();
                        let pattern = &caps["pattern"];
                        let handler = SolamiHandler {
                            sender: cli.sender(),
                            pattern: pattern,
                            channel_id: channel_id
                        };
                        match &caps["command"] {
                            "echo" => {
                                echo::handle(&handler);
                            },
                            _ => println!("Unknown command."),
                        }
                    });
            }
        }
    }

    fn on_close(&mut self, cli: &RtmClient) {
        println!("on_close");
    }

    fn on_connect(&mut self, cli: &RtmClient) {
        println!("on_connect");
    }
}

fn main() {
    let api_key: String = std::env::var("SLACK_API_TOKEN").unwrap();
    let r = RtmClient::login(&api_key);
    match r {
        Ok(client) => {
            let start_response = &client.start_response();
            let mut users: Users = HashMap::new();
            for user in start_response.users.as_ref().unwrap().into_iter() {
                let id = user.id.as_ref().unwrap().clone();
                users.insert(id, user.clone());
            }
            let mut handler = MyHandler {
                users: users,
                me: start_response.slf.as_ref().unwrap().clone(),
            };
            client.run(&mut handler);
        },
        Err(err) => panic!("Error: {}", err),
    }
}
