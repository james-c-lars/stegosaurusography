// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod requests {
    use std::fs::File;
    use stegosaurusography::{
        get_properties, secret_context, Decoder, Encoder, FileProperties, Result,
    };

    /// Used to do the encoding of the secret file into the base file. The results will be
    /// stored in the output file.
    #[tauri::command]
    pub async fn encode(base_file: &str, secret_file: &str, output_file: &str) -> Result<()> {
        log::info!("Encoding request received!");
        log::trace!("Encode Request > base_file={base_file}, secret_file={secret_file}, output_file={output_file}");

        Encoder::new(base_file, secret_file, output_file)
            .and_then(|mut encoder| encoder.encode())
            .map(|_| log::info!("Completed the encoding request!"))
    }

    /// Used to do the decoding of the encoded file. The results will be stored in the output file.
    #[tauri::command]
    pub async fn decode(encoded_file: &str, output_file: &str) -> Result<()> {
        log::info!("Decoding request received!");
        log::trace!("Decode Request > encoded_file={encoded_file}, output_file={output_file}");

        Decoder::new(encoded_file, output_file)
            .and_then(|mut decoder| decoder.decode())
            .map(|_| log::info!("Completed the decoding request!"))
    }

    /// Used to get the properties of the base file. For example, how much data can be stored
    /// secretly, as well as double checking that the file type is supported.
    #[tauri::command]
    pub async fn base_file_properties(base_file: &str) -> Result<FileProperties> {
        log::info!("File property request received!");
        log::trace!("File Property Request > base_file={base_file}");

        get_properties(base_file).map(|props| {
            log::info!("File properties are {props:?}");
            props
        })
    }

    /// Gets the size of a file.
    ///
    /// Useful when paired with base_file_properties as it allows us to see whether a given secret file
    /// is small enough to fit within a base file.
    #[tauri::command]
    pub async fn file_size(file: &str) -> Result<u64> {
        log::info!("File size request received!");
        log::trace!("File Size Request > file={file}");

        secret_context!(File::open(file).and_then(|file| file.metadata())).map(|metadata| {
            let file_size = metadata.len();
            log::info!("File size is {file_size}");
            file_size
        })
    }

    // TODO: Add a command to allow the front-end to send messages to be logged for user interactions
}

fn main() {
    // Set up logging
    let config_str = include_str!("./log_config.yaml");
    let config = serde_yml::from_str(config_str).expect("log_config.yaml improperly structured");
    log4rs::init_raw_config(config).expect("logging config couldn't initialize");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            requests::encode,
            requests::decode,
            requests::base_file_properties,
            requests::file_size
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
