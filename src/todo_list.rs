/* use crate::reminder::{self, schedule_reminder}; */
use chrono::NaiveTime;
use colored::*;
/* use job_scheduler::JobScheduler; */
use prettytable::row;
use prettytable::{Cell, Row, Table};
use std::io::{self, Write};

#[derive(Debug, PartialEq)]
pub enum Priority {
    Baixa,
    Media,
    Alta,
}

pub struct TodoList {
    pub tasks: Vec<(String, NaiveTime, Priority)>,
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

    /* pub fn set_reminder(&self, sched: &mut JobScheduler, index: usize, reminder_time: NaiveTime) {
        let task = self.tasks[index].0.clone();
        schedule_reminder(sched, task, reminder_time)
    } */

    pub fn add_task(&mut self, task: String, time: NaiveTime, priority: Priority) -> bool {
        self.tasks.push((task, time, priority));
        true
    }

    fn read_task_time(&mut self) -> NaiveTime {
        loop {
            let time_str = self.read_user_input("Digite a hora da tarefa (formato HH:MM): ");
            match NaiveTime::parse_from_str(&time_str, "%H:%M") {
                Ok(time) => return time,
                Err(_) => println!("Hora inválida. Por favor, tente novamente."),
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

    pub fn read_task(&mut self, /* sched: &mut JobScheduler */) -> (String, NaiveTime, Priority) {
        let task = self.read_user_input("Digite a sua tarefa: ");
        let time = self.read_task_time();
        let priority = self.read_task_priority();
        (task, time, priority)
    }

    pub fn list_tasks(&self) {
        let mut table = Table::new();
        table.add_row(row!["#", "tarefa", "Hora", "Prioridade"]);
        for (i, (task, time, priority)) in self.tasks.iter().enumerate() {
            let priority_str = match priority {
                Priority::Baixa => "baixa".green(),
                Priority::Media => "media".yellow(),
                Priority::Alta => "alta".bright_red(),
            };

            table.add_row(Row::new(vec![
                Cell::new(&(i + 1).to_string()),
                Cell::new(task),
                Cell::new(&time.format("%H:%M").to_string()),
                Cell::new(&priority_str.to_string()),
            ]));
        }
        table.printstd();
    }

    pub fn remove_task(&mut self, index: usize) {
        self.tasks.remove(index - 1);
    }
}
