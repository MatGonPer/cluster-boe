use serde:: Serialize;

#[derive(Serialize)]
pub struct NodeInfo {
    pub name: String,
    pub status: String,
    pub role: String,
    pub ip_address: String,
    pub kubelet_version: String,
}
