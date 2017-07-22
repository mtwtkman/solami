extern crate slack;
extern crate postgres;

use postgres::Connection;
use slack::User;
use super::SolamiHandler;
use super::super::db as db;
use db::{Select, Update};
use db::inc::{D, Sign};

pub fn handle<'a>(p: SolamiHandler, target: &'a str, sign: &'a str, pg: &Connection) {
    let s: Sign;
    match sign {
        "++" => s = Sign::Inc,
        "--" => s = Sign::Dec,
        _ => s = Sign::Dec, // XXX: adhoc. I must define about this pattern.
    }
    let mut obj = D { user_name: target.to_owned(), count: 0, sign: s };

    obj.select(pg).map(|rows| {
        match rows.is_empty() {
            true => {},
            false => obj.count = rows.get(0).get(0),
        }
        match obj.update(pg) {
            Ok(r) => {
                if sign == "++" { obj.count += 1; }
                else { obj.count -= 1; }
                println!("updated.");
                p.send_with_body(
                    format!("{}は{}です。", target, obj.count).as_str()
                ).map_err(|e| {
                    println!("error occurred with `send_with_body`! ERROR: {}", e);
                });
            },
            Err(e) => {
                println!("failed to updated. ERROR: {}", e);
            }
        }
    })
    .map_err(|e| println!("{}", e));
}
