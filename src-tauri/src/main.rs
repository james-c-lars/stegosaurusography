// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod requests {
    use std::fs::File;
    use stegosaurusography::{get_properties, Decoder, Encoder, Error, FileProperties};

    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    #[tauri::command]
    pub fn greet(name: &str) -> String {
        log::info!("Greet request received!");
        log::trace!("Greet Request > name={name}");

        File::create("./wow.txt").expect("Failed to create file");
        format!("Hello, {}! You've been greeted from Rust!", name)
    }

    /// Used to do the encoding of the secret file into the base file. The results will be
    /// stored in the encoded file.
    #[tauri::command]
    pub async fn encode(
        base_file: &str,
        secret_file: &str,
        encoded_file: &str,
    ) -> Result<(), Error> {
        log::info!("Encoding request received!");
        log::trace!("Encode Request > base_file={base_file}, secret_file={secret_file}, encoded_file={encoded_file}");

        Encoder::new(base_file, secret_file, encoded_file)
            .and_then(|mut encoder| encoder.encode())
            .map_err(|err| {
                log::error!("{err:?}");
                err
            })
    }

    /// Used to do the decoding of the encoded file. The results will be stored in the output file.
    #[tauri::command]
    pub async fn decode(encoded_file: &str, output_file: &str) -> Result<(), Error> {
        log::info!("Decoding request received!");
        log::trace!("Decode Request > encoded_file={encoded_file}, output_file={output_file}");

        Decoder::new(encoded_file, output_file)
            .and_then(|mut decoder| decoder.decode())
            .map_err(|err| {
                log::error!("{err:?}");
                err
            })
    }

    /// Used to get the properties of the base file. For example, how much data can be stored
    /// secretly, as well as double checking that the file type is supported.
    #[tauri::command]
    pub async fn base_file_properties(base_file: &str) -> Result<FileProperties, Error> {
        log::info!("File property request received!");
        log::trace!("File Property Request > base_file={base_file}");

        get_properties(base_file).map_err(|err| {
            log::error!("{err:?}");
            err
        })
    }

    /// Gets the size of a file.
    #[tauri::command]
    pub async fn file_size(file: &str) -> Result<u64, Error> {
        log::info!("File size request received!");
        log::trace!("File Size Request > file={file}");

        File::open(file)
            .and_then(|file| file.metadata())
            .map(|metadata| metadata.len())
            .map_err(|err| {
                log::error!("{err:?}");
                err.into()
            })
    }
}

fn main() {
    // Set up logging
    let config_str = include_str!("./log_config.yaml");
    let config = serde_yaml::from_str(config_str).expect("log_config.yaml improperly structured");
    log4rs::init_raw_config(config).expect("logging config couldn't initialize");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            requests::greet,
            requests::encode,
            requests::decode,
            requests::base_file_properties,
            requests::file_size
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
