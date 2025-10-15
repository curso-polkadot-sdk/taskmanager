use crate::io_utils::read_string;
use crate::models::{Prioridade, Tarefa};

use chrono::{NaiveDate, Utc};

pub struct TaskManager {
    pub tarefas: Vec<Tarefa>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            tarefas: Self::ler_tarefas(),
        }
    }

    pub fn ler_tarefas() -> Vec<Tarefa> {
        todo!()
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
            println!("{}", tarefa.exibir());
        }
    }
}
