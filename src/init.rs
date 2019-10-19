
extern crate rusqlite;
use rusqlite::{Connection, Result};
use std::path::PathBuf;


pub fn initialize_package_database(root_path: &PathBuf) -> Result<()> {
    info!("WOOHOO");
    let conn = Connection::open(root_path.as_path().join(".ppmdb.sqlite"))?;
    conn.execute_batch("
        PRAGMA foreign_keys = ON;
        PRAGMA encoding = 'UTF-8';
        BEGIN TRANSACTION;
        CREATE TABLE IF NOT EXISTS packages (
             id INTEGER PRIMARY KEY AUTOINCREMENT,
             name TEXT NOT NULL UNIQUE,
             version TEXT NOT NULL
             );
        CREATE TABLE IF NOT EXISTS requirements (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            package_id INTEGER NOT NULL,
            FOREIGN KEY (package_id) REFERENCES packages(id)
            );
        CREATE TABLE IF NOT EXISTS alternatives (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            requirement_id INTEGER NOT NULL,
            dep_status BOOLEAN NOT NULL,
            dep_name TEXT NOT NULL UNIQUE,
            dep_vers_rel TEXT CHECK( dep_vers_rel IN ('<','<=', '==', '!=', '>=', '>', '<>', '=>', '><') ) NOT NULL,
            dep_vers_spec TEXT NOT NULL UNIQUE,
            FOREIGN KEY (requirement_id) REFERENCES requirements(id)
            );
        CREATE TABLE IF NOT EXISTS files (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            path TEXT NOT NULL UNIQUE,
            package_id INTEGER NOT NULL,
            FOREIGN KEY (package_id) REFERENCES packages(id)
            );
        COMMIT;")?;
    Ok(())
}
