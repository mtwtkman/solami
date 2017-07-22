extern crate postgres;

use postgres::Connection;
use postgres::rows::Rows;
use super::{Setup, Update, Select};

pub enum Sign {
    Inc,
    Dec,
}

pub struct D {
    pub user_name: String,
    pub count: i32,
    pub sign: Sign,
}

impl Setup for D {
    fn setup(pg: &Connection) -> postgres::Result<u64> {
        pg.execute("
            CREATE TABLE IF NOT EXISTS increments (
                user_name varchar PRIMARY KEY,
                count     int DEFAULT 0 NOT NULL
            )
            ;
        ", &[])
    }
}

impl Select for D {
    fn select(&self, pg: &Connection) -> postgres::Result<Rows<'static>> {
        pg.query(
            "SELECT count FROM increments WHERE user_name = $1;",
            &[&self.user_name]
        )
    }
}

impl Update for D {
    // NOTE: actually UPSERT
    fn update(&self, pg: &Connection) -> postgres::Result<u64> {
        let sign;
        match self.sign {
            Sign::Inc => sign = "+",
            Sign::Dec => sign = "-",
        }
        let sql = format!("
            INSERT INTO increments VALUES ($1, $2)
            ON CONFLICT
            ON CONSTRAINT increments_pkey
            DO UPDATE SET count = increments.count {} 1
            ;
        ", sign);
        pg.execute(sql.as_str(), &[&self.user_name, &self.count])
    }
}
