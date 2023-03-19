// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::File;

use stegosaurusography::{
    decoder::Decoder,
    encoder::Encoder,
    error::Error,
    file_types::base_file::{BaseFile, FileProperties},
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    File::create("./wow.txt").expect("Failed to create file");
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// Used to do the encoding of the secret file into the base file. The results will be
/// stored in the encoded file.
#[tauri::command]
async fn encode(base_file: &str, secret_file: &str, encoded_file: &str) -> Result<(), Error> {
    println!("Encoding request received!");

    let mut encoder = Encoder::new(base_file.into(), secret_file.into(), encoded_file.into())?;
    encoder.encode()
}

/// Used to do the decoding of the encoded file. The results will be stored in the output file.
#[tauri::command]
async fn decode(encoded_file: &str, output_file: &str) -> Result<(), Error> {
    println!("Decoding request received!");

    let mut decoder = Decoder::new(encoded_file.into(), output_file.into())?;
    decoder.decode()
}

/// Used to get the properties of the base file. For example, how much data can be stored
/// secretly, as well as double checking that the file type is supported.
#[tauri::command]
async fn base_file_properties(base_file: &str) -> Result<FileProperties, Error> {
    println!("File property request received!");

    Ok(BaseFile::open(base_file.into())?.get_properties()?)
}

/// Gets the size of a file.
#[tauri::command]
async fn file_size(file: &str) -> Result<u64, Error> {
    println!("File size request received!");

    Ok(File::open(file)?.metadata()?.len())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            encode,
            decode,
            base_file_properties,
            file_size
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
