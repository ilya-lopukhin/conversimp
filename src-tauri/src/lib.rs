use ffmpeg_sidecar::{
    command::FfmpegCommand,
    command::ffmpeg_is_installed,
    download::{download_ffmpeg_package, ffmpeg_download_url, unpack_ffmpeg},
    version::ffmpeg_version_with_path,
   // event::{FfmpegEvent, FfmpegProgress},
};
use std::env;
use std::thread;
use tauri::Emitter;
use tauri::Manager;
use std::fs;
use std::path::Path;

fn create_directory(path_str: &str) -> std::io::Result<&str> {
    let path = Path::new(path_str);
    fs::create_dir(&path)?; // Creates the directory and parent directories if needed
    Ok(path_str)
}

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
    path: String,
    success: bool,
    error: String,
}

fn _get_fmmpeg_version(app_handle: tauri::AppHandle) -> Result<String, Box<dyn std::error::Error>> {
    let path_res = app_handle.path();
    let app_data_dir = path_res.app_data_dir()?;
    let version = ffmpeg_version_with_path(app_data_dir.join("ffmpeg"))?;
    Ok(version)
}

#[tauri::command]
fn get_ffmpeg_version(app_handle: tauri::AppHandle) -> String {
    if ffmpeg_is_installed() {
        match _get_fmmpeg_version(app_handle) {
            Ok(version) => {
                return version
            },
            Err(_) => {
                return format!("")
            }
        }
    } else {
        return format!("")
    }
}

#[tauri::command]
fn download_install_ffmpeg(app_handle: tauri::AppHandle) -> () {
    thread::spawn(move || {
        match _download_install_ffmpeg(app_handle) {
            Ok(_) => {
                println!("_download_ffmpeg finished")
            },
            Err(_) => {
            }
        };
    });

    return ()
}

fn _download_install_ffmpeg(app_handle: tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let path_res = app_handle.path();
    let download_url = ffmpeg_download_url()?;
    let app_data_dir = path_res.app_data_dir()?;
    println!("Downloading from: {:?}", download_url);
    let archive_path = download_ffmpeg_package(download_url, &app_data_dir)?;
    println!("Downloaded package: {:?}", archive_path);
    // Extraction uses `tar` on all platforms (available in Windows since version 1803)
    println!("Extracting...");
    unpack_ffmpeg(&archive_path, &app_data_dir)?;
    let version = ffmpeg_version_with_path(app_data_dir.join("ffmpeg"))?;
    println!("FFmpeg version: {}", version);
    println!("Done! 🏁");
    app_handle.emit("ffmpeg-download-finished", {}).unwrap();
    Ok(())
}

#[tauri::command]
fn init_appdatadir(app_handle: tauri::AppHandle) -> () {
    let path_res = app_handle.path();

    match path_res.app_data_dir() {
        Ok(path) => {
            let path_str: String = format!("{}/", path.display());
            match create_directory(&path_str) {
                Ok(path) => {
                    println!("{} dir created", path);
                },
                Err(e) => {
                    println!("app data dir create failed: {}", e);
                }
            };
        },
        Err(e) => {
           println!("app data dir failed: {}", e)
        },
    }
}

#[tauri::command]
async fn use_ffmpeg(app_handle: tauri::AppHandle, paths: Vec<String>) -> () {
    let path_res = app_handle.path();

    match path_res.app_data_dir() {
        Ok(app_data) => {
            let path_str: String = format!("{}/ffmpeg", app_data.display());
            for path in paths {
                let app_handle_clone = app_handle.clone();
                let path_str_clone = path_str.clone();
                thread::spawn(move || {
                    let _split: Vec<&str> = path.split(".").collect();

                    let _path = path.clone();

                    let path_without_ext = _split[0];

                    let output = format!("{path_without_ext}.mp3");

                    let mut ffmpeg = FfmpegCommand::new_with_path(path_str_clone)
                        .input(_path)
                        .output(output)
                        .print_command()
                        .spawn()
                        .unwrap();

                    let result = ffmpeg.wait().expect("file {path} failed to process");

                    app_handle_clone
                        .emit(
                            "path-processed",
                            Payload {
                                path: path.into(),
                                success: result.success(),
                                error: "".into(),
                            },
                            )
                        .unwrap();
                });
            }
        },
        Err(e) => {
           println!("app data dir failed: {}", e)
        },
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_macos_permissions::init())
        .invoke_handler(tauri::generate_handler![use_ffmpeg, init_appdatadir, get_ffmpeg_version, download_install_ffmpeg])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
