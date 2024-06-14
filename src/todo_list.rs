/* use crate::reminder::{self, schedule_reminder}; */
use chrono::{Local, NaiveDate, NaiveTime};
use colored::*;
/* use job_scheduler::JobScheduler; */
use prettytable::row;
use prettytable::{Cell, Row, Table};
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef};
use rusqlite::{params, ToSql};
use std::fmt::Debug;
use std::io::{self, Write};

use crate::database::Database;

#[derive(Debug, PartialEq)]
pub enum Priority {
    Baixa,
    Media,
    Alta,
}

pub struct TodoList {
    pub tasks: Vec<(String, NaiveDate, NaiveTime, Priority)>,
}

impl ToSql for Priority {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput> {
        match self {
            Priority::Baixa => Ok(ToSqlOutput::from("Baixa")),
            Priority::Media => Ok(ToSqlOutput::from("Média")),
            Priority::Alta => Ok(ToSqlOutput::from("Alta")),
        }
    }
}

impl FromSql for Priority {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value.as_str() {
            Ok("Baixa") => Ok(Priority::Baixa),
            Ok("Média") => Ok(Priority::Media),
            Ok("Alta") => Ok(Priority::Alta),
            Ok(_) => Err(FromSqlError::InvalidType),
            Err(_) => Err(FromSqlError::InvalidType),
        }
    }
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList { tasks: Vec::new() }
    }

    pub fn read_user_input(&self, prompt: &str) -> String {
        let mut input = String::new();
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        return input.trim().to_string();
    }

    pub fn add_task(
        &mut self,
        db: &Database,
        task: String,
        date: NaiveDate,
        time: NaiveTime,
        priority: &Priority,
    ) -> bool {
        let date_str = date.format("%Y-%m-%d").to_string();
        let time_str = time.format("%H:%M:%S").to_string();
        let id = uuid::Uuid::new_v4().to_string();

        let result = db.conn.execute(
            "INSERT INTO tasks (id, name, date, time, priority) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![id, task, date_str, time_str, priority],
        );

        match result {
            Ok(_) => true,
            Err(err) => {
                println!(
                    "error when inserting task into database: {}",
                    err
                );
                false
            }
        }
    }

    fn read_task_data(&mut self) -> NaiveDate {
        loop {
            let date_str = self.read_user_input("Enter the date of the task (12-30-1992 format): ");

            match NaiveDate::parse_from_str(&date_str, "%d-%m-%Y") {
                Ok(date) => {
                    let now = Local::now().date_naive();

                    if date < now {
                        println!("The task date cannot be later than the current date")
                    } else {
                        return date;
                    }
                }
                Err(_) => {
                    println!("Invalid Date. Please, try again");
                    continue;
                }
            }
        }
    }

    fn read_task_time(&mut self) -> NaiveTime {
        loop {
            let time_str = self.read_user_input("Digite a hora da tarefa (formato 14:30): ");

            match NaiveTime::parse_from_str(&time_str, "%H:%M") {
                Ok(time) => {
                    let now = Local::now().time();

                    if time < now {
                        println!("A data da tarefa não pode ser anterior a data atual")
                    } else {
                        return time;
                    }
                }
                Err(_) => {
                    println!("Horário inválido, por favor tente novamente");
                    continue;
                }
            }
        }
    }

    fn read_task_priority(&mut self) -> Priority {
        loop {
            let priority_str = self.read_user_input("Escolha a prioridade (Baixa, Media, Alta): ");
            match priority_str.to_lowercase().as_str() {
                "baixa" => return Priority::Baixa,
                "media" => return Priority::Media,
                "alta" => return Priority::Alta,
                _ => println!("Prioridade inválida. Por favor, tente novamente."),
            }
        }
    }

    //ESTÁ DANDO ERRO PORQUE EXISTEM VALORES DIFERENTES NO BANCO DE DADOS

    pub fn read_task(
        &mut self, /* sched: &mut JobScheduler */
    ) -> (String, NaiveDate, NaiveTime, Priority) {
        let task = self.read_user_input("Digite a sua tarefa: ");
        let date = self.read_task_data();
        let time = self.read_task_time();

        let priority = self.read_task_priority();
        (task, date, time, priority)
    }

    pub fn list_tasks(&self, db: &Database) {
        let mut tasks = db
            .conn
            .prepare("SELECT name, date, time, priority FROM tasks")
            .unwrap();
        let tasks_iter = tasks
            .query_map([], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, Priority>(3)?,
                ))
            })
            .unwrap();

        let mut table = Table::new();
        table.add_row(row!["#", "tarefa", "Data", "Hora", "Prioridade"]);
        for (i, task_row) in tasks_iter.enumerate() {
            let (task, date, time, priority) = task_row.unwrap();
            let priority_str = match priority {
                Priority::Baixa => "baixa".green(),
                Priority::Media => "media".yellow(),
                Priority::Alta => "alta".bright_red(),
            };

            table.add_row(Row::new(vec![
                Cell::new(&(i + 1).to_string()),
                Cell::new(&task),
                Cell::new(&date),
                Cell::new(&time),
                Cell::new(&priority_str.to_string()),
            ]));
        }
        table.printstd();
    }

    pub fn remove_task(&mut self, index: usize) {
        self.tasks.remove(index - 1);
    }
}
