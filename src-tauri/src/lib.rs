use serde::{Deserialize, Serialize};
use std::fs::{create_dir, exists, read_dir, File};
use std::io::{Read, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct GameConfig {
    executable_path: String,
    preview_image: String,
    name: String,
    description: String,
    authors: Vec<String>,
    tags: Vec<String>,
}

#[tauri::command]
fn retrieve_games() -> String {
    read_all_games_config()
}

#[tauri::command]
fn retrieve_tags() -> String {
    read_tags_config()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    init();
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![retrieve_games, retrieve_tags])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub fn init() {
    let config_dir = "./AEMIGameLauncherConfig";
    let games_dir = format!("{}/Games", config_dir);
    let blank_file_path = format!("{}/G_blank.json", games_dir);
    let tag_config_file_path = format!("{}/tags.conf.json", config_dir);

    create_config_dir(config_dir);
    create_games_dir(&games_dir);
    create_game_json_template(&games_dir, &blank_file_path);
    create_tag_config_file(&tag_config_file_path);
}

fn create_game_json_template(games_dir: &str, blank_file_path: &str) {
    let is_games_dir_empty = read_dir(games_dir)
        .map(|mut dir| dir.next().is_none())
        .unwrap_or(false);

    if is_games_dir_empty {
        let default_json = r#"{"executable_path": "","preview_image": "","name": "","description": "","authors": [],"tags": ["blank"]}"#;
        let mut file = File::create(blank_file_path).expect("Failed to create G_blank.json");
        file.write_all(default_json.as_bytes())
            .expect("Failed to write to G_blank.json");
    }
}

fn create_tag_config_file(tag_config_file_path: &str) {
    let default_json = r#"[{"name":"blank", "color":"bg-red-800"}]"#;
    if ! exists(tag_config_file_path).expect("Failed to check config file") {
        let mut file = File::create(tag_config_file_path).expect("Failed to create tags.conf.json");
        file.write_all(default_json.as_bytes())
            .expect("Failed to write to tags.conf.json");
    }
}

fn create_games_dir(games_dir: &String) {
    if let Err(_) = create_dir(&games_dir) {
        if !Path::new(&games_dir).exists() {
            panic!("");
        }
    }
}

fn create_config_dir(config_dir: &str) {
    if let Err(_) = create_dir(config_dir) {
        if !Path::new(config_dir).exists() {
            panic!("Config folder does not exist and could not be created.");
        }
    }
}

fn read_tags_config() -> String {
    let path = "./AEMIGameLauncherConfig/tags.conf.json";
    if exists(path).expect("Failed to read tags.conf.json") {
        let mut file = match File::open(&path) {
            Ok(f) => f,
            Err(_) => panic!("Failed to open tags.conf.json"),
        };

        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            return contents
        }
    }
    panic!("tags.conf.json not found");
}

fn read_all_games_config() -> String {
    let games_dir = "./AEMIGameLauncherConfig/Games";
    let mut game_configs: Vec<GameConfig> = Vec::new();

    if let Ok(entries) = read_dir(games_dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_file() && path.extension().map(|ext| ext == "json").unwrap_or(false) {
                let mut file = match File::open(&path) {
                    Ok(f) => f,
                    Err(_) => continue,
                };

                let mut contents = String::new();
                if file.read_to_string(&mut contents).is_ok() {
                    if let Ok(config) = serde_json::from_str::<GameConfig>(&contents) {
                        game_configs.push(config);
                    }
                }
            }
        }
    }

    serde_json::to_string(&game_configs).unwrap_or_else(|_| "[]".to_string())
}
