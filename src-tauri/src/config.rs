use serde::{Deserialize, Serialize}; // 确保导入 serde 库

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendConfig {
    pub api_endpoint: String,
    pub model: String,
}