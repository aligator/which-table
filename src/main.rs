use crate::db::Db;
use crate::db::Odbc;
use crate::info::Dbms;

mod db;
mod info;
mod search;

// These constants will be provided by the CLI input in the future.
const cli_system: &str = "mariadb";
const cli_server: &str = "localhost";
const cli_db: &str = "rust";
const cli_user: &str = "rust";
const cli_pass: &str = "rust";

fn main() {
    let env = Odbc::create_env().unwrap();
    let mut dbc = Odbc::new(&env);

    let dbms = Dbms::get(cli_system).expect("DBMS not supported");
    let con_str = Odbc::build_con_str(dbms.driver_name, cli_server, cli_db, cli_user, cli_pass);

    if let Err(e) = dbc.connect(&con_str) {
        panic!("{}", e.msg);
    }

    match dbc.all_tables() {
        Ok(t) => println!("{:?}", t),
        Err(e) => println!("{}", e.msg),
    }
}