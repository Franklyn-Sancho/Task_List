use database::Database;
use menu::menu;


mod menu;
mod read_input_user;
mod reminder;
mod todo_list_tests;
mod database;
mod task_list_postgres;


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
