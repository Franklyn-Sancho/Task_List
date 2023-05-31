use colored::*;
/* use job_scheduler::{Job, JobScheduler}; */
use prettytable::row;
use prettytable::Table;
use std::io::{self, Write};
use todo_list::TodoList;

mod reminder;
mod todo_list;
mod todo_list_tests;

fn display_menu() {
    let mut table = Table::new();
    table.set_titles(row!["Menu da da aplicação: ".green().bold()]);
    table.add_row(row!["1. Adicionar tarefa".blue()]);
    table.add_row(row!["2. Listar tarefas".blue()]);
    table.add_row(row!["3. Editar tarefa".blue()]);
    table.add_row(row!["4. Remover tarefa".blue()]);
    table.add_row(row!["5. Sair".bright_red()]);
    table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.printstd();
}

fn main() {
    let mut todo_list = TodoList::new();
    let mut input = String::new();
    /* let mut sched = JobScheduler::new(); */

    loop {
        input.clear();
        display_menu();
        print!("Escolha uma opção: ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).unwrap();
        let choice: u32 = input.trim().parse().unwrap();

        match choice {
            1 => {
                let (task, time, priority) = todo_list.read_task();
                if todo_list.add_task(task, time, priority) {
                    println!("tarefa adicionada com sucesso");
                } else {
                    println!("Erro ao adicionar tarefa");
                }
            }
            2 => todo_list.list_tasks(),
            3 => {
                let index = loop {
                    let index_str = todo_list.read_user_input("Digite o número da tarefa: ");
                    match index_str.parse::<usize>() {
                        Ok(index) => break index,
                        Err(_) => println!("Índice inválido. Por favor, tente novamente."),
                    }
                };
                let (task, time, priority) = todo_list.read_task();
                todo_list.tasks[index - 1] = (task, time, priority);
            }
            4 => {
                input.clear();
                print!("Digite o numero da tarefa: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();
                let index: usize = input.trim().parse().unwrap();
                todo_list.remove_task(index - 1);
            }
            5 => break,
            _ => continue,
        }
    }
}
