use std::result::Result;

use odbc::*;

use crate::search;

trait Db {
    fn connect<'env>(&mut self, connString: &str) -> Result<(), Err>;
    fn all_tables(&self) -> &[String];
    fn search(&self, term: &str) -> Vec<search::Res>;
}

struct Odbc<'env> {
    environment: Environment<Version3>,
    connection: Connection<'env>,
}

impl<'env> Odbc<'env> {
    fn init(&mut self) -> Result<(), Err> {
        let envRes = create_environment_v3();

        match envRes {
            Ok(environment) => self.environment = environment,
            Err(diagnostics) => {
                // todo fn to create Err out of diagnostics
                let error = match diagnostics {
                    Some(diagnostics) => {
                        Err {
                            code: 0,
                            msg: diagnostics.to_string(),
                        }
                    }
                    None => {
                        Err {
                            code: 1,
                            msg: "odbc environment creation failed".to_string(),
                        }
                    }
                };

                return Result::Err(error);
            }
        }

        return Result::Ok(())
    }
}

impl<'env> Db for Odbc<'env> {
    fn connect(&mut self, connString: &str) -> Result<(), Err> {
        info!("odbc: connecting to {}", connString);
        let connRes = self.environment.connect_with_connection_string(connString);
        return match connRes {
            Ok(connection) => {
                self.connection = connection;
                Result::Ok(())
            },
            Err(diagnostics) => {
                // todo fn to create Err out of diagnostics
                return Result::Err(Err {
                    code: 0,
                    msg: diagnostics.to_string(),
                });
            }
        }
    }

    fn all_tables(&self) -> &[String] {
        unimplemented!()
    }

    fn search(&self, term: &str) -> Vec<search::Res> {
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
struct Err {
    code: u16,
    msg: String,
}

#[derive(Debug, Copy, Clone)]
struct Auth<'a> {
    user: &'a str,
    pass: &'a str,
}