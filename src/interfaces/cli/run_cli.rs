


use crate::database::config::Config;

use super::{database_cli::DatabaseCli, menu::menu};



pub fn run_cli() {
    match Config::from_env() {
        Ok(config) => {
            config.print_pg_config();
            match config.create_pool() {
                Ok(pool) => {
                    match DatabaseCli::new(pool) {
                        Ok(db) => {
                            if let Err(e) = db.create_tables() {
                                eprintln!("Error creating tables: {}", e);
                                return;
                            }
                            // Chamar o menu CLI aqui
                            // menu(&mut db);
                        }
                        Err(e) => {
                            eprintln!("Error initializing DatabaseCli: {}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error creating database pool: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Error loading configuration: {}", e);
        }
    }
}
