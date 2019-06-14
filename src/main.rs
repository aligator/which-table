use crate::db::Db;

mod db;
mod search;

fn main() {
    let env = db::Odbc::get_environment().unwrap();
    let mut dbc = db::Odbc::new(&env);

    let res = dbc.connect("Driver=SQLite3;Database=test.db");

    match res {
        Ok(_) => print!("run OK"),
        Err(err) => print!("Run {}", err.msg),
    }
}