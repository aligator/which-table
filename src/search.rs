use crate::info;

#[derive(Debug, Clone)]
pub struct Res<'a> {
    table: &'a str,
    occurences: &'a info::TableMeta,
}