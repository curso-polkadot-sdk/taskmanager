use crate::io_utils::read_string;
use crate::models::{Prioridade, Tarefa, TarefaError};
use std::fs;
// use std::path::Path;

use chrono::{NaiveDate, Utc};

const FILE_PATH: &str = "tarefas.json";
pub struct TaskManager {
    pub tarefas: Vec<Tarefa>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            tarefas: Self::ler_tarefas(),
        }
    }

    pub fn adicionar_tarefa(&mut self) {
        let titulo = read_string("Título: ");
        let descricao = read_string("Descrição: ");
        let categoria = read_string("Categoria: ");
        let pri_str = read_string("Prioridade(Baixa, Média, Alta): ");
        let prioridade = match pri_str.to_lowercase().as_str() {
            "baixa" => Prioridade::Baixa,
            "média" => Prioridade::Media,
            "alta" => Prioridade::Alta,
            _ => Prioridade::Baixa,
        };
        let data_str = read_string("Vencimento: (DD-MM-AAAA): ");
        let data = match NaiveDate::parse_from_str(&data_str, "%d-%m-%Y") {
            Ok(dt) => dt,
            Err(_) => Utc::now().date_naive(),
        };
        let tarefa = Tarefa::new(titulo, descricao, categoria, data, prioridade);
        println!("Tarefa {} criada com sucesso", tarefa.titulo);

        self.tarefas.push(tarefa);
        println!("Tarefa adicionada na lista")
    }

    pub fn listar_tarefas(&self) {
        if self.tarefas.is_empty() {
            println!("Lista vazia");
            return;
        }

        for tarefa in &self.tarefas {
            println!("{}", tarefa);
        }
    }

    pub fn concluir_tarefa(&mut self) {
        let titulo = read_string("Título da tarefa a ser concluída: ");
        match self.tarefas.iter_mut().find(|t| t.titulo == titulo) {
            Some(tarefa) => {
                tarefa.finalizar();
                println!("Tarefa {} completada!", tarefa.titulo);
                return;
            },
            None => {
                println!("Tarefa não encontrada");
                return;
            }        
        }
    }

    pub fn ler_tarefas() -> Vec<Tarefa> {
        // Ler os dados do arquivo
        let data = match fs::read_to_string(FILE_PATH) {
            Ok(data) => data,
            Err(_) => {
                println!("Primeira vez, criando arquivo de tarefas...");
                return Vec::new();
            }
        };

        // Converter de JSON para o vetor de tarefas (desserializar) from_str
        serde_json::from_str(&data).expect("Erro ao desserializar")

    }

    pub fn salvar_tarefas(&self) -> Result<(), TarefaError>{
        // Converter para JSON (serializar o vetor de tarefas) to_string_pretty
        let json = serde_json::to_string_pretty(&self.tarefas)?;
        // Salvar no arquivo write do std::fs
        fs::write(FILE_PATH, json)?;
        // Mostrar mensagem
        println!("Tarefas salvas com sucesso em {}!", FILE_PATH);
        Ok(())
    }

    // Criar um erro próprio, ok_or e and_then
    // pub fn atualizar_prazo
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Status;

    #[test]
    fn test_concluir_tarefa() {
        let mut manager = TaskManager::new();
        let size = manager.tarefas.len();
        manager.tarefas.push(Tarefa::new(
            "Test".to_string(),
            "Desc".to_string(),
            "Cat".to_string(),
            Utc::now().date_naive(),
            Prioridade::Baixa,
        ));
        manager.concluir_tarefa();
        assert_eq!(manager.tarefas[size].status, Status::Concluida { data_conclusao: Utc::now().date_naive() });
    }
}