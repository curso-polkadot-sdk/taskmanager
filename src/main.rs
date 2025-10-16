// Gerenciador de tarefas
// Adicionar tarefas
// Listar tarefas
// Marcar como finalizada

use crate::task_manager::TaskManager;

mod io_utils;
mod models;
mod task_manager;

fn main() {
    let mut manager = TaskManager::new();
    loop {
        println!("Gerenciador de Tarefas");
        println!("======================");
        println!("1) Adicionar Tarefa");
        println!("2) Listar Tarefas");
        println!("3) Marcar Tarefa Finalizada");
        println!("4) Salvar e Sair");

        let opcao = io_utils::read_string("Escolha uma opção: ");

        match opcao.trim() {
            "1" => manager.adicionar_tarefa(),
            "2" => manager.listar_tarefas(),
            "3" => manager.concluir_tarefa(),
            "4" => {
                println!("Saindo...");
                break;
            }
            _ => println!("Opção inválida."),
        }
    }
}
