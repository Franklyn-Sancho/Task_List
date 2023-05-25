use std::io::{self, Write};

struct TodoList {
    tasks: Vec<String>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { tasks: Vec::new() }
    }

    fn add_task(&mut self, task: String) {
        self.tasks.push(task)
    }

    fn list_tasks(&self) {
        for (i, task) in self.tasks.iter().enumerate() {
            println!("Essas são as suas listas: ");
            println!("{}: {}", i + 1, task)
        }
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
        println!("Menu da aplicação");
        println!("1. Adicionar tarefa");
        println!("2. Listar tarefas");
        println!("3. Remover tarefa");
        println!("4. Sair");
        print!("Escolha uma opção: ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).unwrap();
        let choice: u32 = input.trim().parse().unwrap();

        match choice {
            1 => {
                input.clear();
                print!("Digite a tarefa: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();
                todo_list.add_task(input.trim().to_string());
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
