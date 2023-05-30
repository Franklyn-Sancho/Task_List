/* use crate::reminder::{self, schedule_reminder}; */
use chrono::NaiveTime;
use colored::*;
/* use job_scheduler::JobScheduler; */
use prettytable::row;
use prettytable::{Cell, Row, Table};
use std::io::{self, Write};

#[derive(Debug)]
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

    /* pub fn set_reminder(&self, sched: &mut JobScheduler, index: usize, reminder_time: NaiveTime) {
        let task = self.tasks[index].0.clone();
        schedule_reminder(sched, task, reminder_time)
    } */

    pub fn add_task(&mut self, task: String, time: NaiveTime, priority: Priority) {
        self.tasks.push((task, time, priority));
        println!("Tarefa adicionada com sucesso")
    }

    pub fn read_task(&mut self, /* sched: &mut JobScheduler */) -> (String, NaiveTime, Priority) {
        let mut input = String::new();

        input.clear();
        print!("Digite a sua tarefa: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap(); //ler a entrada do usuário
        let task = input.trim().to_string();

        input.clear();
        print!("Digite a hora da tarefa (formato HH:MM): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let time = NaiveTime::parse_from_str(input.trim(), "%H:%M").unwrap();

        /* self.set_reminder(sched, self.tasks.len(), time); */

        input.clear();
        print!("Escolha a prioridade (Baixa, Media, Alta): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let priority = match input.trim().to_lowercase().as_str() {
            "baixa" => Priority::Baixa,
            "media" => Priority::Media,
            "alta" => Priority::Alta,
            _ => panic!("Prioridade Inválida"),
        };

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
