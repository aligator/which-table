use crate::db::Db;
use crate::db::Odbc;
use crate::info::Dbms;

mod db;
mod info;
mod search;

// These constants will be provided by the CLI input in the future.
const CLI_SYSTEM: &str = "mariadb";
const CLI_SERVER: &str = "localhost";
const CLI_DB: &str = "rust";
const CLI_USER: &str = "rust";
const CLI_PASS: &str = "rust";

fn main() {
    let env = Odbc::create_env().unwrap();
    let mut dbc = Odbc::new(&env);

    let dbms = Dbms::get(CLI_SYSTEM).expect("DBMS not supported");
    let con_str = Odbc::build_con_str(dbms.default_driver, CLI_SERVER, CLI_DB, CLI_USER, CLI_PASS);

    if let Err(e) = dbc.connect(&con_str) {
        panic!("{}", e.msg);
    }

    // match dbc.all_tables() {
    //     Ok(t) => println!("{:?}", t),
    //     Err(e) => println!("{}", e.msg),
    // }

    let res = dbc.search("Pizza");
}