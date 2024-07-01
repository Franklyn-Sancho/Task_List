use colored::*;
use database::Database;
use menu::menu;
/* use job_scheduler::{Job, JobScheduler}; */
use prettytable::row;
use prettytable::Table;
use task_list::Task;

mod database;
mod menu;
mod read_input_user;
mod reminder;
mod task_list;
mod todo_list_tests;

fn display_menu() {
    let mut table = Table::new();
    table.set_titles(row!["Task List Menu: ".green().bold()]);
    table.add_row(row!["1. Add Task".blue()]);
    table.add_row(row!["2. List Tasks".blue()]);
    table.add_row(row!["3. Update Tasks ".blue()]);
    table.add_row(row!["4. Remove Tasks".blue()]);
    table.add_row(row!["5. Exit".bright_red()]);
    table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.printstd();
}

fn main() {
    let db = Database::new("tasks.db");

    db.create_tables();

    /* menu(&db); */
    menu(&db);

    Task::remove_old_task(&db);
}

/* fn menu(db: &Database) {
    let mut todo_list = TodoList::new();
    let mut input = String::new();
    /* let mut sched = JobScheduler::new(); */

    loop {
        input.clear();
        display_menu();
        print!("Choice your option: ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).unwrap();
        let choice: u32 = input.trim().parse().unwrap();

        match choice {
            1 => {
                let (task, date, time, priority) = todo_list.read_task();
                if todo_list.add_task(&db, task, date, time, &priority) {
                    println!("Task Added Successfully");
                } else {
                    println!("Error Adding Task");
                }
            }
            2 => todo_list.list_tasks(&db),
            3 => {
                let index = loop {
                    let index_str = todo_list.read_user_input("Digite o número da tarefa: ");
                    match index_str.parse::<usize>() {
                        Ok(index) => break index,
                        Err(_) => println!("Invalid Index. Please, try again."),
                    }
                };
                let (task, date, time, priority) = todo_list.read_task();
                todo_list.tasks[index - 1] = (task, date, time, priority);
            }
            4 => {
                let name = todo_list.read_user_input("digite o nome da tarefa: ");
                todo_list.remove_task(db, &name);
                /*  input.clear();
                print!("Digite o numero da tarefa: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();
                todo_list.remove_task(index - 1); */
            }
            5 => break,
            _ => continue,
        }
    }
} */
