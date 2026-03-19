use tauri::State;
use crate::models::{Provider, CreateProviderRequest, UpdateProviderRequest};
use crate::storage::StorageManager;
use crate::secrets::SecretManager;

fn validate_url(url: &str) -> Result<(), String> {
    let url = url.trim();
    if url.is_empty() {
        return Err("Base URL 不能为空".into());
    }
    let parsed = url::Url::parse(url).map_err(|_| "Base URL 格式无效".to_string())?;
    if !["http", "https"].contains(&parsed.scheme()) {
        return Err("Base URL 必须以 http:// 或 https:// 开头".into());
    }
    Ok(())
}

#[tauri::command]
pub fn list_providers(storage: State<'_, StorageManager>) -> Result<Vec<Provider>, String> {
    storage.list()
}

#[tauri::command]
pub fn get_provider(id: String, storage: State<'_, StorageManager>) -> Result<Provider, String> {
    storage.get(&id)
}

#[tauri::command]
pub fn create_provider(
    request: CreateProviderRequest,
    storage: State<'_, StorageManager>,
    secrets: State<'_, SecretManager>,
) -> Result<Provider, String> {
    if request.name.trim().is_empty() {
        return Err("平台名称不能为空".into());
    }
    validate_url(&request.base_url)?;
    if request.api_key.trim().is_empty() {
        return Err("API Key 不能为空".into());
    }

    let remark = request.remark.as_ref()
        .map(|r| r.trim().to_string())
        .filter(|r| !r.is_empty());
    
    let provider = Provider::new(
        request.name.trim().into(),
        request.base_url.trim().into(),
        remark,
    );

    secrets.store(&provider.id, &request.api_key)?;
    let provider_id = provider.id.clone();
    storage.create(provider).map_err(|e| {
        let _ = secrets.delete(&provider_id);
        e
    })
}

#[tauri::command]
pub fn update_provider(
    request: UpdateProviderRequest,
    storage: State<'_, StorageManager>,
    secrets: State<'_, SecretManager>,
) -> Result<Provider, String> {
    if request.name.trim().is_empty() {
        return Err("平台名称不能为空".into());
    }
    validate_url(&request.base_url)?;

    let mut provider = storage.get(&request.id)?;
    provider.name = request.name.trim().into();
    provider.base_url = request.base_url.trim().into();
    provider.remark = request.remark.as_ref()
        .map(|r| r.trim().to_string())
        .filter(|r| !r.is_empty());
    provider.touch();

    if let Some(ref key) = request.api_key {
        if !key.trim().is_empty() {
            secrets.store(&provider.id, key)?;
        }
    }

    storage.update(provider)
}

#[tauri::command]
pub fn delete_provider(
    id: String,
    storage: State<'_, StorageManager>,
    secrets: State<'_, SecretManager>,
) -> Result<(), String> {
    storage.delete(&id)?;
    let _ = secrets.delete(&id);
    Ok(())
}

#[tauri::command]
pub fn get_api_key(id: String, secrets: State<'_, SecretManager>) -> Result<String, String> {
    secrets.get(&id)
}

#[tauri::command]
pub async fn copy_to_clipboard(text: String, app: tauri::AppHandle) -> Result<(), String> {
    use tauri_plugin_clipboard_manager::ClipboardExt;
    app.clipboard().write_text(&text).map_err(|e| e.to_string())
}