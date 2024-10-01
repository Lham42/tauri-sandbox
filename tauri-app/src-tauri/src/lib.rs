use tauri::Manager;
use std::path::PathBuf;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust! It is great to meet you!", name)
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let resource_path = app.path().resource_dir()
                .expect("failed to get resource dir")
                .join("binaries")
                .join("run");
            
            if resource_path.exists() {
                app.manage(RunCommand(resource_path.clone()));
                println!("Run command registered at: {:?}", resource_path);
            } else {
                println!("Run command not found at: {:?}", resource_path);
            }
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

struct RunCommand(PathBuf); 