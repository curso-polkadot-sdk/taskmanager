use chrono::{NaiveDate, Utc};

pub struct Tarefa {
    pub titulo: String,
    pub descricao: String,
    pub categoria: String,
    pub data: NaiveDate,
    pub prioridade: Prioridade,
    pub status: Status,
}

pub enum Prioridade {
    Alta,
    Media,
    Baixa,
}

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

    pub fn exibir(&self) -> String {
        let icone_status = match self.status {
            Status::Pendente => "[ ]",
            Status::Concluida { .. } => "[X]",
        };

        format!(
            "{} {} (Data Prevista: {}",
            icone_status,
            self.titulo,
            self.data.format("%d-%m-%Y")
        )
    }

    pub fn finalizar(&mut self) {
        self.status = Status::Concluida {
            data_conclusao: Utc::now().date_naive(),
        };
    }
}
