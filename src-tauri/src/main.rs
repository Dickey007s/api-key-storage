#![windows_subsystem = "windows"]

mod commands;
mod models;
mod secrets;
mod storage;

use commands::*;
use secrets::SecretManager;
use storage::StorageManager;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            let storage = StorageManager::new(&app.handle())?;
            let secrets = SecretManager::new(&app.handle())?;
            
            app.manage(storage);
            app.manage(secrets);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            list_providers,
            get_provider,
            create_provider,
            update_provider,
            delete_provider,
            get_api_key,
            copy_to_clipboard
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
