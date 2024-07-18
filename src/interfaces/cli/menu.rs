use std::io::{self, Write};

use colored::Colorize;
use prettytable::{row, Table};

use crate::interfaces::cli::task_list::Task;

use super::database_cli::DatabaseCli;


/* use crate::{database::Database, task_list::Task}; */

fn display_menu() {
    let mut table = Table::new();
    table.set_titles(row!["Task List Menu: ".green().bold()]);
    table.add_row(row!["1. Add Task".blue()]);
    table.add_row(row!["2. List Tasks".blue()]);
    table.add_row(row!["3. Complete Task ".blue()]);
    table.add_row(row!["4. Update Tasks ".blue()]);
    table.add_row(row!["4. Remove Tasks".blue()]);
    table.add_row(row!["5. Exit".bright_red()]);
    table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.printstd();
}

pub fn menu(db: &mut DatabaseCli) {
    let mut input = String::new();

    loop {
        input.clear();
        display_menu();
        print!("Escolha sua opção: ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).unwrap();
        let choice: u32 = input.trim().parse().unwrap();

        match choice {
            1 => Task::create_and_insert_task(db), // Chamada corrigida
            2 => Task::list_tasks(db),
            3 => Task::complete_task(db),
            4 => Task::update_task_interactive(db),
            5 => Task::remove_task(db),
            6 => break,
            _ => continue,
        }
    }
}

