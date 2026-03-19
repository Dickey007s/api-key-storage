use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use tauri::Manager;

#[derive(Debug, Default, Serialize, Deserialize)]
struct SecretsData {
    secrets: HashMap<String, String>,
}

pub struct SecretManager {
    secrets_file: PathBuf,
    data: Mutex<SecretsData>,
}

impl SecretManager {
    pub fn new(app: &tauri::AppHandle) -> Result<Self, String> {
        let data_dir = app
            .path()
            .app_data_dir()
            .map_err(|e| format!("获取密钥目录失败: {}", e))?;

        fs::create_dir_all(&data_dir).map_err(|e| format!("创建密钥目录失败: {}", e))?;

        let secrets_file = data_dir.join("secrets.json");
        let data = if secrets_file.exists() {
            let content = fs::read_to_string(&secrets_file)
                .map_err(|e| format!("读取密钥文件失败: {}", e))?;
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            SecretsData::default()
        };

        Ok(Self {
            secrets_file,
            data: Mutex::new(data),
        })
    }

    fn save(&self) -> Result<(), String> {
        let data = self.data.lock().map_err(|e| e.to_string())?;
        let content =
            serde_json::to_string(&*data).map_err(|e| format!("序列化密钥失败: {}", e))?;
        fs::write(&self.secrets_file, content).map_err(|e| format!("写入密钥文件失败: {}", e))
    }

    pub fn store(&self, id: &str, key: &str) -> Result<(), String> {
        let mut data = self.data.lock().map_err(|e| e.to_string())?;
        data.secrets.insert(id.to_string(), key.to_string());
        drop(data);
        self.save()
    }

    pub fn get(&self, id: &str) -> Result<String, String> {
        let data = self.data.lock().map_err(|e| e.to_string())?;
        Ok(data.secrets.get(id).cloned().unwrap_or_default())
    }

    pub fn delete(&self, id: &str) -> Result<(), String> {
        let mut data = self.data.lock().map_err(|e| e.to_string())?;
        data.secrets.remove(id);
        drop(data);
        self.save()
    }
}
