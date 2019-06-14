use crate::db::Db;

mod db;
mod search;

fn main() {
    let env = db::Odbc::create_env().unwrap();
    let mut dbc = db::Odbc::new(&env);

    let res = dbc.connect("Driver=SQLite3;Database=test.db");

    match res {
        Ok(_) => {
            let tabs = dbc.all_tables();
            print!("Tables {:?}", tabs.unwrap());
        },
        Err(err) => print!("Run {}", err.msg),
    }
}