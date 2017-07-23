extern crate slack;
extern crate slack_api;
extern crate regex;
extern crate postgres;

use std::clone::Clone;
use std::collections::HashMap;
use slack::{Event, RtmClient, Message, User};
use regex::Regex;
use postgres::{Connection, TlsMode};

mod handlers;
use handlers::{SolamiHandler, Users, inc, echo};

mod db;
use db::Setup;

struct MyHandler {
    users: Users,
    me: User,
    pg_connection: Connection,
}

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
                let re = Regex::new(r"(^(?P<target>^\w+)(?P<sign>(\+\+|\-\-|\?))\s*$|!(?P<command>\w+)\s+(?P<body>.+))").unwrap();
                re.captures(&message_standard.text.as_ref().unwrap())
                    .map_or_else(|| {}, |caps| {
                        let handler = SolamiHandler {
                            sender: cli.sender(),
                            channel_id: &message_standard.channel.as_ref().unwrap(),
                        };
                        if let Some(target) = caps.name("target") {
                            inc::handle(&handler, target.as_str(), &caps["sign"], &self.pg_connection);
                        } else {
                            match &caps["command"] {
                                "echo" => {
                                    let mut splited = caps["body"].split_whitespace();
                                    let cmd = splited.next().unwrap();
                                    let mut rest = Vec::new();
                                    for s in splited {
                                        rest.push(s);
                                    }
                                    echo::handle(&handler, cmd, rest.join(" ").as_str(), &self.pg_connection);
                                },
                                _ => println!("Unknown command."),
                            }
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
    let api_key = std::env::var("SLACK_API_TOKEN").unwrap();
    let r = RtmClient::login(&api_key);
    let db_url = std::env::var("DATABASE_URL").unwrap();
    let conn = Connection::connect(&*db_url, TlsMode::None).unwrap();
    match db::inc::D::setup(&conn) {
        Ok(_) => println!("created increments table."),
        Err(e) => println!("failed to create increments table. ERROR: {}", e),
    }
    match db::echo::D::setup(&conn) {
        Ok(_) => println!("created echos table."),
        Err(e) => println!("failed to create echos table. ERROR: {}", e),
    }
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
                pg_connection: conn,
            };
            client.run(&mut handler);
        },
        Err(err) => panic!("Error: {}", err),
    }
}
