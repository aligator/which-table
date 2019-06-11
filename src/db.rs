use std::result;
use crate::search;

trait Db {
    fn connect<'a>(&self, con: &'a str, db: &'a str, auth: Box<Auth>) -> result::Result<(), Err>;
    fn all_tables(&self) -> &[String];
    fn search<'x>(&self, term: &'x str) -> &[search::Res];
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