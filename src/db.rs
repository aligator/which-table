use core::borrow::Borrow;

use odbc::{Connection, create_environment_v3, DiagnosticRecord, Environment, Statement, Version3};

use crate::search;

pub trait Db {
    #[must_use]
    fn connect<'env>(&mut self, con_str: &str) -> Result<(), Err>;
    fn all_tables(&self) -> Result<Vec<&str>, DiagnosticRecord>;
    fn search(&self, term: &str) -> Vec<search::Res>;
}

pub struct Odbc<'env> {
    pub env: &'env Environment<odbc::Version3>,
    pub con: Option<Connection<'env>>,
}

impl<'env> Odbc<'env> {
    pub fn new(env: &Environment<odbc::Version3>) -> Odbc {
        return Odbc {
            env,
            con: None,
        }
    }

    pub fn get_environment() -> Result<Environment<Version3>, Err> {
        let env_res = create_environment_v3();
        let env: Environment<Version3>;
        match env_res {
            Ok(environment) => env = environment,
            Err(diagnose) => {
                // todo fn to create Err out of diagnostics
                let error = match diagnose {
                    Some(diagnose) => Err::new(1, &diagnose.to_string()),
                    None => Err::new(2, &"odbc environment creation failed".to_string())
                };

                return Result::Err(error);
            }
        }

        return Result::Ok(env)
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

    fn all_tables(&self) -> Result<Vec<&str>, DiagnosticRecord> {
        let mut tables: Vec<&str> = Vec::new();

        let con = self.con.as_ref().unwrap();
        let stmt = Statement::with_parent(con)?;
        let mut rs = stmt.tables_str("%", "%", "%", "TABLE")?;
        let cols = rs.num_result_cols()?;
        while let Some(mut cursor) = rs.fetch()? {
            for i in 1..(cols + 1) {
                match cursor.get_data::<&str>(i as u16)? {
                    Some(val) => tables.push(val),
                    None => (),
                }
            }
        }

        Ok(tables)
    }

    fn search(&self, term: &str) -> Vec<search::Res> {
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
}

#[derive(Debug, Copy, Clone)]
struct Auth<'a> {
    user: &'a str,
    pass: &'a str,
}