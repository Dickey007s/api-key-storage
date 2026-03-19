use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use serde_json;
use tauri::Manager;
use crate::models::{Provider, Storage};

pub struct StorageManager {
    providers_file: PathBuf,
    data: Mutex<Storage>,
}

impl StorageManager {
    pub fn new(app: &tauri::AppHandle) -> Result<Self, String> {
        let data_dir = app
            .path()
            .app_data_dir()
            .map_err(|e| format!("获取数据目录失败: {}", e))?;

        fs::create_dir_all(&data_dir)
            .map_err(|e| format!("创建数据目录失败: {}", e))?;

        let providers_file = data_dir.join("providers.json");

        let data = if providers_file.exists() {
            let content = fs::read_to_string(&providers_file)
                .map_err(|e| format!("读取存储文件失败: {}", e))?;
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Storage::default()
        };

        Ok(Self {
            providers_file,
            data: Mutex::new(data),
        })
    }

    fn save(&self) -> Result<(), String> {
        let data = self.data.lock().map_err(|e| e.to_string())?;
        let content = serde_json::to_string_pretty(&*data)
            .map_err(|e| format!("序列化失败: {}", e))?;
        fs::write(&self.providers_file, content)
            .map_err(|e| format!("写入文件失败: {}", e))
    }

    pub fn list(&self) -> Result<Vec<Provider>, String> {
        let data = self.data.lock().map_err(|e| e.to_string())?;
        Ok(data.providers.clone())
    }

    pub fn get(&self, id: &str) -> Result<Provider, String> {
        let data = self.data.lock().map_err(|e| e.to_string())?;
        data.providers
            .iter()
            .find(|p| p.id == id)
            .cloned()
            .ok_or_else(|| "平台不存在".to_string())
    }

    pub fn create(&self, provider: Provider) -> Result<Provider, String> {
        let mut data = self.data.lock().map_err(|e| e.to_string())?;
        data.providers.push(provider.clone());
        drop(data);
        self.save()?;
        Ok(provider)
    }

    pub fn update(&self, provider: Provider) -> Result<Provider, String> {
        let mut data = self.data.lock().map_err(|e| e.to_string())?;
        let index = data
            .providers
            .iter()
            .position(|p| p.id == provider.id)
            .ok_or_else(|| "平台不存在".to_string())?;
        data.providers[index] = provider.clone();
        drop(data);
        self.save()?;
        Ok(provider)
    }

    pub fn delete(&self, id: &str) -> Result<(), String> {
        let mut data = self.data.lock().map_err(|e| e.to_string())?;
        let index = data
            .providers
            .iter()
            .position(|p| p.id == id)
            .ok_or_else(|| "平台不存在".to_string())?;
        data.providers.remove(index);
        drop(data);
        self.save()
    }
}