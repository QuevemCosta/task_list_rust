use crate::Cli;
use crate::Commands;
use crate::services::task_service;
use rusqlite::Connection;

use crate::errors::task_errors::TaskError;
use crate::models::task::Task;

pub fn app(cli: Cli, conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    match &cli.command {
        Commands::Add { description } => {
            match task_service::create_task(&conn, description.to_string()) {
                Ok(()) => println!("Tarefa Adicionada com sucesso !!!"),

                Err(TaskError::EmptyDescription) => {
                    eprintln!("Erro: A descrição não pode ser vazia ou conter apenas espaços!!!")
                }

                Err(TaskError::MaxLenChars) => {
                    eprintln!("Erro: A descrição não pode ter mais de 149 caracteres !!!")
                }

                Err(_) => eprintln!("Erro ao criar a tarefa !!!"),
            }
        }
        Commands::List => match task_service::list_tasks(&conn) {
            Ok(tasks) => view_list(tasks),
            Err(_) => eprintln!("Erro na listagem das tarefas !!!"),
        },
        Commands::Check { id } => match task_service::toggle_task(&conn, id.to_string()) {
            Ok(()) => println!("Atualização realizada com sucesso!!!"),
            Err(TaskError::TaskNotFound) => {
                eprintln!("Erro: O ID informado não existe ou é invalido !!!")
            }
            Err(_) => eprintln!("Erro: Erro ao atualizar a tarefa !!!"),
        },
        Commands::Delete { id } => match task_service::delete_task(&conn, id.to_string()) {
            Ok(()) => println!("Tarefa apagada com sucesso !!!"),
            Err(_) => eprintln!("Erro ao apagar tarefa"),
        },
    }
    Ok(())
}

fn view_list(tasks: Vec<Task>) {
    if tasks.is_empty() {
        println!("\nNenhuma tarefa Cadastrada.\n");
        return;
    }

    println!();
    println!("┌────┬──────────┬────────────────────────────────────────────────────┐");
    println!("│ ID │ Status   │ Descrição                                          │");
    println!("├────┼──────────┼────────────────────────────────────────────────────┤");
    for task in tasks {
        let id = task.id.unwrap_or(0);
        let status = if task.completed { "[x]" } else { "[ ]" };
        let description = task.description;
        println!("│ {:>2} │ {:<8} │ {:<50} │", id, status, description);
    }
    println!("└────┴──────────┴────────────────────────────────────────────────────┘");
}
