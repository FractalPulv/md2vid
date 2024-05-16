use std::fs;
use serde_json::{json, Map, Value};
use tauri::Window;

pub fn get_all_files_frontmatter() -> Result<String, String> {
    let dir_path = std::env::var("DIR_PATH").expect("DIR_PATH not found in .env file");
    let mut frontmatters = vec![];
    let paths = fs::read_dir(dir_path).map_err(|e| e.to_string())?;
    
    let mut count = 0; // Counter for the number of files processed

    for path in paths {
        if count >= 40 {
            break; // Exit the loop if we have processed 5 files
        }

        let path = path.map_err(|e| e.to_string())?.path();
        if path.is_file() {
            let file = fs::read_to_string(&path).map_err(|e| e.to_string())?;
            let filename = path.file_name().unwrap().to_string_lossy().to_string();
            match extract_frontmatter(&file, &filename, &path) {
                Ok(frontmatter) => {
                    println!("Frontmatter: {:?}", frontmatter);
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

fn extract_frontmatter(file: &str, filename: &str, path: &std::path::Path) -> Result<serde_json::Value, serde_json::Error> {
    let start_delimiter = "---";
    let end_delimiter = "---";
    let start_index = file.find(start_delimiter).unwrap() + start_delimiter.len();
    let end_index = file[start_index..].find(start_delimiter).map(|i| start_index + i).unwrap_or_else(|| file.len());
    let frontmatter_str = &file[start_index..end_index].trim();
    
    // Split the frontmatter into lines
    let lines: Vec<&str> = frontmatter_str.split('\n').filter(|s| !s.trim().is_empty()).collect();

    let mut frontmatter_map = Map::new();
    for line in lines {
        let mut parts = line.splitn(2, ':');
        let key = parts.next().unwrap().trim();
        let value = parts.next().unwrap().trim();
        
        // Parse the value properly and insert it into the JSON map
        let parsed_value = parse_value(value);
        frontmatter_map.insert(key.to_string(), parsed_value);
    }

    // Insert the filename into the JSON map
    frontmatter_map.insert("filename".to_string(), json!(filename));
    // Insert the FULL file path into the JSON map
    frontmatter_map.insert("filepath".to_string(), json!(path.to_str().unwrap()));

    Ok(Value::Object(frontmatter_map))
}

fn parse_value(value: &str) -> Value {
    // Try to parse as a boolean
    if let Ok(boolean) = value.parse::<bool>() {
        return json!(boolean);
    }
    // Try to parse as a number (integer or float)
    if let Ok(number) = value.parse::<f64>() {
        return json!(number);
    }
    // Default to string if nothing else matches
    json!(value)
}

// make a function which takes in a path and returns a Result
// the function should read the file, extract the frontmatter, print it to the console.
// it should also extract the text content of the file and return it as a string while also printing it to the console.
// important: in each text file there is dataviewjs code which is surrounded by triple backticks (```)
// only store and print the text AFTER the dataviewjs code (ie. the text after the last set of triple backticks)
// if there is no dataviewjs code in the file, just print the entire file content as the text content
pub async fn read_file_and_extract_frontmatter(path: &str) -> Result<String, String> {
    let file = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let filename = path.split('/').last().unwrap();
    let frontmatter = extract_frontmatter(&file, filename, std::path::Path::new(path)).map_err(|e| e.to_string())?;
    let text_content = extract_text_content(&file);
    println!("===============================");
    println!("Frontmatter: {:?}", frontmatter);
    println!("Text content: {}", text_content);
    println!("===============================");
    Ok(text_content)
}

pub fn extract_text_content(file: &str) -> String {
    let dataviewjs_delimiter = "```";
    let dataviewjs_start = file.rfind(dataviewjs_delimiter).map(|i| i + dataviewjs_delimiter.len()).unwrap_or(0);
    let text_content = &file[dataviewjs_start..].trim();
    text_content.to_string()
}