use crate::database::database::Database;

use super::menu::menu;


pub fn run_cli() {
    match Database::new() {
        Ok(mut db) => {
            if let Err(e) = db.create_tables() {
                eprintln!("Error creating tables: {}", e);
                return;
            }

            menu(&mut db);
        }
        Err(e) => {
            eprintln!("Error connecting to the database: {}", e);
        }
    }
}