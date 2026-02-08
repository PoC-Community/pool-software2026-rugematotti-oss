// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            counter: Mutex::new(0),
        })
        .invoke_handler(tauri::generate_handler![
            increment_counter,
            get_counter
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}




use std::sync::Mutex;
struct AppState {
    counter: Mutex<u32>,
}
#[tauri::command]
fn increment_counter(state: tauri::State<AppState>) -> Result<u32, String> {
    let mut counter = state.counter.lock().map_err(|_| "Mutex lock failed")?;
    *counter += 1;
    Ok(*counter)
}

#[tauri::command]
fn get_counter(state: tauri::State<AppState>) -> Result<u32, String> {
    let counter = state.counter.lock().map_err(|_| "Mutex lock failed")?;
    Ok(*counter)
}


fn greet(name: String) -> String {
    format!("Hello, {}! this is a rust mssg", name)
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Yo {}, welcome to Tauri!", name)
}