use std::{collections::HashMap, path::PathBuf};
use crate::{core::{console::ConsoleController, file_io::FileManager}, data::config::{Config, Filetype, General}};

#[allow(dead_code)]
pub fn init(base_path: PathBuf) {
    let console = ConsoleController::new();
    let mut file_manager = FileManager::new(base_path);
    
    // .boj 폴더 생성
    if let Err(e) = file_manager.create_dir(".boj") {
        console.error(format!("An error occurred while creating the .boj folder: {e}"));
        return;
    }

    // .boj 폴더로 이동
    if let Err(e) = file_manager.move_dir(".boj") {
        console.error(format!("An error occurred while moving to the .boj folder: {e}"));
        return;
    }

    let general = General { 
        selenium_browser: "Chrome".to_string(), 
        default_filetype: Some("py".to_string()), 
        editor_command: "code".to_string() 
    };

    
    let python = Filetype {
        language: "python".to_string(),
        main: "main.py".to_string(),
        run: "python main.py".to_string(),
        source_templates: Some(vec!["template.py".to_string()]),
        root_templates: None,
        compile: None,
        after: None,
    };

    let cpp = Filetype {
        language: "c++".to_string(),
        main: "main.cpp".to_string(),
        run: "./main.out".to_string(),
        source_templates: Some(vec!["template.cpp".to_string()]),
        root_templates: Some(vec!["template.cpp".to_string()]),
        compile: Some("g++ -std=c++17 -O2 -Wall main.cpp".to_string()),
        after: Some("rm a.out".to_string()),
    };

    let mut filetype: HashMap<String, Filetype> = HashMap::new();
    filetype.insert("python".to_string(), python);
    filetype.insert("cpp".to_string(), cpp);
    

    let config = Config {
        general,
        filetype,
    };

    let config_data = serde_json::to_string(&config);

    if let Err(e) = file_manager.write("config.json", &config_data.unwrap().to_string()) {
        console.error(format!("An error occurred while creating the config.json file: {e}"));
        return;
    }

    console.info("Initialized.");
}
