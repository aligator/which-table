use odbc::{Connection, DiagnosticRecord, Environment, Statement};

use crate::search;

pub trait Db {
    #[must_use]
    fn connect<'env>(&mut self, con_str: &str) -> Result<(), Err>;
    fn all_tables(&self) -> Result<Vec<String>, Err>;
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

    fn load_all_tables(&self) -> Result<Vec<String>, DiagnosticRecord> {
        let mut tables: Vec<String> = Vec::new();

        let con = self.con.as_ref().unwrap();
        let stmt = Statement::with_parent(con)?;
        
        let mut res = stmt.tables_str("%", "%", "%", "TABLE")?;
        let cols = res.num_result_cols()?;
        
        while let Some(mut cur) = res.fetch()? {
            for i in 1..(cols + 1) {
                let col = i as u16;
                
                if let Some(val) = cur.get_data::<&str>(col)? {
                    tables.push(val.to_owned());
                }
            }
        }
        Ok(tables)
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
                let custom = Err::new(1, &diagnose.to_string());
                Result::Err(custom)
            }
        }
    }

    // Wrap load_all_tables() to allow easy use of '?'
    fn all_tables(&self) -> Result<Vec<String>, Err> {
        
        match self.load_all_tables() {
            Ok(tables) => Result::Ok(tables),
            Err(diagnose) => {
                let custom = Err::new(2, &diagnose.to_string());
                Result::Err(custom)
            }
        }
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
}