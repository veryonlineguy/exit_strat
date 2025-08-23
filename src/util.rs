use rusqlite::Result;
use std::fs;

pub const DATA_DIR: &str = "data";
pub const VAULT_DIR: &str = "/home/emma/Documents/life/Daily";

pub fn get_database_path() -> Result<String> {
    if !std::path::Path::new(DATA_DIR).exists() {
        fs::create_dir_all(DATA_DIR).map_err(|e| {
            rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_IOERR),
                Some(format!("Failed to create directory: {}", e)),
            )
        })?;
    }
    Ok(format!("{}/links.db", DATA_DIR))
}
