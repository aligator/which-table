use odbc::{Connection, DiagnosticRecord, Environment, Statement};

use crate::search;

type TableMeta = [String; 4];

pub trait Db {
    #[must_use]
    fn connect<'env>(&mut self, con_str: &str) -> Result<(), Err>;
    fn all_tables(&self) -> Result<Vec<Vec<String>>, Err>;
    fn search(&mut self, term: &str) -> Result<Box<Vec<search::Res>>, Err>;
}

pub struct Odbc<'env> {
    pub env: &'env Environment<odbc::Version3>,
    pub con: Option<Connection<'env>>,
    tables: Option<Vec<Vec<String>>>,
}

impl<'env> Odbc<'env> {
    pub fn new(env: &Environment<odbc::Version3>) -> Odbc {
        Odbc {
            env,
            con: None,
            tables: None,
        }
    }

    pub fn create_env() -> Result<Environment<odbc::Version3>, Err> {
        let res = odbc::create_environment_v3();
        let env = match res {
            Ok(env) => env,
            Err(dia) => {
                let msg = match dia {
                    Some(d) => d.to_string(),
                    None => String::from("failed to create environment"),
                };
                return Result::Err(
                    Err::new(0, &msg)
                );
            }
        };
        Result::Ok(env)
    }

    fn load_all_tables(&self) -> Result<Vec<Vec<String>>, DiagnosticRecord> {
        let mut tables: Vec<Vec<String>> = Vec::new();

        let con = self.con.as_ref().unwrap();
        let stmt = Statement::with_parent(con)?;

        let mut res = stmt.tables_opt_str(Option::None, Option::None, Option::None, "TABLE")?;
        let cols = res.num_result_cols()?;

        while let Some(mut cur) = res.fetch()? {
            let mut row: Vec<String> = Vec::new();

            for i in 1..(cols + 1) {
                let col_n = i as u16;
                
                if let Some(val) = cur.get_data::<&str>(col_n)? {
                    row.push(val.to_owned());
                } else {
                    row.push(String::from(""))
                }
            }
            tables.push(row);
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
            Err(dia) => {
                Result::Err(
                    Err::new(1, &dia.to_string())
                )
            }
        }
    }

    // Wrap load_all_tables() to allow easy use of '?'
    fn all_tables(&self) -> Result<Vec<Vec<String>>, Err> {
        
        match self.load_all_tables() {
            Ok(tables) => Result::Ok(tables),
            Err(dia) => {
                Result::Err(
                    Err::new(2, &dia.to_string())
                )
            }
        }
    }

    fn search(&mut self, term: &str) -> Result<Box<Vec<search::Res>>, Err> {
        if self.tables.is_none() {
            
            self.tables = match self.load_all_tables() {
                Ok(t) => Some(t),
                Err(dia) => {
                    return Result::Err(
                        Err::new(2, &dia.to_string())
                    );
                }
            };
        }
        Result::Ok(Box::new(Vec::new()))
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