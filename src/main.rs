// src/main.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, State, Window, Emitter};
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod native_features;

use native_features::NativeFeatures;

#[derive(Debug)]
pub struct AppState {
    pub download_manager: Arc<Mutex<DownloadManager>>,
    pub settings: Arc<Mutex<AppSettings>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadRequest {
    pub url: String,
    pub quality: String,
    pub output_dir: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DownloadProgress {
    pub id: String,
    pub progress: f32,
    pub status: String,
    pub file_path: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub download_dir: String,
    pub quality: String,
    pub auto_start: bool,
    pub minimize_to_tray: bool,
    pub global_hotkeys: bool,
    pub theme: String,
    pub volume: f32,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            download_dir: std::env::current_dir()
                .unwrap_or_default()
                .join("downloads")
                .to_string_lossy()
                .to_string(),
            quality: "high".to_string(),
            auto_start: false,
            minimize_to_tray: true,
            global_hotkeys: true,
            theme: "dark".to_string(),
            volume: 0.75,
        }
    }
}

#[derive(Debug)]
pub struct DownloadManager {
    downloads: Arc<Mutex<HashMap<String, DownloadProgress>>>,
}

impl DownloadManager {
    pub fn new() -> Self {
        Self {
            downloads: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn start_download(&mut self, request: DownloadRequest) -> Result<String, String> {
        let download_id = uuid::Uuid::new_v4().to_string();
        let progress = DownloadProgress {
            id: download_id.clone(),
            progress: 0.0,
            status: "downloading".to_string(),
            file_path: None,
            error: None,
        };

        self.downloads.lock().unwrap().insert(download_id.clone(), progress);

        // Start download in background
        let downloads = self.downloads.clone();
        let id = download_id.clone();
        tokio::spawn(async move {
            // Simulate download progress
            for i in 0..=100 {
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                if let Ok(mut downloads) = downloads.lock() {
                    if let Some(progress) = downloads.get_mut(&id) {
                        progress.progress = i as f32;
                        if i == 100 {
                            progress.status = "completed".to_string();
                            progress.file_path = Some(format!("/path/to/{}.mp3", id));
                        }
                    }
                }
            }
        });

        Ok(download_id)
    }

    pub fn get_progress(&self, download_id: &str) -> Option<DownloadProgress> {
        self.downloads.lock().unwrap().get(download_id).cloned()
    }

    pub fn get_history(&self) -> Vec<DownloadProgress> {
        self.downloads.lock().unwrap().values().cloned().collect()
    }
}

// Tauri commands
#[tauri::command]
async fn download_universal_url(
    request: DownloadRequest,
    state: State<'_, AppState>,
    window: Window,
) -> Result<String, String> {
    let mut manager = state.download_manager.lock().unwrap();
    
    // Start download process
    let download_id = manager.start_download(request.clone())?;
    
    // Emit progress updates to frontend
    let window_clone = window.clone();
    let download_id_clone = download_id.clone();
    let download_manager = state.download_manager.clone();
    
    tokio::spawn(async move {
        loop {
            if let Some(progress) = download_manager.lock().unwrap().get_progress(&download_id_clone) {
                let _ = window_clone.emit("download-progress", &progress);
                
                if progress.status == "completed" || progress.status == "failed" {
                    break;
                }
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }
    });
    
    Ok(download_id)
}

#[tauri::command]
async fn get_download_history(
    state: State<'_, AppState>
) -> Result<Vec<DownloadProgress>, String> {
    let manager = state.download_manager.lock().unwrap();
    Ok(manager.get_history())
}

#[tauri::command]
async fn get_app_settings(
    state: State<'_, AppState>
) -> Result<AppSettings, String> {
    let settings = state.settings.lock().unwrap();
    Ok(settings.clone())
}

#[tauri::command]
async fn update_app_settings(
    new_settings: AppSettings,
    state: State<'_, AppState>
) -> Result<(), String> {
    let mut settings = state.settings.lock().unwrap();
    *settings = new_settings;
    Ok(())
}

#[tauri::command]
async fn pause_download(
    _download_id: String,
    _state: State<'_, AppState>
) -> Result<(), String> {
    // Implementation for pausing downloads
    Ok(())
}

#[tauri::command]
async fn resume_download(
    _download_id: String,
    _state: State<'_, AppState>
) -> Result<(), String> {
    // Implementation for resuming downloads
    Ok(())
}

#[tauri::command]
async fn cancel_download(
    _download_id: String,
    _state: State<'_, AppState>
) -> Result<(), String> {
    // Implementation for canceling downloads
    Ok(())
}

#[tauri::command]
async fn clear_completed_downloads(
    _state: State<'_, AppState>
) -> Result<(), String> {
    // Implementation for clearing completed downloads
    Ok(())
}

#[tauri::command]
async fn show_notification(
    title: String,
    body: String,
) -> Result<(), String> {
    NativeFeatures::show_download_notification(&title, &body)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn open_in_file_manager(
    file_path: String,
) -> Result<(), String> {
    NativeFeatures::open_file_location(&file_path)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn set_auto_start(
    enabled: bool,
) -> Result<(), String> {
    NativeFeatures::set_auto_start(enabled)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_audio_devices() -> Result<Vec<AudioDevice>, String> {
    // Implementation for getting audio devices
    Ok(vec![
        AudioDevice { id: "default".to_string(), name: "Default Device".to_string() },
        AudioDevice { id: "speakers".to_string(), name: "Speakers".to_string() },
        AudioDevice { id: "headphones".to_string(), name: "Headphones".to_string() },
    ])
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AudioDevice {
    pub id: String,
    pub name: String,
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState {
            download_manager: Arc::new(Mutex::new(DownloadManager::new())),
            settings: Arc::new(Mutex::new(AppSettings::default())),
        })
        .invoke_handler(tauri::generate_handler![
            download_universal_url,
            get_download_history,
            get_app_settings,
            update_app_settings,
            pause_download,
            resume_download,
            cancel_download,
            clear_completed_downloads,
            show_notification,
            open_in_file_manager,
            set_auto_start,
            get_audio_devices
        ])
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            
            // Set up window configuration
            let _ = window.set_title("SwingMusic Desktop");
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
