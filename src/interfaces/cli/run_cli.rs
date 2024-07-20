

use super::{database_cli::DatabaseCli, menu::menu};



/* pub fn run_cli() {
    match create_pool_from_env() {
        Ok(pool) => {
            match DatabaseCli::new(pool) {
                Ok(mut db) => {
                    if let Err(e) = db.create_tables() {
                        eprintln!("Error creating tables: {}", e);
                        return;
                    }

                    menu(&mut db);
                }
                Err(e) => {
                    eprintln!("Error initializing DatabaseSync: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Error creating database pool: {}", e);
        }
    }
} */
