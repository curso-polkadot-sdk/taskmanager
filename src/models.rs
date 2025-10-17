use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;


#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Tarefa {
    pub titulo: String,
    pub descricao: String,
    pub categoria: String,
    pub data: NaiveDate,
    pub prioridade: Prioridade,
    pub status: Status,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Prioridade {
    Alta,
    Media,
    Baixa,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Status {
    Pendente,
    Concluida { data_conclusao: NaiveDate },
}

impl Tarefa {
    pub fn new(
        titulo: String,
        descricao: String,
        categoria: String,
        data: NaiveDate,
        prioridade: Prioridade,
    ) -> Self {
        Self {
            titulo,
            descricao,
            categoria,
            data,
            prioridade,
            status: Status::Pendente,
        }
    }

    pub fn finalizar(&mut self) {
        self.status = Status::Concluida {
            data_conclusao: Utc::now().date_naive(),
        };
    }
}


impl fmt::Display for Tarefa {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let icone_status = match self.status {
            Status::Pendente => "[ ]",
            Status::Concluida { .. } => "[X]",
        };

        write!(
            f,
            "{} {} (Data Prevista: {})",
            icone_status,
            self.titulo,
            self.data.format("%d-%m-%Y")
        )
    }
}