use odbc::{Connection, DiagnosticRecord, Environment, Statement};

use crate::search;

pub trait Db {
    #[must_use]
    fn connect<'env>(&mut self, con_str: &str) -> Result<(), Err>;
    fn all_tables(&self) -> Result<Vec<String>, DiagnosticRecord>;
    fn search(&self, term: &str) -> Box<Vec<search::Res>>;
}

pub struct Odbc<'env> {
    pub env: &'env Environment<odbc::Version3>,
    pub con: Option<Connection<'env>>,
}

impl<'env> Odbc<'env> {
    pub fn new(env: &Environment<odbc::Version3>) -> Odbc {
        Odbc {
            env,
            con: None,
        }
    }

    pub fn create_env() -> Result<Environment<odbc::Version3>, Err> {
        let res = odbc::create_environment_v3();
        let env = match res {
            Ok(env) => env,
            Err(diagnose) => {
                let msg = match diagnose {
                    Some(d) => d.to_string(),
                    None => String::from("failed to create environemnt"),
                };
                let custom = Err::new(0, &msg);
                return Result::Err(custom);
            }
        };
        Result::Ok(env)
    }
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

    fn all_tables(&self) -> Result<Vec<String>, DiagnosticRecord> {
        let mut tables: Vec<String> = Vec::new();

        let con = self.con.as_ref().unwrap();
        let stmt = Statement::with_parent(con)?;
        let mut rs = stmt.tables_str("%", "%", "%", "TABLE")?;
        let cols = rs.num_result_cols()?;
        while let Some(mut cursor) = rs.fetch()? {
            for i in 1..(cols + 1) {
                match cursor.get_data::<&str>(i as u16)? {
                    Some(val) => tables.push(val.to_owned()),
                    None => (),
                }
            }
        }

        Ok(tables)
    }

    fn search(&self, term: &str) -> Box<Vec<search::Res>> {
        unimplemented!();
    }
}

#[derive(Debug, Clone)]
pub struct Err {
    pub code: u16,
    pub msg: String,
}

impl Err {
    fn new(code: u16, msg: &str) -> Err {
        Err {
            code,
            msg: String::from(msg)
        }
    }

    fn msg(&self) -> String {
        self.msg.clone()
    }
}