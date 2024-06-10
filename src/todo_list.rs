/* use crate::reminder::{self, schedule_reminder}; */
use chrono::{Local, NaiveDate, NaiveTime};
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
    pub tasks: Vec<(String, NaiveDate, NaiveTime, Priority)>,
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

    pub fn add_task(
        &mut self,
        task: String,
        date: NaiveDate,
        time: NaiveTime,
        priority: Priority,
    ) -> bool {
        self.tasks.push((task, date, time, priority));
        true
    }

    /* fn read_task_time(&mut self) -> NaiveTime {
        loop {
            let time_str = self.read_user_input("Digite a hora da tarefa (formato HH:MM): ");

            match NaiveTime::parse_from_str(&time_str, "%H:%M") {
                Ok(time) => {
                    // Obter a hora atual sem data
                    let now = Local::now().time();

                    if time < now {
                        println!("O horário da tarefa não pode ser anterior ao horário atual.");
                    } else {
                        return time;
                    }
                }
                Err(_) => println!("Hora inválida. Por favor, tente novamente."),
            }
        }
    }

    fn read_task_date(&mut self) -> NaiveDate {
        loop {
            let date_str = self.read_user_input("Digite a data da tarefa (formato 30-12-1992): ");
            match NaiveDate::parse_from_str(&date_str, "%d-%m-%Y") {
                Ok(date) => {
                    let now = Local::now().date_naive();

                    if date < now {
                        println!("A data da teareja não pode ser anterior a data atual");
                    } else {
                        return date;
                    }
                }
                Err(_) => println!("Data inválida. Por favor, tenta novamente"),
            }
        }
    } */

    /* fn read_task_datetime(&mut self) -> NaiveDateTime {
        loop {
            let date_str = self.read_user_input("Digite a data da tarefa (formato 30-12-1992): ");

            match NaiveDate::parse_from_str(&date_str, "%d-%m-%Y") {
                Ok(date) => {
                    let time_str =
                        self.read_user_input("Digite a hora da tarefa (formato 14:30): ");

                    match NaiveTime::parse_from_str(&time_str, "%H:%M") {
                        Ok(time) => {
                            let now = Local::now().naive_local();
                            let datetime = NaiveDateTime::new(date, time);

                            if datetime < now {
                                println!("A data da tarefa não pode ser anterior à data atual.");
                            } else {
                                return datetime;
                            }
                        }
                        Err(_) => println!("Hora inválida. Por favor, tente novamente."),
                    }
                }
                Err(_) => {
                    println!("Data inválida. Por favor, tente novamente.");
                    continue; // Retorna ao início do loop para solicitar nova data
                }
            }
        }
    } */

    fn read_task_data(&mut self) -> NaiveDate {
        loop {
            let date_str = self.read_user_input("Digite a data da tarefa (formato 30-12-1992): ");

            match NaiveDate::parse_from_str(&date_str, "%d-%m-%Y") {
                Ok(date) => {
                    let now = Local::now().date_naive();

                    if date < now {
                        println!("A data da tarefa não pode ser enterior a data atual")
                    } else {
                        return date;
                    }
                }
                Err(_) => {
                    println!("Data inválida. Por favor, tente novamente.");
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

    pub fn read_task(
        &mut self, /* sched: &mut JobScheduler */
    ) -> (String, NaiveDate, NaiveTime, Priority) {
        let task = self.read_user_input("Digite a sua tarefa: ");
        let date = self.read_task_data();
        let time = self.read_task_time();

        let priority = self.read_task_priority();
        (task, date, time, priority)
    }

    pub fn list_tasks(&self) {
        let mut table = Table::new();
        table.add_row(row!["#", "tarefa", "Data", "Hora", "Prioridade"]);
        for (i, (task, date, time, priority)) in self.tasks.iter().enumerate() {
            let priority_str = match priority {
                Priority::Baixa => "baixa".green(),
                Priority::Media => "media".yellow(),
                Priority::Alta => "alta".bright_red(),
            };

            table.add_row(Row::new(vec![
                Cell::new(&(i + 1).to_string()),
                Cell::new(task),
                Cell::new(&date.format("%d-%m-%Y").to_string()),
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
