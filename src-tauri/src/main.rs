// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dotenv::dotenv;
use std::fs;

use serde_json::Value;
use serde_json::json;
use serde_json::Map;

fn main() {
    dotenv().ok();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_all_files_frontmatter])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn get_all_files_frontmatter() -> Result<String, String> {
    let dir_path = std::env::var("DIR_PATH").expect("DIR_PATH not found in .env file");
    let mut frontmatters = vec![];
    let paths = fs::read_dir(dir_path).map_err(|e| e.to_string())?;
    
    let mut count = 0; // Counter for the number of files processed

    for path in paths {
        if count >= 5 {
            break; // Exit the loop if we have processed 5 files
        }

        let path = path.map_err(|e| e.to_string())?.path();
        if path.is_file() {
            let file = fs::read_to_string(&path).map_err(|e| e.to_string())?;
            match extract_frontmatter(&file) {
                Ok(frontmatter) => {
                    frontmatters.push(frontmatter);
                    count += 1; // Increment the counter
                },
                Err(e) => {
                    return Err(format!(
                        "Error parsing frontmatter in file {}: {}",
                        path.display(),
                        e
                    ))
                }
            }
        }
    }

    serde_json::to_string(&frontmatters).map_err(|e| e.to_string())
}

fn extract_frontmatter(file: &str) -> Result<serde_json::Value, serde_json::Error> {
    let start_delimiter = "---";
    let end_delimiter = "---";
    let start_index = file.find(start_delimiter).unwrap() + start_delimiter.len();
    let end_index = file[start_index..].find(start_delimiter).map(|i| start_index + i).unwrap_or_else(|| file.len());
    let frontmatter_str = &file[start_index..end_index].trim();
    
    // Split the frontmatter into lines
    let lines: Vec<&str> = frontmatter_str.split('\n').filter(|s|!s.trim().is_empty()).collect();

    println!("Lines:");
    println!("{:?}", lines);
    
    // Construct a JSON object from the frontmatter lines
    let mut json_object = serde_json::Map::new();
    for line in lines {
        let parts: Vec<&str> = line.splitn(2, ':').map(|s| s.trim()).collect();
        if parts.len() == 2 {
            let key = parts[0];
            let value = parts[1].trim();
            // Directly insert the value into the JSON object without wrapping it in serde_json::Value::String
            json_object.insert(key.to_string(), json!(value));
        }
    }


    println!("{:?}", json_object);

    // Convert JSON object to serde_json::Value
    let result = serde_json::Value::Object(json_object);

    println!("{:?}", result);

    Ok(result)
}








