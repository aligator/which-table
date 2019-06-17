use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Dbms {
    name: &'static str,
    odbc_table_pos: u8,
}

impl Dbms {
    pub fn get(name: &str) -> Option<Dbms> {
        let systems = Self::load_all();
        None
    }

    fn new(name: &'static str, odbc_table_pos: u8) -> Dbms {
        Dbms {
            name,
            odbc_table_pos,
        }
    }

    fn load_all() -> HashMap<&'static str, Dbms> {
        let mut sys_map = HashMap::new();

        sys_map.insert("mariadb", Dbms::new("", 0));
        sys_map.insert("mysql", Dbms::new("", 0));
        sys_map.insert("mssql", Dbms::new("", 0));
        sys_map.insert("sqlite", Dbms::new("", 0));

        sys_map
    }
}