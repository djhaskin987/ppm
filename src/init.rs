
extern crate rusqlite;
use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;
use std::path::PathBuf;

pub fn initialize_package_database(root_path: &PathBuf) -> Result<()> {
    let conn = Connection::open(root_path.as_path().join(".ppmdb.sqlite"))?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS packages (
            id integer not null primary key,
            name text not null unique,
            version text not null
            )",
            NO_PARAMS)?;
    Ok(())
}
