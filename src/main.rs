extern crate slack;
extern crate slack_api;

use std::clone::Clone;
use std::collections::HashMap;
use slack::{Event, RtmClient, Message, User};
use slack_api::users::{list, ListError, ListRequest};
use slack_api::requests::{Client};


struct MyHandler {
    users: HashMap<String, User>,
}

#[allow(unused_variables)]
impl slack::EventHandler for MyHandler {
    fn on_event(&mut self, cli: &RtmClient, event: Event) {
        println!("on_event(event: {:?})", event);
        if let Event::Message(message) = event {
            if let Message::Standard(message_standard) = *message{
                println!("{:?}", message_standard);
                let text = message_standard.text.unwrap();
                let user = message_standard.user.unwrap();
                println!("{} said {}", user, text);
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

fn users(api_key: &String) -> Result<HashMap<String, User>, ListError<slack_api::requests::Error>>  {
    let client = Client::new().unwrap();
    let list_request = ListRequest { presence: Some(true) };
    list(&client, &api_key, &list_request).and_then(|response| {
        let mut users_map: HashMap<String, User> = HashMap::new();
        for member in response.members.into_iter().flat_map(Vec::into_iter) {
            let id = member.id.as_ref().unwrap().clone();
            users_map.insert(id, member);
        }
        Ok(users_map)
    })
}

fn main() {
    let api_key: String = std::env::var("SLACK_API_TOKEN").unwrap();
    let r = RtmClient::login(&api_key);
    match r {
        Ok(client) => {
            match users(&api_key) {
                Ok(users_map) => {
                    let mut handler = MyHandler { users: users_map };
                    client.run(&mut handler);
                },
                Err(err) => println!("{}", err),
            }
        },
        Err(err) => panic!("Error: {}", err),
    }
}
