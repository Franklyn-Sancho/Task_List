use std::env;

use interfaces::{app::server::run_web, cli::run_cli::run_cli};



mod utils;
mod interfaces;
mod reminder;
mod todo_list_tests;
mod database;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&"--web".to_string()) {
        // Iniciar a versão web da aplicação
        if let Err(e) = run_web() {
            eprintln!("Failed to start web server: {}", e);
        }
    } else {
        // Iniciar a versão CLI
        run_cli();
    }
}