// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust! It is great to meet you!", name)
// }

// #[cfg_attr(mobile, tauri::mobile_entry_point)]
// pub fn run() {
//     tauri::Builder::default()
//         .plugin(tauri_plugin_shell::init())
//         .invoke_handler(tauri::generate_handler![greet])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }

// use tauri::Manager;
// use std::path::PathBuf;

// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

// // Struct to hold the path to the 'run' binary
// struct RunBinary(PathBuf);

// pub fn run() {
//     tauri::Builder::default()
//         .setup(|app| {
//             let resource_path = app.path_resolver()
//                 .resolve_resource("run")
//                 .expect("failed to resolve 'run' binary");
            
//             println!("'run' binary path: {:?}", resource_path);
            
//             app.manage(RunBinary(resource_path));
//             Ok(())
//         })
//         .invoke_handler(tauri::generate_handler![greet])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }
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