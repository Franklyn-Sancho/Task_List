use chrono::{Local, NaiveDate, NaiveTime};
use colored::Colorize;
use prettytable::{row, Cell, Row, Table};

/* use crate::{
    database::Database,
    read_input_user::{self},
}; */

/* pub struct Task {
    pub id: String,
    pub task: String,
    pub date: NaiveDate,
    pub time: NaiveTime,
    pub priority: Priority,
    pub status: Status,
} */

/* #[derive(Debug, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Debug)]
pub enum Status {
    Pendent,
    Completed,
}

impl Task {
    pub fn new(
        task: String,
        date: NaiveDate,
        time: NaiveTime,
        priority: Priority,
        status: Status,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            task,
            date,
            time,
            priority,
            status,
        }
    }

    pub fn create_and_insert_task(db: &Database) {
        let task_details = Task::read_task();
        let (task, date, time, priority, status) = task_details;

        let new_task = Task::new(task, date, time, priority, status);

        if db.insert_task(&new_task).is_ok() {
            println!("Task was added successfully!");
        } else {
            println!("Failed to create task.");
        }
    }

    pub fn update_task_interactive(db: &Database) {
        let task =
            read_input_user::read_user_input("Digite o nome da tarefa que deseja atualizar: ");

        if let Err(_) = db.task_exists(&task) {
            println!("Tarefa não existe.");
            return;
        }

        let (new_task, date, time, priority, status) = Task::read_task();

        let updated_task = Task {
            id: uuid::Uuid::new_v4().to_string(),
            task: new_task,
            date,
            time,
            priority,
            status,
        };

        if db.update_task_database(&task, &updated_task).is_ok() {
            println!("Task was updated successfuly!");
        } else {
            println!("Failed to update task.");
        }
    }

    pub fn read_task_datetime() -> (NaiveDate, NaiveTime) {
        let date = Task::read_task_date();
        let time = Task::read_task_time(&date);
        (date, time)
    }

    fn read_task_date() -> NaiveDate {
        loop {
            let date_str = read_input_user::read_user_input(
                "Enter the date of the task (DD-MM-YYYY format): ",
            );
            if let Ok(date) = NaiveDate::parse_from_str(&date_str, "%d-%m-%Y") {
                let now = Local::now().date_naive();
                if date >= now {
                    return date;
                } else {
                    println!("The task date cannot be earlier than the current date.");
                }
            } else {
                println!("Invalid date format. Please enter date in DD-MM-YYYY format.");
            }
        }
    }

    fn read_task_time(date: &NaiveDate) -> NaiveTime {
        loop {
            let time_str =
                read_input_user::read_user_input("Enter the time of the task (HH:MM format): ");
            if let Ok(time) = NaiveTime::parse_from_str(&time_str, "%H:%M") {
                let now = Local::now();
                if *date == now.date_naive() && time < now.time() {
                    println!("The task time cannot be earlier than the current time for today. Please choose a time after the current time.");
                } else {
                    return time;
                }
            } else {
                println!("Invalid time format. Please enter time in HH:MM format.");
            }
        }
    }

    fn read_task_priority() -> Priority {
        loop {
            let priority_str = read_input_user::read_user_input(
                "Enter the priority of the task  (Low, Medium, High): ",
            );
            match priority_str.to_lowercase().as_str() {
                "low" => return Priority::Low,
                "medium" => return Priority::Medium,
                "high" => return Priority::High,
                _ => println!("Invalid Priority. Please, try again."),
            }
        }
    }

    /* fn read_task_status() -> Status {
        loop {
            let priority_str = read_input_user::read_user_input(
                "Enter the status of the task  (pending or completed): ",
            );
            match priority_str.to_lowercase().as_str() {
                "pending" => return Status::Pendent,
                "completed" => return Status::Completed,

                _ => println!("Invalid Priority. Please, try again."),
            }
        }
    } */

    pub fn read_task() -> (String, NaiveDate, NaiveTime, Priority, Status) {
        let task = read_input_user::read_user_input("Enter with your name task: ");
        let datetime = Task::read_task_datetime();
        let (date, time) = datetime;

        let priority = Task::read_task_priority();
        let status = Status::Pendent;
        (task, date, time, priority, status)
    }

    pub fn list_tasks(db: &Database) {
        let tasks = match db.get_tasks() {
            Ok(tasks) => tasks,
            Err(e) => {
                println!("Error fetching tasks: {}", e);
                return;
            }
        };

        let mut table = Table::new();
        table.add_row(row!["#", "Task", "Date", "Time", "Priority", "Status"]);
        for (i, (task, date, time, priority, status)) in tasks.iter().enumerate() {
            let priority_str = match priority {
                Priority::Low => "Low".bold().blue(),
                Priority::Medium => "Medium".bold().yellow(),
                Priority::High => "High".bold().bright_red(),
            };

            let status_str = match status {
                Status::Pendent => "Pendent".bold(),
                Status::Completed => "Completed".bold().green(),
            };

            table.add_row(Row::new(vec![
                Cell::new(&(i + 1).to_string()),
                Cell::new(task),
                Cell::new(date),
                Cell::new(time),
                Cell::new(&priority_str.to_string()),
                Cell::new(&status_str.to_string()),
            ]));
        }
        table.printstd();
    }

    pub fn remove_task(db: &Database) {
        loop {
            let task = read_input_user::read_user_input("digite o nome da tarefa: ");

            if !db.task_exists(&task).unwrap_or(false) {
                println!("Task does not exist.");
                return;
            }
            db.remove_task(&task);
        }
    }

    pub fn complete_task(db: &Database) {

        Task::list_tasks(db);

        loop {
            let task_complete =
                read_input_user::read_user_input("qual tarefa você deseja completar: ");

            if !db.task_exists(&task_complete).unwrap_or(false) {
                println!("Task does not exist.");
                return;
            }

            let _ = db.update_task_status(&task_complete);
        }
    }

    /* pub fn remove_old_task(db: &Database) {
        loop {
            let current_date = Local::now().naive_local().date();
            let current_time = Local::now().naive_local().time();
            db.remove_task_by_datetime(current_date, current_time);


            // Aguarde 1 hora antes de verificar novamente
            let _ = time::sleep(Duration::from_secs(3600));
        }
    } */
} */
