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
            match tabs {
                Ok(tabs) => print!("Tables {:?}", tabs),
                Err(err) => print!("Run {}", err.msg),
            }

        },
        Err(err) => print!("Run {}", err.msg),
    }
}