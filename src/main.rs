use chrono::NaiveTime;
use colored::*;
use prettytable::row;
use prettytable::{Cell, Row, Table};
use std::io::{self, Write};

struct TodoList {
    tasks: Vec<(String, NaiveTime)>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { tasks: Vec::new() }
    }

    fn add_task(&mut self, task: String, time: NaiveTime) {
        self.tasks.push((task, time));
        println!("Tarefa adicionada com sucesso")
    }

    fn list_tasks(&self) {
        let mut table = Table::new();
        table.add_row(row!["#", "tarefa", "Hora"]);
        for (i, (task, time)) in self.tasks.iter().enumerate() {
            table.add_row(Row::new(vec![
                Cell::new(&(i + 1).to_string()),
                Cell::new(task),
                Cell::new(&time.format("%H:%M").to_string()),
            ]));
        }
        table.printstd();
    }

    fn remove_task(&mut self, index: usize) {
        self.tasks.remove(index - 1);
    }
}

fn main() {
    let mut todo_list = TodoList::new();
    let mut input = String::new();

    loop {
        input.clear();
        let mut table = Table::new();
        table.set_titles(row!["Menu da da aplicação: ".green().bold()]);
        table.add_row(row!["1. Adicionar tarefa".blue()]);
        table.add_row(row!["2. Listar tarefas".blue()]);
        table.add_row(row!["3. Remover tarefa".blue()]);
        table.add_row(row!["4. Sair".bright_red()]);
        table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        table.printstd();
        print!("Escolha uma opção: ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).unwrap();
        let choice: u32 = input.trim().parse().unwrap();

        match choice {
            1 => {
                input.clear();
                print!("Digite a sua tarefa: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();
                let task = input.trim().to_string();

                input.clear();
                print!("Digite a hora da tarefa (formato HH:MM): ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();
                let time = NaiveTime::parse_from_str(input.trim(), "%H:%M").unwrap();

                todo_list.add_task(task, time);
            }
            2 => todo_list.list_tasks(),
            3 => {
                input.clear();
                print!("Digite o numero da tarefa: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();
                let index: usize = input.trim().parse().unwrap();
                todo_list.remove_task(index - 1);
            }
            4 => break,
            _ => continue,
        }
    }
}
