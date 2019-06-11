#[derive(Debug, Clone)]
pub struct Res<'a> {
    table: &'a str,
    occurences: Vec<String>,
}