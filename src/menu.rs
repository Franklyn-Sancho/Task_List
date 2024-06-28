use std::io::{self, Write};

use crate::{database::Database, display_menu, task_list::Task};

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
            1 => Task::create_and_insert_task(&db),
            2 => Task::list_tasks(&db),
            3 => Task::update_task_interactive(&db),
            4 => Task::remove_task(&db),
            5 => break,
            _ => continue,
        }
    }
}
