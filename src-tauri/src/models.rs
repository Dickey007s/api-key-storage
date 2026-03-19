use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provider {
    pub id: String,
    pub name: String,
    pub base_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl Provider {
    pub fn new(name: String, base_url: String, remark: Option<String>) -> Self {
        let now = Utc::now().to_rfc3339();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            base_url,
            remark,
            created_at: now.clone(),
            updated_at: now,
        }
    }
    
    pub fn touch(&mut self) {
        self.updated_at = Utc::now().to_rfc3339();
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateProviderRequest {
    pub name: String,
    pub base_url: String,
    pub api_key: String,
    #[serde(default)]
    pub remark: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProviderRequest {
    pub id: String,
    pub name: String,
    pub base_url: String,
    #[serde(default)]
    pub api_key: Option<String>,
    #[serde(default)]
    pub remark: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Storage {
    pub providers: Vec<Provider>,
}
