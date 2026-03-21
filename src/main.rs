#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::{DateTime, Utc};
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::{Emitter, Manager, State, Window};

mod native_features;

use native_features::NativeFeatures;

#[derive(Debug)]
pub struct AppState {
    pub download_manager: Arc<Mutex<DownloadManager>>,
    pub settings: Arc<Mutex<AppSettings>>,
    pub connection: Arc<Mutex<Option<WebAppSession>>>,
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
pub struct WebAppConnection {
    pub url: String,
    pub pairing_code: String,
    pub is_connected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionRequest {
    pub url: String,
    #[serde(default)]
    pub pairing_code: String,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub password: Option<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragonflyStats {
    pub connected: bool,
    pub latency_ms: f64,
    pub memory_used: String,
    pub memory_peak: String,
    pub total_keys: i64,
    pub uptime_seconds: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheService {
    pub name: String,
    pub key_count: i64,
    pub hit_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragonflyServices {
    pub services: Vec<CacheService>,
}

#[derive(Debug, Clone)]
pub struct WebAppSession {
    pub base_url: String,
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub username: String,
    pub connected_at: DateTime<Utc>,
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

    pub fn insert_or_update(&self, progress: DownloadProgress) {
        if let Ok(mut downloads) = self.downloads.lock() {
            downloads.insert(progress.id.clone(), progress);
        }
    }

    pub fn get_progress(&self, download_id: &str) -> Option<DownloadProgress> {
        self.downloads.lock().ok()?.get(download_id).cloned()
    }

    pub fn get_history(&self) -> Vec<DownloadProgress> {
        self.downloads
            .lock()
            .map(|map| map.values().cloned().collect())
            .unwrap_or_default()
    }

    pub fn update_status(&self, download_id: &str, status: &str) {
        if let Ok(mut downloads) = self.downloads.lock() {
            if let Some(item) = downloads.get_mut(download_id) {
                item.status = status.to_string();
            }
        }
    }

    pub fn remove(&self, download_id: &str) {
        if let Ok(mut downloads) = self.downloads.lock() {
            downloads.remove(download_id);
        }
    }

    pub fn clear_completed(&self) {
        if let Ok(mut downloads) = self.downloads.lock() {
            downloads.retain(|_, item| {
                item.status != "completed" && item.status != "failed" && item.status != "cancelled"
            });
        }
    }
}

fn normalize_base_url(raw: &str) -> Result<String, String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err("Server URL is required".to_string());
    }

    let parsed = Url::parse(trimmed)
        .map_err(|_| "Server URL must include http:// or https://".to_string())?;

    if parsed.scheme() != "http" && parsed.scheme() != "https" {
        return Err("Server URL must start with http:// or https://".to_string());
    }

    let mut normalized = parsed;
    normalized.set_path("");
    normalized.set_query(None);
    normalized.set_fragment(None);

    Ok(normalized.as_str().trim_end_matches('/').to_string())
}

fn build_http_client() -> Result<Client, String> {
    Client::builder()
        .timeout(Duration::from_secs(20))
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {e}"))
}

async fn fetch_pair_tokens(
    client: &Client,
    base_url: &str,
    pairing_code: &str,
) -> Result<(String, Option<String>), String> {
    let code = pairing_code
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .collect::<String>()
        .to_uppercase();
    if code.is_empty() {
        return Err("Pairing code is required".to_string());
    }

    let url = format!("{base_url}/auth/pair");
    let response = client
        .get(url)
        .query(&[("code", code.as_str())])
        .send()
        .await
        .map_err(|e| format!("Could not reach server: {e}"))?;

    let status = response.status();
    let payload = response
        .json::<Value>()
        .await
        .map_err(|e| format!("Invalid pairing response payload: {e}"))?;

    if !status.is_success() {
        let msg = payload
            .get("msg")
            .and_then(Value::as_str)
            .unwrap_or("Pairing failed");
        return Err(format!("{msg} (status {status})"));
    }

    let access = payload
        .get("accesstoken")
        .or_else(|| payload.get("access_token"))
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();

    if access.is_empty() {
        return Err("Pairing succeeded but no access token was returned".to_string());
    }

    let refresh = payload
        .get("refreshtoken")
        .or_else(|| payload.get("refresh_token"))
        .and_then(Value::as_str)
        .map(|s| s.to_string());

    Ok((access, refresh))
}

async fn fetch_login_tokens(
    client: &Client,
    base_url: &str,
    username: &str,
    password: &str,
) -> Result<(String, Option<String>), String> {
    let user = username.trim();
    if user.is_empty() {
        return Err("Username is required when pairing code is not provided".to_string());
    }
    if password.is_empty() {
        return Err("Password is required when pairing code is not provided".to_string());
    }

    let response = client
        .post(format!("{base_url}/auth/login"))
        .json(&serde_json::json!({
            "username": user,
            "password": password,
        }))
        .send()
        .await
        .map_err(|e| format!("Could not reach server: {e}"))?;

    let status = response.status();
    let payload = response
        .json::<Value>()
        .await
        .map_err(|e| format!("Invalid login response payload: {e}"))?;

    if !status.is_success() {
        let msg = payload
            .get("msg")
            .or_else(|| payload.get("error"))
            .and_then(Value::as_str)
            .unwrap_or("Login failed");
        return Err(format!("{msg} (status {status})"));
    }

    let access = payload
        .get("accesstoken")
        .or_else(|| payload.get("access_token"))
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();

    if access.is_empty() {
        return Err("Login succeeded but no access token was returned".to_string());
    }

    let refresh = payload
        .get("refreshtoken")
        .or_else(|| payload.get("refresh_token"))
        .and_then(Value::as_str)
        .map(|s| s.to_string());

    Ok((access, refresh))
}

async fn fetch_current_username(
    client: &Client,
    base_url: &str,
    access_token: &str,
) -> Result<String, String> {
    let url = format!("{base_url}/auth/user");
    let response = client
        .get(url)
        .bearer_auth(access_token)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch current user: {e}"))?;

    let status = response.status();
    let payload = response
        .json::<Value>()
        .await
        .map_err(|e| format!("Invalid current user payload: {e}"))?;

    if !status.is_success() {
        let msg = payload
            .get("msg")
            .and_then(Value::as_str)
            .unwrap_or("Failed to validate session");
        return Err(format!("{msg} (status {status})"));
    }

    let username = payload
        .get("username")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();

    if username.is_empty() {
        return Err("Connected but received no username from server".to_string());
    }

    Ok(username)
}

fn array_len(payload: &Value, key: &str) -> usize {
    payload
        .get(key)
        .and_then(Value::as_array)
        .map(|items| items.len())
        .unwrap_or(0)
}

fn response_error(payload: &Value, fallback: &str) -> String {
    payload
        .get("msg")
        .or_else(|| payload.get("error"))
        .and_then(Value::as_str)
        .unwrap_or(fallback)
        .to_string()
}

async fn collect_sync_items(
    client: &Client,
    session: &WebAppSession,
) -> Result<Vec<String>, String> {
    let recents_response = client
        .get(format!("{}/nothome/recents/added", session.base_url))
        .bearer_auth(&session.access_token)
        .query(&[("limit", 20)])
        .send()
        .await
        .map_err(|e| format!("Failed to fetch recent items: {e}"))?;
    let recents_status = recents_response.status();
    let recents = recents_response
        .json::<Value>()
        .await
        .map_err(|e| format!("Failed to parse recent items response: {e}"))?;
    if !recents_status.is_success() {
        return Err(format!(
            "{} (status {recents_status})",
            response_error(&recents, "Failed to fetch recent items")
        ));
    }

    let playlists_response = client
        .get(format!("{}/playlists", session.base_url))
        .bearer_auth(&session.access_token)
        .query(&[("no_images", true)])
        .send()
        .await
        .map_err(|e| format!("Failed to fetch playlists: {e}"))?;
    let playlists_status = playlists_response.status();
    let playlists = playlists_response
        .json::<Value>()
        .await
        .map_err(|e| format!("Failed to parse playlists response: {e}"))?;
    if !playlists_status.is_success() {
        return Err(format!(
            "{} (status {playlists_status})",
            response_error(&playlists, "Failed to fetch playlists")
        ));
    }

    let favorites_response = client
        .get(format!("{}/favorites", session.base_url))
        .bearer_auth(&session.access_token)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch favorites: {e}"))?;
    let favorites_status = favorites_response.status();
    let favorites = favorites_response
        .json::<Value>()
        .await
        .map_err(|e| format!("Failed to parse favorites response: {e}"))?;
    if !favorites_status.is_success() {
        return Err(format!(
            "{} (status {favorites_status})",
            response_error(&favorites, "Failed to fetch favorites")
        ));
    }

    let recent_count = array_len(&recents, "items");
    let playlist_count = array_len(&playlists, "data");
    let favorite_tracks = array_len(&favorites, "tracks");
    let favorite_albums = array_len(&favorites, "albums");
    let favorite_artists = array_len(&favorites, "artists");

    Ok(vec![
        format!("Connected as {}", session.username),
        format!("Recent library items synced: {recent_count}"),
        format!("Playlists synced: {playlist_count}"),
        format!(
            "Favorites synced: {} tracks, {} albums, {} artists",
            favorite_tracks, favorite_albums, favorite_artists
        ),
        format!(
            "Last sync: {}",
            session
                .connected_at
                .with_timezone(&chrono::Local)
                .format("%Y-%m-%d %H:%M:%S")
        ),
    ])
}

fn parse_download_progress(job: &Value) -> Option<DownloadProgress> {
    let id = job
        .get("id")
        .and_then(Value::as_i64)
        .map(|value| value.to_string())
        .or_else(|| job.get("id").and_then(Value::as_str).map(ToOwned::to_owned))?;

    let progress = job
        .get("progress")
        .and_then(Value::as_f64)
        .unwrap_or(0.0)
        .clamp(0.0, 100.0) as f32;

    let status = job
        .get("state")
        .or_else(|| job.get("status"))
        .and_then(Value::as_str)
        .unwrap_or("queued")
        .to_string();

    let file_path = job
        .get("target_path")
        .or_else(|| job.get("file_path"))
        .and_then(Value::as_str)
        .map(|value| value.to_string());

    let error = job
        .get("error")
        .or_else(|| job.get("message"))
        .and_then(Value::as_str)
        .map(|value| value.to_string());

    Some(DownloadProgress {
        id,
        progress,
        status,
        file_path,
        error,
    })
}

async fn create_remote_download_job(
    client: &Client,
    session: &WebAppSession,
    request: &DownloadRequest,
) -> Result<DownloadProgress, String> {
    let response = client
        .post(format!("{}/api/downloads/jobs", session.base_url))
        .bearer_auth(&session.access_token)
        .json(&serde_json::json!({
            "source_url": request.url,
            "source": "universal",
            "quality": request.quality,
            "codec": "mp3",
            "item_type": "track",
            "target_path": request.output_dir,
        }))
        .send()
        .await
        .map_err(|e| format!("Failed to create remote download job: {e}"))?;

    let status = response.status();
    let payload = response
        .json::<Value>()
        .await
        .map_err(|e| format!("Invalid create job response payload: {e}"))?;

    if !status.is_success() {
        let msg = response_error(&payload, "Failed to create download job");
        return Err(format!("{msg} (status {status})"));
    }

    if let Some(job) = payload.get("job").and_then(parse_download_progress) {
        return Ok(job);
    }

    let job_id = payload
        .get("job_id")
        .and_then(Value::as_i64)
        .map(|id| id.to_string())
        .or_else(|| {
            payload
                .get("job_id")
                .and_then(Value::as_str)
                .map(ToOwned::to_owned)
        })
        .ok_or_else(|| "Server created a job but did not return job id".to_string())?;

    Ok(DownloadProgress {
        id: job_id,
        progress: 0.0,
        status: "queued".to_string(),
        file_path: None,
        error: None,
    })
}

async fn fetch_remote_download_job(
    client: &Client,
    session: &WebAppSession,
    job_id: &str,
) -> Result<DownloadProgress, String> {
    let response = client
        .get(format!(
            "{}/api/downloads/jobs/{}",
            session.base_url, job_id
        ))
        .bearer_auth(&session.access_token)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch remote job status: {e}"))?;

    let status = response.status();
    let payload = response
        .json::<Value>()
        .await
        .map_err(|e| format!("Invalid job status response payload: {e}"))?;

    if !status.is_success() {
        let msg = response_error(&payload, "Failed to fetch job status");
        return Err(format!("{msg} (status {status})"));
    }

    parse_download_progress(&payload)
        .ok_or_else(|| "Job status payload is missing required fields".to_string())
}

async fn fetch_remote_download_history(
    client: &Client,
    session: &WebAppSession,
) -> Result<Vec<DownloadProgress>, String> {
    let response = client
        .get(format!("{}/api/downloads/jobs", session.base_url))
        .bearer_auth(&session.access_token)
        .query(&[("limit", 300)])
        .send()
        .await
        .map_err(|e| format!("Failed to fetch download history: {e}"))?;

    let status = response.status();
    let payload = response
        .json::<Value>()
        .await
        .map_err(|e| format!("Invalid download history payload: {e}"))?;

    if !status.is_success() {
        let msg = response_error(&payload, "Failed to fetch download history");
        return Err(format!("{msg} (status {status})"));
    }

    let mut rows = Vec::new();
    if let Some(items) = payload.get("jobs").and_then(Value::as_array) {
        for item in items {
            if let Some(progress) = parse_download_progress(item) {
                rows.push(progress);
            }
        }
    }

    Ok(rows)
}

async fn trigger_remote_job_action(
    client: &Client,
    session: &WebAppSession,
    job_id: &str,
    action: &str,
) -> Result<(), String> {
    let response = client
        .post(format!(
            "{}/api/downloads/jobs/{}/{}",
            session.base_url, job_id, action
        ))
        .bearer_auth(&session.access_token)
        .send()
        .await
        .map_err(|e| format!("Failed to trigger job action {action}: {e}"))?;

    let status = response.status();
    let payload = response
        .json::<Value>()
        .await
        .map_err(|e| format!("Invalid job action payload: {e}"))?;

    if !status.is_success() {
        let msg = response_error(&payload, "Download action failed");
        return Err(format!("{msg} (status {status})"));
    }

    if payload
        .get("success")
        .and_then(Value::as_bool)
        .unwrap_or(true)
    {
        Ok(())
    } else {
        Err(response_error(&payload, "Download action was rejected"))
    }
}

async fn clear_remote_download_history(
    client: &Client,
    session: &WebAppSession,
) -> Result<(), String> {
    let response = client
        .post(format!("{}/api/downloads/history/clear", session.base_url))
        .bearer_auth(&session.access_token)
        .send()
        .await
        .map_err(|e| format!("Failed to clear remote download history: {e}"))?;

    let status = response.status();
    let payload = response
        .json::<Value>()
        .await
        .map_err(|e| format!("Invalid clear history payload: {e}"))?;

    if !status.is_success() {
        let msg = response_error(&payload, "Failed to clear download history");
        return Err(format!("{msg} (status {status})"));
    }

    Ok(())
}

fn connected_session(state: &AppState) -> Result<WebAppSession, String> {
    let guard = state
        .connection
        .lock()
        .map_err(|e| format!("Connection lock error: {e}"))?;
    guard
        .clone()
        .ok_or_else(|| "Desktop app is not connected to any web app instance.".to_string())
}

#[tauri::command]
async fn download_universal_url(
    request: DownloadRequest,
    state: State<'_, AppState>,
    window: Window,
) -> Result<String, String> {
    let session = connected_session(&state)?;
    let client = build_http_client()?;
    let job = create_remote_download_job(&client, &session, &request).await?;
    let download_id = job.id.clone();

    state
        .download_manager
        .lock()
        .map_err(|e| format!("Download manager lock error: {e}"))?
        .insert_or_update(job.clone());

    let window_clone = window.clone();
    let download_id_clone = download_id.clone();
    let download_manager = state.download_manager.clone();
    let session_clone = session.clone();

    tokio::spawn(async move {
        let client = match build_http_client() {
            Ok(client) => client,
            Err(_) => return,
        };

        loop {
            match fetch_remote_download_job(&client, &session_clone, &download_id_clone).await {
                Ok(mut progress) => {
                    if progress.status == "cancelled" {
                        let local_status = download_manager
                            .lock()
                            .ok()
                            .and_then(|guard| guard.get_progress(&download_id_clone))
                            .map(|item| item.status)
                            .unwrap_or_default();
                        if local_status == "paused" {
                            progress.status = "paused".to_string();
                        }
                    }
                    if let Ok(guard) = download_manager.lock() {
                        guard.insert_or_update(progress.clone());
                    }
                    let _ = window_clone.emit("download-progress", &progress);
                    if matches!(progress.status.as_str(), "completed" | "failed") {
                        break;
                    }
                }
                Err(error) => {
                    let failed = DownloadProgress {
                        id: download_id_clone.clone(),
                        progress: 0.0,
                        status: "failed".to_string(),
                        file_path: None,
                        error: Some(error),
                    };
                    if let Ok(guard) = download_manager.lock() {
                        guard.insert_or_update(failed.clone());
                    }
                    let _ = window_clone.emit("download-progress", &failed);
                    break;
                }
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }
    });

    Ok(download_id)
}

#[tauri::command]
async fn get_download_history(state: State<'_, AppState>) -> Result<Vec<DownloadProgress>, String> {
    if let Ok(session) = connected_session(&state) {
        let client = build_http_client()?;
        let history = fetch_remote_download_history(&client, &session).await?;

        let manager = state
            .download_manager
            .lock()
            .map_err(|e| format!("Download manager lock error: {e}"))?;
        for item in &history {
            manager.insert_or_update(item.clone());
        }
        return Ok(history);
    }

    let manager = state
        .download_manager
        .lock()
        .map_err(|e| format!("Download manager lock error: {e}"))?;
    Ok(manager.get_history())
}

#[tauri::command]
async fn get_app_settings(state: State<'_, AppState>) -> Result<AppSettings, String> {
    let settings = state
        .settings
        .lock()
        .map_err(|e| format!("Settings lock error: {e}"))?;
    Ok(settings.clone())
}

#[tauri::command]
async fn update_app_settings(
    new_settings: AppSettings,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut settings = state
        .settings
        .lock()
        .map_err(|e| format!("Settings lock error: {e}"))?;
    *settings = new_settings;
    Ok(())
}

#[tauri::command]
async fn pause_download(download_id: String, state: State<'_, AppState>) -> Result<(), String> {
    let session = connected_session(&state)?;
    let client = build_http_client()?;
    trigger_remote_job_action(&client, &session, &download_id, "cancel").await?;

    let manager = state
        .download_manager
        .lock()
        .map_err(|e| format!("Download manager lock error: {e}"))?;
    manager.update_status(&download_id, "paused");
    Ok(())
}

#[tauri::command]
async fn resume_download(download_id: String, state: State<'_, AppState>) -> Result<(), String> {
    let session = connected_session(&state)?;
    let client = build_http_client()?;
    trigger_remote_job_action(&client, &session, &download_id, "retry").await?;

    let manager = state
        .download_manager
        .lock()
        .map_err(|e| format!("Download manager lock error: {e}"))?;
    manager.update_status(&download_id, "downloading");
    Ok(())
}

#[tauri::command]
async fn cancel_download(download_id: String, state: State<'_, AppState>) -> Result<(), String> {
    if let Ok(session) = connected_session(&state) {
        let client = build_http_client()?;
        trigger_remote_job_action(&client, &session, &download_id, "cancel").await?;
    }
    let manager = state
        .download_manager
        .lock()
        .map_err(|e| format!("Download manager lock error: {e}"))?;
    manager.update_status(&download_id, "cancelled");
    Ok(())
}

#[tauri::command]
async fn clear_completed_downloads(state: State<'_, AppState>) -> Result<(), String> {
    if let Ok(session) = connected_session(&state) {
        let client = build_http_client()?;
        clear_remote_download_history(&client, &session).await?;
    }

    let manager = state
        .download_manager
        .lock()
        .map_err(|e| format!("Download manager lock error: {e}"))?;
    manager.clear_completed();
    Ok(())
}

#[tauri::command]
async fn connect_to_web_app(
    request: ConnectionRequest,
    state: State<'_, AppState>,
) -> Result<bool, String> {
    let base_url = normalize_base_url(&request.url)?;
    let client = build_http_client()?;

    let normalized_code = request.pairing_code.trim();
    let (access_token, refresh_token) = if !normalized_code.is_empty() {
        fetch_pair_tokens(&client, &base_url, normalized_code).await?
    } else {
        let username = request.username.as_deref().unwrap_or_default();
        let password = request.password.as_deref().unwrap_or_default();
        fetch_login_tokens(&client, &base_url, username, password).await?
    };
    let username = fetch_current_username(&client, &base_url, &access_token).await?;

    let session = WebAppSession {
        base_url,
        access_token,
        refresh_token,
        username,
        connected_at: Utc::now(),
    };

    let mut guard = state
        .connection
        .lock()
        .map_err(|e| format!("Connection lock error: {e}"))?;
    *guard = Some(session);

    Ok(true)
}

#[tauri::command]
async fn disconnect_from_web_app(state: State<'_, AppState>) -> Result<(), String> {
    let mut guard = state
        .connection
        .lock()
        .map_err(|e| format!("Connection lock error: {e}"))?;
    *guard = None;
    Ok(())
}

#[tauri::command]
async fn generate_pairing_code(state: State<'_, AppState>) -> Result<String, String> {
    let session = {
        let guard = state
            .connection
            .lock()
            .map_err(|e| format!("Connection lock error: {e}"))?;
        guard.clone()
    };

    let session = session.ok_or_else(|| {
        "Desktop app is not connected. Generate pairing code from web UI first.".to_string()
    })?;

    let client = build_http_client()?;
    let response = client
        .get(format!("{}/auth/getpaircode", session.base_url))
        .bearer_auth(&session.access_token)
        .send()
        .await
        .map_err(|e| format!("Failed to request pairing code: {e}"))?;

    if response.status() == reqwest::StatusCode::UNAUTHORIZED {
        return Err("Session expired. Reconnect desktop app first.".to_string());
    }

    let payload = response
        .json::<Value>()
        .await
        .map_err(|e| format!("Invalid pairing code response: {e}"))?;

    let code = payload
        .get("code")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();

    if code.is_empty() {
        return Err("Server returned an empty pairing code".to_string());
    }

    Ok(code)
}

#[tauri::command]
async fn sync_with_web_app(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let session = {
        let guard = state
            .connection
            .lock()
            .map_err(|e| format!("Connection lock error: {e}"))?;
        guard.clone()
    };

    let session = session
        .ok_or_else(|| "Desktop app is not connected to any web app instance.".to_string())?;

    let client = build_http_client()?;
    collect_sync_items(&client, &session).await
}

#[tauri::command]
async fn show_notification(title: String, body: String) -> Result<(), String> {
    NativeFeatures::show_download_notification(&title, &body).map_err(|e| e.to_string())
}

#[tauri::command]
async fn open_in_file_manager(file_path: String) -> Result<(), String> {
    NativeFeatures::open_file_location(&file_path).map_err(|e| e.to_string())
}

#[tauri::command]
async fn set_auto_start(enabled: bool) -> Result<(), String> {
    NativeFeatures::set_auto_start(enabled).map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_audio_devices() -> Result<Vec<AudioDevice>, String> {
    Ok(vec![
        AudioDevice {
            id: "default".to_string(),
            name: "Default Device".to_string(),
        },
        AudioDevice {
            id: "speakers".to_string(),
            name: "Speakers".to_string(),
        },
        AudioDevice {
            id: "headphones".to_string(),
            name: "Headphones".to_string(),
        },
    ])
}

#[tauri::command]
async fn get_dragonfly_stats(state: State<'_, AppState>) -> Result<DragonflyStats, String> {
    let session = connected_session(&state)?;
    let client = build_http_client()?;

    // Fetch health endpoint
    let health_response = client
        .get(format!("{}/dragonfly/health", session.base_url))
        .bearer_auth(&session.access_token)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch DragonflyDB health: {e}"))?;

    let health = health_response
        .json::<Value>()
        .await
        .map_err(|e| format!("Invalid health response: {e}"))?;

    // Fetch stats endpoint
    let stats_response = client
        .get(format!("{}/dragonfly/stats", session.base_url))
        .bearer_auth(&session.access_token)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch DragonflyDB stats: {e}"))?;

    let stats = stats_response
        .json::<Value>()
        .await
        .map_err(|e| format!("Invalid stats response: {e}"))?;

    Ok(DragonflyStats {
        connected: health.get("connected").and_then(Value::as_bool).unwrap_or(false),
        latency_ms: health.get("latency_ms").and_then(Value::as_f64).unwrap_or(0.0),
        memory_used: stats
            .get("memory")
            .and_then(|m| m.get("used_memory_human"))
            .and_then(Value::as_str)
            .unwrap_or("0B")
            .to_string(),
        memory_peak: stats
            .get("memory")
            .and_then(|m| m.get("used_memory_peak_human"))
            .and_then(Value::as_str)
            .unwrap_or("0B")
            .to_string(),
        total_keys: stats
            .get("keyspace")
            .and_then(|k| k.get("db0"))
            .and_then(|db| db.get("keys"))
            .and_then(Value::as_i64)
            .unwrap_or(0),
        uptime_seconds: stats
            .get("server")
            .and_then(|s| s.get("uptime_in_seconds"))
            .and_then(Value::as_i64)
            .unwrap_or(0),
    })
}

#[tauri::command]
async fn get_dragonfly_services(state: State<'_, AppState>) -> Result<DragonflyServices, String> {
    let session = connected_session(&state)?;
    let client = build_http_client()?;

    let response = client
        .get(format!("{}/dragonfly/services", session.base_url))
        .bearer_auth(&session.access_token)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch DragonflyDB services: {e}"))?;

    let data = response
        .json::<Value>()
        .await
        .map_err(|e| format!("Invalid services response: {e}"))?;

    let services = data
        .get("services")
        .and_then(Value::as_array)
        .map(|arr| {
            arr.iter()
                .filter_map(|s| {
                    Some(CacheService {
                        name: s.get("name").and_then(Value::as_str).unwrap_or("").to_string(),
                        key_count: s.get("key_count").and_then(Value::as_i64).unwrap_or(0),
                        hit_rate: s.get("hit_rate").and_then(Value::as_f64).unwrap_or(0.0),
                    })
                })
                .collect()
        })
        .unwrap_or_default();

    Ok(DragonflyServices { services })
}

#[tauri::command]
async fn clear_dragonfly_cache(namespace: String, state: State<'_, AppState>) -> Result<(), String> {
    let session = connected_session(&state)?;
    let client = build_http_client()?;

    client
        .post(format!("{}/dragonfly/clear/{}", session.base_url, namespace))
        .bearer_auth(&session.access_token)
        .send()
        .await
        .map_err(|e| format!("Failed to clear cache: {e}"))?;

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AudioDevice {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInfo {
    pub version: String,
    pub notes: Option<String>,
    pub pub_date: Option<String>,
    pub install_available: bool,
}

#[tauri::command]
async fn check_for_updates(app: tauri::AppHandle) -> Result<Option<UpdateInfo>, String> {
    use tauri_plugin_updater::UpdaterExt;
    
    let updater = app.updater().map_err(|e| format!("Failed to get updater: {e}"))?;
    
    match updater.check().await {
        Ok(Some(update)) => {
            Ok(Some(UpdateInfo {
                version: update.version,
                notes: update.notes,
                pub_date: update.pub_date,
                install_available: true,
            }))
        }
        Ok(None) => Ok(None),
        Err(e) => Err(format!("Update check failed: {e}")),
    }
}

#[tauri::command]
async fn install_update(app: tauri::AppHandle) -> Result<(), String> {
    use tauri_plugin_updater::UpdaterExt;
    
    let updater = app.updater().map_err(|e| format!("Failed to get updater: {e}"))?;
    
    match updater.check().await {
        Ok(Some(update)) => {
            update.download_and_install(|_, _| {}, || {})
                .await
                .map_err(|e| format!("Update installation failed: {e}"))?;
            Ok(())
        }
        Ok(None) => Err("No update available".to_string()),
        Err(e) => Err(format!("Update check failed: {e}")),
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .manage(AppState {
            download_manager: Arc::new(Mutex::new(DownloadManager::new())),
            settings: Arc::new(Mutex::new(AppSettings::default())),
            connection: Arc::new(Mutex::new(None)),
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
            connect_to_web_app,
            disconnect_from_web_app,
            generate_pairing_code,
            sync_with_web_app,
            show_notification,
            open_in_file_manager,
            set_auto_start,
            get_audio_devices,
            check_for_updates,
            install_update,
            get_dragonfly_stats,
            get_dragonfly_services,
            clear_dragonfly_cache
        ])
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            let _ = window.set_title("SwingMusic Desktop");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
