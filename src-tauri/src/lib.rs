// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn counter(counter: &str) -> String {
    match counter.parse::<i32>() {
        Ok(value) => format!("{}", value + 1),
        Err(_) => "Invalid number".to_string(),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, counter])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
