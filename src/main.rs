#[macro_use] extern crate log;

use crate::db::Db;
use crate::db::Odbc;

mod db;
mod search;

const CON_STR: &str = "Driver={MySQL ODBC 8.0 ANSI Driver};server=localhost;database=foodunit2;user=foodunit2;password=foodunit2";

fn main() {
    let env = Odbc::create_env().unwrap();
    let mut dbc = Odbc::new(&env);

    if let Err(e) = dbc.connect(CON_STR) {
        panic!("{}", e.msg);
    }

    match dbc.all_tables() {
        Ok(t) => println!("{:?}", t),
        Err(e) => println!("{}", e.msg),
    }
}