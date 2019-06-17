use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Dbms {
    odbc_table_pos: u8,
}

impl Dbms {
    pub fn get(name: &str) -> Option<Dbms> {
        let systems = Self::load_all();
        
        return match systems.get(name) {
            Some(sys) => Some(sys.clone()),
            None => None,
        };
    }

    fn new(odbc_table_pos: u8) -> Dbms {
        Dbms {
            odbc_table_pos,
        }
    }

    fn load_all() -> HashMap<&'static str, Dbms> {
        let mut sys_map = HashMap::new();

        sys_map.insert("mariadb", Dbms::new(0));
        sys_map.insert("mysql", Dbms::new(3));
        sys_map.insert("mssql", Dbms::new(0));
        sys_map.insert("sqlite", Dbms::new(0));

        sys_map
    }
}