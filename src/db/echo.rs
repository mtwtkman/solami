extern crate postgres;

use postgres::Connection;
use postgres::rows::Rows;
use super::{Setup, Insert, Update, Select, Query, Delete};

#[derive(Default)]
pub struct D {
    pub name: String,
    pub pattern: String,
    pub response: String,
}

impl Setup for D {
    fn setup(pg: &Connection) -> postgres::Result<u64> {
        pg.execute("
            CREATE TABLE IF NOT EXISTS echos (
                name     text PRIMARY KEY,
                pattern  text NOT NULL,
                response text NOT NULL
            )
            ;
        ", &[])
    }
}

impl Insert for D {
    fn insert(&self, pg: &Connection) -> postgres::Result<u64> {
        pg.execute(
            "INSERT INTO echos VALUES ($1, $2, $3);",
            &[&self.name, &self.pattern, &self.response])
    }
}

impl Update for D {
    fn update(&self, pg: &Connection) -> postgres::Result<u64> {
        pg.execute(
            "UPDATE echos SET pattern=$2, response=$3 WHERE name=$1;",
            &[&self.name, &self.pattern, &self.response]
        )
    }
}

impl Select for D {
    fn select(&self, pg: &Connection, query: Option<Query>) -> postgres::Result<Rows<'static>> {
        let q = match query {
            Some(m) => {
                let mut query_strings: Vec<String> = vec!["WHERE".to_owned()];
                for (k, v) in m.iter() {
                    query_strings.push(format!("{}={}", k, v));
                }

                query_strings.iter().map(|v| v.as_str()).collect::<Vec<&str>>().as_slice().join(" AND ")
            },
            None => "".to_owned(),
        };
        let sql = format!("SELECT * FROM echos {};", q.to_owned());
        pg.query(sql.as_str(), &[&self.name]
        )
    }
}
