extern crate postgres;

use postgres::Connection;
use postgres::rows::Rows;

pub mod inc;

pub trait Setup {
    fn setup(&Connection) -> postgres::Result<u64>;
}

pub trait Select {
    fn select(&self, &Connection) -> postgres::Result<Rows<'static>>;
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
