use database::database::Database;
use interfaces::cli::menu::menu;


mod utils;
mod interfaces;
mod reminder;
mod todo_list_tests;
mod database;


fn main() {
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
