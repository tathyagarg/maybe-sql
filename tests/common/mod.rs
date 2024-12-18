use std::fs;
use std::path::Path;

pub fn ensure_db_existance(db_name: &str) -> bool {
    Path::new(db_name).exists()
}

pub fn ensure_table_existance(db_name: &str, table_name: &str) -> bool {
    Path::new(format!("{}/{}.asql", db_name, table_name).as_str()).exists()
}

pub fn remove_db(db_name: &str) -> Result<(), std::io::Error> {
    fs::remove_dir_all(db_name)
}

pub fn soft_remove_db(db_name: &str) -> Result<(), std::io::Error> {
    if !ensure_db_existance(db_name) {
        Ok(())
    } else {
        remove_db(db_name)
    }
}
