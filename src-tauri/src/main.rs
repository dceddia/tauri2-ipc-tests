// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Duration;
use tauri::Manager;

#[tauri::command]
fn get_blob(_request: tauri::ipc::Request<'_>) -> tauri::ipc::Response {
    let mut blob = vec![];
    for i in 0..(1920 * 1080 * 3) {
        blob.push(i as u8);
    }

    tauri::ipc::Response::new(blob)
}

fn main() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_blob])
        .build(tauri::generate_context!())
        .unwrap();

    app.run(|app, event| {
        match event {
            tauri::RunEvent::Ready => {
                // start sending events
                let app_clone = app.clone();

                std::thread::spawn(move || loop {
                    std::thread::sleep(Duration::from_millis(1000));
                    let mut blob = vec![];
                    for i in 0..(1920 * 1080 * 3) {
                        blob.push(i as u8);
                    }
                    let timer = std::time::Instant::now();
                    app_clone.emit("from_backend", &blob).unwrap();
                    println!("emit: sent {} bytes in {:?}", blob.len(), timer.elapsed());
                });
            }
            _ => {}
        }
    });
}
