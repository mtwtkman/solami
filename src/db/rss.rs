extern crate postgres;

use postgres::Connection;
use postgres::rows::Rows;
use super::{Setup, Insert, Update, Select, Query, Delete};

#[derive(Default)]
pub struct D {
    name: String,
    url: String,
}

impl Setup for D {
    fn setup(pg: &Connection) -> postgres::Result<u64> {
        pg.execute("
            CREATE TABLE IF NOT EXISTS rsses (
                name text PRIMARY KEY,
                url  text,
                UNIQUE(url)
            )
            ;
        ", &[])
    }
}

impl Insert for D {
    fn insert(&self, pg: &Connection) -> postgres::Result<u64> {
        pg.execute(
            "INSERT INTO rsses VALUES ($1, $2);",
            &[&self.name, &self.url]
        )
    }
}

impl Update for D {
    fn update(&self, pg: &Connection) -> postgres::Result<u64> {
        pg.execute(
            "UPDATE rsses SET url = $2 WHERE name = $1;",
            &[&self.name, &self.url]
        )
    }
}

impl Select for D {
    fn select(&self, pg: &Connection, query: Option<Query>) -> postgres::Result<Rows<'static>> {
        let q = match query {
            Some(m) => {
                let mut query_strings: Vec<String> = vec![];
                for (k, v) in m.iter() {
                    query_strings.push(format!("{}='{}'", k, v));
                }

                let ands = query_strings.iter().map(|v| v.as_str()).collect::<Vec<&str>>().as_slice().join(" AND ");
                "WHERE ".to_owned() + &*ands
            },
            None => "".to_owned(),
        };
        let sql = format!("SELECT name, url FROM rsses {};", q);
        pg.query(sql.as_str(), &[])
    }
}

impl Delete for D {
    fn delete(&self, pg: &Connection) -> postgres::Result<u64> {
        pg.execute("DELETE FROM rsses WHERE name = $1;", &[&self.name])
    }
}
