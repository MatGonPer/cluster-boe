use axum::{extract::Multipart, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct ProjetoUpload {
    pub titulo: String,
    pub resumo: String,
    pub curso_id: i32,
    pub semestre_id: i32,
    pub github_url: Option<String>,
    pub integrantes_json: Vec<IntegranteUpload>,
    pub arquivo_bytes: Vec<u8>,
    pub capa_bytes: Option<Vec<u8>>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct IntegranteUpload {
    pub nome: String,
    pub matricula: String,
}

// Esta função é responsável pela captura dos dados do projeto que vem do Frontend
pub async fn upload_projeto(mut multipart: Multipart) -> impl IntoResponse {

    let mut projeto = ProjetoUpload::default();

    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap_or("").to_string();

        match name.as_str() {
            "titulo" => projeto.titulo = field.text().await.unwrap_or_default(),
            "resumo" => projeto.resumo = field.text().await.unwrap_or_default(),
            "curso_id" => {
                let texto = field.text().await.unwrap_or_default();
                projeto.curso_id = texto.parse().unwrap_or(0);
            },
            "semestre_id" => {
                let texto = field.text().await.unwrap_or_default();
                projeto.semestre_id = texto.parse().unwrap_or(0);
            },
            "github_url" => {
                let url = field.text().await.unwrap_or_default();
                if !url.trim().is_empty() {
                    projeto.github_url = Some(url);
                }
            },
            "integrantes" => {
                let texto_json = field.text().await.unwrap_or_default();
                if let Ok(lista) = serde_json::from_str::<Vec<IntegranteUpload>>(&texto_json) {
                    projeto.integrantes_json = lista;
                }
            },
            "arquivo_projeto" => {
                if let Ok(bytes) = field.bytes().await {
                    projeto.arquivo_bytes = bytes.to_vec();
                }
            },
            "capa" => {
                if let Ok(bytes) = field.bytes().await {
                    if !bytes.is_empty() {
                        projeto.capa_bytes = Some(bytes.to_vec());
                    }
                }
            },
            _ => {
                continue;
            }
        }
    }

    if projeto.titulo.trim().is_empty() {
        return (StatusCode::BAD_REQUEST, "Erro: O título é obrigatório.").into_response();
    }
    if projeto.resumo.trim().is_empty() {
        return (StatusCode::BAD_REQUEST, "Erro: O resumo é obrigatório.").into_response();
    }
    if projeto.curso_id == 0 || projeto.semestre_id == 0 {
        return (StatusCode::BAD_REQUEST, "Erro: Curso e Semestre são obrigatórios.").into_response();
    }
    if projeto.integrantes_json.is_empty() {
        return (StatusCode::BAD_REQUEST, "Erro: A lista de integrantes é obrigatória.").into_response();
    }
    if projeto.arquivo_bytes.is_empty() && projeto.github_url.is_none() {
        return (StatusCode::BAD_REQUEST, "Erro: Envie o arquivo do projeto ou o link do GitHub.").into_response();
    }

    (StatusCode::ACCEPTED, "Formulário validado e recebido na memória com sucesso!").into_response()
}
