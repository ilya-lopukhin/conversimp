// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ffmpeg_sidecar;
use std::env;

fn main() {

    match env::current_exe() {
        Ok(path) => println!("Current executable path: {}", path.display()),
        Err(e) => eprintln!("Failed to get executable path: {}", e),
    }

    tauri_app_lib::run();
    ffmpeg_sidecar::download::auto_download().unwrap();
}
