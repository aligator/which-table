use crate::db::Db;

mod db;
mod search;

const CON_STR: &str = "Driver={MySQL ODBC 8.0 ANSI Driver};server=localhost;database=foodunit2;user=foodunit2;password=foodunit2";

fn main() {
    let env = db::Odbc::create_env().unwrap();
    let mut dbc = db::Odbc::new(&env);

    dbc.connect(CON_STR).expect("failed to connect to database");

    match dbc.all_tables() {
        Ok(t) => println!("{:?}", t),
        Err(e) => println!("{}", e.msg),
    }
}