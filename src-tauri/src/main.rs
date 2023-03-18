// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::File;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    File::create("./wow.txt").expect("Failed to create file");
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn encode(base_file: &str, secret_file: &str, encoded_file: &str) -> Result<(), ()> {
    println!("Encoding request received!");
    println!("Base: {}", base_file);
    println!("Secret File: {}", secret_file);
    println!("Encoded File: {}", encoded_file);

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, encode])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
