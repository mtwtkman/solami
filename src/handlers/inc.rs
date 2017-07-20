extern crate slack;
extern crate postgres;

use postgres::Connection;
use slack::User;
use super::SolamiHandler;
use super::super::db as db;
use db::{Select, Update};
use db::inc::D;

pub fn handle<'a>(p: SolamiHandler, inc_name: &'a str, pg: &Connection) {
    let mut obj = D { user_name: inc_name.to_owned(), count: 0 };
    &obj.select(pg).map(|rows| {
        match rows.is_empty() {
            true => {},
            false => obj.count = rows.get(0).get(0),
        }
        match obj.update(pg) {
            Ok(ref r) => {
                println!("updated.");
                p.send_with_body(
                    format!("{}は{}です。", inc_name, &obj.count + 1).as_str()
                ).map_err(|e| {
                    println!("error occurred with `send_with_body`! ERROR: {}", e);
                });
            },
            Err(ref e) => {
                println!("failed to updated. ERROR: {}", e);
            }
        }
    })
    .map_err(|e| println!("{}", e));
}
