// src/handlers/cluster_handler.rs
use crate::errors::AppError;
use crate::middleware::AuthenticatedUser;
use crate::models::cluster_model::NodeInfo;
use crate::models::user_model::UserRole; // Importe o enum UserRole
use crate::AppState;
use axum::{extract::State, Extension, Json};
use kube::{Api, Client, api::ListParams};
use k8s_openapi::api::core::v1::Node;
use std::sync::Arc;

pub async fn list_nodes(
    Extension(auth_user): Extension<AuthenticatedUser>, // Extrai o utilizador do middleware
    State(_state): State<Arc<AppState>>, // Renomeado para _state pois não o usamos (ainda)
) -> Result<Json<Vec<NodeInfo>>, AppError> {

    // --- PASSO DE AUTORIZAÇÃO (RBAC) ---
    if auth_user.role != UserRole::Admin {
        // Usa a variante 'Forbidden' que acabámos de definir
        return Err(AppError::Forbidden("Acesso negado. Requer privilégios de administrador.".to_string()));
    }

    // --- LÓGICA DE NEGÓCIO (Interação com o Cluster) ---
    let client = Client::try_default().await
        .map_err(|e| AppError::InternalServerError(format!("Falha ao conectar ao cluster K8s: {}", e)))?;

    let nodes_api: Api<Node> = Api::all(client);

    let node_list = nodes_api.list(&ListParams::default()).await
        .map_err(|e| AppError::InternalServerError(format!("Falha ao listar nós: {}", e)))?;

    // Mapeia a resposta complexa do K8s para a nossa struct simples 'NodeInfo'
    let node_infos: Vec<NodeInfo> = node_list.into_iter().map(|node| {
        let (status, role) = get_node_status_and_role(&node);
        
        // --- LÓGICA DE EXTRAÇÃO CORRIGIDA E SEGURA ---
        let (ip_address, kubelet_version) = node.status.as_ref().map_or(
            ("N/A".to_string(), "N/A".to_string()), // Valor padrão se 'status' for None
            |s| {
                let ip = s.addresses.as_ref()
                    .and_then(|a| a.iter().find(|addr| addr.type_ == "InternalIP"))
                    .map_or("N/A".to_string(), |addr| addr.address.clone());
                
                let version = s.node_info.as_ref()
                    .map_or("N/A".to_string(), |ni| ni.kubelet_version.clone()); // Corrigido
                
                (ip, version)
            }
        );
        // --- FIM DA CORREÇÃO ---

        NodeInfo {
            name: node.metadata.name.unwrap_or_default(),
            kubelet_version,
            ip_address,
            status,
            role,
        }
    }).collect();

    Ok(Json(node_infos))
}

// Função helper (sem alterações)
fn get_node_status_and_role(node: &Node) -> (String, String) {
    let status = node.status.as_ref()
        .and_then(|s| s.conditions.as_ref())
        .and_then(|conds| conds.iter().find(|c| c.type_ == "Ready"))
        .map_or("NotReady".to_string(), |c| c.status.clone());

    let role = node.metadata.labels.as_ref()
        .map_or("worker".to_string(), |labels| {
            if labels.contains_key("node-role.kubernetes.io/control-plane") {
                "control-plane".to_string()
            } else if labels.contains_key("node-role.kubernetes.io/master") {
                "control-plane".to_string()
            } else {
                "worker".to_string()
            }
        });

    (status, role)
}
