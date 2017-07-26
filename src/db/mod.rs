extern crate postgres;

use std::collections::HashMap;
use postgres::Connection;
use postgres::rows::Rows;

pub mod inc;
pub mod echo;
pub mod rss;

pub trait Setup {
    fn setup(&Connection) -> postgres::Result<u64>;
}

pub type Query<'a> = HashMap<&'a str, &'a str>;

pub trait Select {
    fn select(&self, &Connection, Option<Query>) -> postgres::Result<Rows<'static>>;
}

pub trait Insert {
    fn insert(&self, &Connection) -> postgres::Result<u64>;
}

pub trait Update {
    fn update(&self, &Connection) -> postgres::Result<u64>;
}

pub trait Delete {
    fn delete(&self, &Connection) -> postgres::Result<u64>;
}
