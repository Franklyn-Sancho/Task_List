use chrono::{Local, NaiveDate, NaiveTime};
use colored::Colorize;
use prettytable::{row, Cell, Row, Table};
use rusqlite::DatabaseName;

use crate::{
    database::{self, Database},
    read_input_user::{self, read_user_input},
};

pub struct Task {
    pub id: String,
    pub task: String,
    pub date: NaiveDate,
    pub time: NaiveTime,
    pub priority: Priority,
}

#[derive(Debug, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl Task {
    pub fn new(task: String, date: NaiveDate, time: NaiveTime, priority: Priority) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            task,
            date,
            time,
            priority,
        }
    }

    pub fn create_and_insert_task(db: &Database) {
        let task_details = Task::read_task();
        let (task, date, time, priority) = task_details;

        let new_task = Task::new(task, date, time, priority);

        if Database::insert_task(db, &new_task).is_ok() {
            println!("Tarefa criada com sucesso!");
        } else {
            println!("Falha ao criar tarefa.");
        }
    }

    pub fn update_task_interactive(db: &Database) {
        let task =
            read_input_user::read_user_input("Digite o nome da tarefa que deseja atualizar: ");

        if let Err(_) = Database::task_exists(db, &task) {
            println!("Tarefa não existe.");
            return;
        }

        let (new_task, date, time, priority) = Task::read_task();

        let updated_task = Task {
            id: uuid::Uuid::new_v4().to_string(),
            task: new_task,
            date,
            time,
            priority,
        };

        if Database::update_task_database(db, &task, &updated_task).is_ok() {
            println!("Tarefa atualizada com sucesso!");
        } else {
            println!("Falha ao atualizar a tarefa.");
        }
    }

    pub fn read_task_datetime() -> (NaiveDate, NaiveTime) {
        let date = Task::read_task_date();
        let time = Task::read_task_time(&date);
        (date, time)
    }
    
    fn read_task_date() -> NaiveDate {
        loop {
            let date_str = read_input_user::read_user_input("Enter the date of the task (DD-MM-YYYY format): ");
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
            let time_str = read_input_user::read_user_input("Enter the time of the task (HH:MM format): ");
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

    pub fn read_task() -> (String, NaiveDate, NaiveTime, Priority) {
        let task = read_input_user::read_user_input("Enter with your name task: ");
        let datetime = Task::read_task_datetime();
        let (date, time) = datetime;

        let priority = Task::read_task_priority();
        (task, date, time, priority)
    }

    pub fn list_tasks(db: &Database) {
        let tasks = match Database::get_tasks(&db) {
            Ok(tasks) => tasks,
            Err(e) => {
                println!("Error fetching tasks: {}", e);
                return;
            }
        };

        let mut table = Table::new();
        table.add_row(row!["#", "Task", "Date", "Time", "Priority"]);
        for (i, (task, date, time, priority)) in tasks.iter().enumerate() {
            let priority_str = match priority {
                Priority::Low => "low".green(),
                Priority::Medium => "medium".yellow(),
                Priority::High => "high".bright_red(),
            };

            table.add_row(Row::new(vec![
                Cell::new(&(i + 1).to_string()),
                Cell::new(task),
                Cell::new(date),
                Cell::new(time),
                Cell::new(&priority_str.to_string()),
            ]));
        }
        table.printstd();
    }

    pub fn remove_task(db: &Database) {
        loop {
            let task = read_input_user::read_user_input("digite o nome da tarefa: ");

            if !Database::task_exists(db, &task).unwrap_or(false) {
                println!("Task does not exist.");
                return;
            }
            Database::remove_task(&db, &task);
        }
    }
}
