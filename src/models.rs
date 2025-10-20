use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::{fmt, fmt::Display, error::Error};


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

#[derive(Debug)]
pub enum TarefaError {
    FileIOError(String),
    DeserializeError(String),
    TarefaNaoEncontrada,
}

impl Display for TarefaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TarefaError::FileIOError(msg) => write!(f, "Erro ao tratar o arquivo: {}", msg),
            TarefaError::DeserializeError(msg) => write!(f, "Erro ao desserializar: {}", msg),
            TarefaError::TarefaNaoEncontrada => write!(f, "Tarefa não encontrada"),

        }
    }
}

impl Error for TarefaError {}

impl From<std::io::Error> for TarefaError {
    fn from(error: std::io::Error) -> Self {
        TarefaError::FileIOError(error.to_string())
    }
}

impl From<serde_json::Error> for TarefaError {
    fn from(error: serde_json::Error) -> Self {
        TarefaError::DeserializeError(error.to_string())
    }
}
