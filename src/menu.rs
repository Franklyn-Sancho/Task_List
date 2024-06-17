use std::io::{self, Write};

use crate::{database::Database, display_menu, read_input_user::{self, read_user_input}, task_list::Task};

pub fn menu(db: &Database) {
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
                let task_details = Task::read_task(); // Supondo que read_task seja uma função que retorna os detalhes da tarefa
                let (task, date, time, priority) = task_details;
                let success = Task::create_task(&db, task, date, time, priority);

                if success {
                    println!("Tarefa criada com sucesso!");
                } else {
                    println!("Falha ao criar tarefa.");
                }
            }
            2 => Task::list_tasks(&db),
            3 => {
                /* let index = loop {
                    let index_str = todo_list.read_user_input("Digite o número da tarefa: ");
                    match index_str.parse::<usize>() {
                        Ok(index) => break index,
                        Err(_) => println!("Invalid Index. Please, try again."),
                    }
                };
                let (task, date, time, priority) = todo_list.read_task();
                todo_list.tasks[index - 1] = (task, date, time, priority); */
            }
            4 => {
                let name = read_input_user::read_user_input("digite o nome da tarefa: ");
                Database::remove_task(&db, &name);
            }
            5 => break,
            _ => continue,
        }
    }
}
