use std::fs::{create_dir, exists};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    init();
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub fn init(){
    match create_dir("./AEMIGameLauncherConfig") {
        Ok(_) => {println!("Config directory was successfully created");}
        Err(_) => {}
    }
    match exists("./AEMIGameLauncherConfig") {
        Ok(_) => {}
        Err(_) => {panic!("Config folder do not exist or is unreadable");}
    }
}
