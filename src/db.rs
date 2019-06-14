use odbc::Environment;
use odbc::Connection;
use crate::search;

trait Db {
    #[must_use]
    fn connect<'env>(&mut self, con_str: &str) -> Result<(), Err>;
    fn all_tables(&self) -> &[String];
    fn search(&self, term: &str) -> Vec<search::Res>;
}

pub struct Odbc<'env> {
    pub env: &'env Environment<odbc::Version3>,
    pub con: Option<Connection<'env>>,
}

impl<'env> Db for Odbc<'env> {
    fn connect(&mut self, con_str: &str) -> Result<(), Err> {
        let res = self.env.connect_with_connection_string(con_str);

        return match res {
            Ok(con) => {
                self.con = Option::Some(con);
                Result::Ok(())
            },
            Err(diagnose) => {
                let custom = Err::new(0, &diagnose.to_string());
                Result::Err(custom)
            }
        }
    }

    fn all_tables(&self) -> &[String] {
        unimplemented!();
    }

    fn search(&self, term: &str) -> Vec<search::Res> {
        unimplemented!();
    }
}

#[derive(Debug, Clone)]
struct Err {
    code: u16,
    msg: String,
}

impl Err {
    fn new(code: u16, msg: &str) -> Err {
        Err {
            code,
            msg: String::from(msg)
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Auth<'a> {
    user: &'a str,
    pass: &'a str,
}