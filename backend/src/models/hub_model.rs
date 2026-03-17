use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Curso {
    pub id: i32,
    pub nome: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Semestre {
    pub id: i32,
    pub ano: i16,
    pub periodo: i16,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Tecnologia {
    pub id: i32,
    pub nome: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Projeto {
    pub id: i32,
    pub titulo: String,
    pub tema: String,
    pub resumo: String,
    pub github_url: String,
    
    pub arquivo_path: Option<String>,
    pub readme_conteudo: Option<String>,
    
    pub curso_id: i32,
    pub semestre_id: i32,
    
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct SubmeterProjetoPayload {
    #[validate(length(min = 5, message = "O título deve ter pelo menos 5 caracteres"))]
    pub titulo: String,
    
    #[validate(length(min = 5, message = "O tema deve ter pelo menos 5 caracteres"))]
    pub tema: String,
    
    #[validate(length(min = 20, message = "Forneça um resumo mais detalhado (min 20 caracteres)"))]
    pub resumo: String,
    
    #[validate(url(message = "Forneça uma URL válida do GitHub"))]
    pub github_url: String,
    
    pub curso_id: i32,
    pub semestre_id: i32,
    
    pub tecnologias_ids: Vec<i32>,
    pub alunos_ids: Vec<i32>,
}
