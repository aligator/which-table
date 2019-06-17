use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Dbms {
    pub driver_name: &'static str,
    pub odbc_table_pos: u8,
}

#[derive(Debug, PartialEq, Default)]
pub struct TableMeta<'a> {
    pub catalog: Option<&'a str>,
    pub schema: Option<&'a str>,
    pub table: Option<&'a str>,
    pub t_type: Option<&'a str>,
    pub remarks: Option<&'a str>,
}

impl Dbms {
    pub fn get(name: &str) -> Option<Dbms> {
        let systems = Self::load_all();
        
        return match systems.get(name) {
            Some(sys) => Some(sys.clone()),
            None => None,
        };
    }

    fn new(driver_name: &'static str, odbc_table_pos: u8) -> Dbms {
        Dbms {
            driver_name,
            odbc_table_pos,
        } 
    }

    fn load_all() -> HashMap<&'static str, Dbms> {
        let mut sys_map = HashMap::new();

        sys_map.insert("mariadb", Dbms::new("MariaDB", 2));
        sys_map.insert("mysql", Dbms::new("MySQL ODBC 8.0 ANSI Driver", 2));
        sys_map.insert("sqlite", Dbms::new("SQLite3", 1));

        sys_map
    }
}