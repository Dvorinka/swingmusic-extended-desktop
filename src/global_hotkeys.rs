// src-tauri/src/global_hotkeys.rs
use tauri::{Manager, Window};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotkeyConfig {
    pub key_combination: String,
    pub action: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotkeyEvent {
    pub hotkey_id: String,
    pub action: String,
    pub timestamp: u64,
}

pub struct GlobalHotkeyManager {
    hotkeys: Arc<Mutex<HashMap<String, HotkeyConfig>>>,
    is_running: Arc<Mutex<bool>>,
    event_sender: Arc<Mutex<Option<tauri::AppHandle>>>,
}

impl GlobalHotkeyManager {
    pub fn new() -> Self {
        Self {
            hotkeys: Arc::new(Mutex::new(HashMap::new())),
            is_running: Arc::new(Mutex::new(false)),
            event_sender: Arc::new(Mutex::new(None)),
        }
    }
    
    pub fn initialize(&mut self, app: tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
        // Store app handle for event sending
        *self.event_sender.lock().unwrap() = Some(app);
        
        // Set up default hotkeys
        self.setup_default_hotkeys()?;
        
        // Start hotkey listener
        self.start_hotkey_listener()?;
        
        Ok(())
    }
    
    fn setup_default_hotkeys(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut hotkeys = self.hotkeys.lock().unwrap();
        
        // Media control hotkeys
        hotkeys.insert("play_pause".to_string(), HotkeyConfig {
            key_combination: "Space+Alt".to_string(),
            action: "play_pause".to_string(),
            enabled: true,
        });
        
        hotkeys.insert("next_track".to_string(), HotkeyConfig {
            key_combination: "Right+Alt".to_string(),
            action: "next_track".to_string(),
            enabled: true,
        });
        
        hotkeys.insert("prev_track".to_string(), HotkeyConfig {
            key_combination: "Left+Alt".to_string(),
            action: "prev_track".to_string(),
            enabled: true,
        });
        
        hotkeys.insert("volume_up".to_string(), HotkeyConfig {
            key_combination: "Up+Alt".to_string(),
            action: "volume_up".to_string(),
            enabled: true,
        });
        
        hotkeys.insert("volume_down".to_string(), HotkeyConfig {
            key_combination: "Down+Alt".to_string(),
            action: "volume_down".to_string(),
            enabled: true,
        });
        
        // Application hotkeys
        hotkeys.insert("toggle_window".to_string(), HotkeyConfig {
            key_combination: "F11+Alt".to_string(),
            action: "toggle_window".to_string(),
            enabled: true,
        });
        
        hotkeys.insert("show_hide".to_string(), HotkeyConfig {
            key_combination: "H+Alt".to_string(),
            action: "show_hide".to_string(),
            enabled: true,
        });
        
        Ok(())
    }
    
    fn start_hotkey_listener(&self) -> Result<(), Box<dyn std::error::Error>> {
        let hotkeys = self.hotkeys.clone();
        let is_running = self.is_running.clone();
        let event_sender = self.event_sender.clone();
        
        *is_running.lock().unwrap() = true;
        
        // Start platform-specific hotkey listener
        #[cfg(target_os = "windows")]
        {
            thread::spawn(move || {
                Self::windows_hotkey_loop(hotkeys, is_running, event_sender);
            });
        }
        
        #[cfg(target_os = "macos")]
        {
            thread::spawn(move || {
                Self::macos_hotkey_loop(hotkeys, is_running, event_sender);
            });
        }
        
        #[cfg(target_os = "linux")]
        {
            thread::spawn(move || {
                Self::linux_hotkey_loop(hotkeys, is_running, event_sender);
            });
        }
        
        Ok(())
    }
    
    #[cfg(target_os = "windows")]
    fn windows_hotkey_loop(
        hotkeys: Arc<Mutex<HashMap<String, HotkeyConfig>>>,
        is_running: Arc<Mutex<bool>>,
        event_sender: Arc<Mutex<Option<tauri::AppHandle>>>,
    ) {
        use windows_sys::Win32::{
            Foundation::{HWND, WPARAM, LPARAM, LRESULT},
            UI::WindowsAndMessaging::{
                RegisterHotKey, UnregisterHotKey, GetMessageA, TranslateMessage,
                DispatchMessageA, WM_HOTKEY, MOD_ALT, VK_SPACE, VK_RIGHT, VK_LEFT,
                VK_UP, VK_DOWN, VK_F11, VK_H, MOD_NOREPEAT,
            },
        };
        
        let mut hotkey_ids = HashMap::new();
        let mut next_id = 1;
        
        // Register hotkeys
        {
            let hotkeys_map = hotkeys.lock().unwrap();
            for (hotkey_id, config) in hotkeys_map.iter() {
                if config.enabled {
                    let (vk_code, modifiers) = Self::parse_windows_hotkey(&config.key_combination);
                    
                    unsafe {
                        if RegisterHotKey(HWND(0), next_id, modifiers, vk_code).is_ok() {
                            hotkey_ids.insert(hotkey_id.clone(), next_id);
                            next_id += 1;
                        }
                    }
                }
            }
        }
        
        // Message loop
        while *is_running.lock().unwrap() {
            unsafe {
                let mut msg = std::mem::zeroed();
                if GetMessageA(&mut msg, HWND(0), 0, 0).into() {
                    TranslateMessage(&msg);
                    DispatchMessageA(&msg);
                    
                    if msg.message == WM_HOTKEY {
                        let hotkey_id = msg.wParam as u32;
                        
                        // Find which hotkey was pressed
                        let hotkeys_map = hotkeys.lock().unwrap();
                        for (id, config) in hotkeys_map.iter() {
                            if let Some(&registered_id) = hotkey_ids.get(id) {
                                if registered_id == hotkey_id {
                                    Self::send_hotkey_event(&event_sender, id.clone(), &config.action);
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            
            thread::sleep(Duration::from_millis(10));
        }
        
        // Unregister hotkeys
        for (_, hotkey_id) in hotkey_ids {
            unsafe {
                let _ = UnregisterHotKey(HWND(0), hotkey_id);
            }
        }
    }
    
    #[cfg(target_os = "macos")]
    fn macos_hotkey_loop(
        hotkeys: Arc<Mutex<HashMap<String, HotkeyConfig>>>,
        is_running: Arc<Mutex<bool>>,
        event_sender: Arc<Mutex<Option<tauri::AppHandle>>>,
    ) {
        // macOS hotkey implementation using Carbon events
        // This is a simplified implementation
        
        while *is_running.lock().unwrap() {
            // Check for hotkey presses
            // In a real implementation, this would use CGEventTap or similar
            
            thread::sleep(Duration::from_millis(50));
        }
    }
    
    #[cfg(target_os = "linux")]
    fn linux_hotkey_loop(
        hotkeys: Arc<Mutex<HashMap<String, HotkeyConfig>>>,
        is_running: Arc<Mutex<bool>>,
        event_sender: Arc<Mutex<Option<tauri::AppHandle>>>,
    ) {
        // Linux hotkey implementation using X11 or Wayland
        // This is a simplified implementation using xdotool
        
        while *is_running.lock().unwrap() {
            // Check for hotkey presses
            // In a real implementation, this would use X11 events or libinput
            
            thread::sleep(Duration::from_millis(50));
        }
    }
    
    #[cfg(target_os = "windows")]
    fn parse_windows_hotkey(key_combination: &str) -> (u32, u32) {
        let mut modifiers = 0u32;
        let mut key_code = 0u32;
        
        use windows_sys::Win32::UI::WindowsAndMessaging::{
            MOD_ALT, MOD_CONTROL, MOD_SHIFT, MOD_WIN,
            VK_SPACE, VK_RIGHT, VK_LEFT, VK_UP, VK_DOWN, VK_F11, VK_H,
        };
        
        for part in key_combination.split('+') {
            match part.trim() {
                "Alt" => modifiers |= MOD_ALT,
                "Ctrl" | "Control" => modifiers |= MOD_CONTROL,
                "Shift" => modifiers |= MOD_SHIFT,
                "Win" | "Windows" => modifiers |= MOD_WIN,
                "Space" => key_code = VK_SPACE,
                "Right" => key_code = VK_RIGHT,
                "Left" => key_code = VK_LEFT,
                "Up" => key_code = VK_UP,
                "Down" => key_code = VK_DOWN,
                "F11" => key_code = VK_F11,
                "H" => key_code = VK_H,
                _ => {}
            }
        }
        
        (key_code, modifiers | MOD_NOREPEAT)
    }
    
    fn send_hotkey_event(
        event_sender: &Arc<Mutex<Option<tauri::AppHandle>>>,
        hotkey_id: String,
        action: String,
    ) {
        if let Some(app) = event_sender.lock().unwrap().as_ref() {
            let event = HotkeyEvent {
                hotkey_id,
                action,
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            };
            
            // Send event to frontend
            let _ = app.emit_all("global-hotkey", &event);
        }
    }
    
    pub fn add_hotkey(&self, hotkey_id: String, config: HotkeyConfig) -> Result<(), Box<dyn std::error::Error>> {
        let mut hotkeys = self.hotkeys.lock().unwrap();
        hotkeys.insert(hotkey_id, config);
        
        // Restart hotkey listener to pick up new hotkeys
        self.restart_hotkey_listener()?;
        
        Ok(())
    }
    
    pub fn remove_hotkey(&self, hotkey_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut hotkeys = self.hotkeys.lock().unwrap();
        hotkeys.remove(hotkey_id);
        
        // Restart hotkey listener
        self.restart_hotkey_listener()?;
        
        Ok(())
    }
    
    pub fn update_hotkey(&self, hotkey_id: &str, config: HotkeyConfig) -> Result<(), Box<dyn std::error::Error>> {
        let mut hotkeys = self.hotkeys.lock().unwrap();
        hotkeys.insert(hotkey_id.to_string(), config);
        
        // Restart hotkey listener
        self.restart_hotkey_listener()?;
        
        Ok(())
    }
    
    pub fn get_hotkeys(&self) -> HashMap<String, HotkeyConfig> {
        self.hotkeys.lock().unwrap().clone()
    }
    
    pub fn enable_hotkey(&self, hotkey_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut hotkeys = self.hotkeys.lock().unwrap();
        if let Some(config) = hotkeys.get_mut(hotkey_id) {
            config.enabled = true;
            self.restart_hotkey_listener()?;
        }
        Ok(())
    }
    
    pub fn disable_hotkey(&self, hotkey_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut hotkeys = self.hotkeys.lock().unwrap();
        if let Some(config) = hotkeys.get_mut(hotkey_id) {
            config.enabled = false;
            self.restart_hotkey_listener()?;
        }
        Ok(())
    }
    
    fn restart_hotkey_listener(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Stop current listener
        *self.is_running.lock().unwrap() = false;
        thread::sleep(Duration::from_millis(100));
        
        // Start new listener
        self.start_hotkey_listener()?;
        
        Ok(())
    }
    
    pub fn shutdown(&self) {
        *self.is_running.lock().unwrap() = false;
    }
}

// Tauri commands for hotkey management
#[tauri::command]
async fn add_global_hotkey(
    hotkey_id: String,
    key_combination: String,
    action: String,
    app: tauri::AppHandle,
) -> Result<(), String> {
    // Get hotkey manager from app state
    let manager = app.state::<Arc<Mutex<GlobalHotkeyManager>>>();
    let manager = manager.lock().unwrap();
    
    let config = HotkeyConfig {
        key_combination,
        action,
        enabled: true,
    };
    
    manager.add_hotkey(hotkey_id, config)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn remove_global_hotkey(
    hotkey_id: String,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let manager = app.state::<Arc<Mutex<GlobalHotkeyManager>>>();
    let manager = manager.lock().unwrap();
    
    manager.remove_hotkey(&hotkey_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn update_global_hotkey(
    hotkey_id: String,
    config: HotkeyConfig,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let manager = app.state::<Arc<Mutex<GlobalHotkeyManager>>>();
    let manager = manager.lock().unwrap();
    
    manager.update_hotkey(&hotkey_id, config)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_global_hotkeys(app: tauri::AppHandle) -> Result<HashMap<String, HotkeyConfig>, String> {
    let manager = app.state::<Arc<Mutex<GlobalHotkeyManager>>>();
    let manager = manager.lock().unwrap();
    
    Ok(manager.get_hotkeys())
}

#[tauri::command]
async fn enable_global_hotkey(
    hotkey_id: String,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let manager = app.state::<Arc<Mutex<GlobalHotkeyManager>>>();
    let manager = manager.lock().unwrap();
    
    manager.enable_hotkey(&hotkey_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn disable_global_hotkey(
    hotkey_id: String,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let manager = app.state::<Arc<Mutex<GlobalHotkeyManager>>>();
    let manager = manager.lock().unwrap();
    
    manager.disable_hotkey(&hotkey_id)
        .map_err(|e| e.to_string())
}

// Helper function to validate hotkey combinations
pub fn validate_hotkey_combination(combination: &str) -> Result<(), String> {
    let parts: Vec<&str> = combination.split('+').collect();
    
    if parts.is_empty() || parts.len() > 3 {
        return Err("Invalid hotkey combination".to_string());
    }
    
    let mut has_modifier = false;
    let mut has_key = false;
    
    for part in parts {
        let part = part.trim();
        match part {
            "Alt" | "Ctrl" | "Control" | "Shift" | "Win" | "Windows" => {
                has_modifier = true;
            }
            "Space" | "Right" | "Left" | "Up" | "Down" | "Enter" | "Escape" | "Tab" => {
                has_key = true;
            }
            c if c.len() == 1 => {
                // Single character key
                has_key = true;
            }
            c if c.starts_with('F') && c[1..].parse::<u32>().is_ok() => {
                // Function key
                has_key = true;
            }
            _ => {
                return Err(format!("Invalid hotkey component: {}", part));
            }
        }
    }
    
    if !has_key {
        return Err("Hotkey must include a key".to_string());
    }
    
    Ok(())
}

// Platform-specific hotkey validation
#[cfg(target_os = "windows")]
pub fn validate_windows_hotkey(combination: &str) -> Result<(), String> {
    validate_hotkey_combination(combination)?;
    
    // Windows-specific validation
    let parts: Vec<&str> = combination.split('+').collect();
    
    // Windows requires at least one modifier for global hotkeys
    let has_modifier = parts.iter().any(|&part| {
        matches!(part.trim(), "Alt" | "Ctrl" | "Control" | "Shift" | "Win" | "Windows")
    });
    
    if !has_modifier {
        return Err("Windows global hotkeys require at least one modifier key (Alt, Ctrl, Shift, or Win)".to_string());
    }
    
    Ok(())
}

#[cfg(target_os = "macos")]
pub fn validate_macos_hotkey(combination: &str) -> Result<(), String> {
    validate_hotkey_combination(combination)?;
    
    // macOS-specific validation
    // macOS hotkeys typically use Command key
    let parts: Vec<&str> = combination.split('+').collect();
    
    let has_cmd = parts.iter().any(|&part| {
        matches!(part.trim(), "Cmd" | "Command" | "Meta")
    });
    
    if !has_cmd {
        return Err("macOS global hotkeys typically use Command key".to_string());
    }
    
    Ok(())
}

#[cfg(target_os = "linux")]
pub fn validate_linux_hotkey(combination: &str) -> Result<(), String> {
    validate_hotkey_combination(combination)?;
    
    // Linux-specific validation
    // Linux hotkeys can use various combinations
    
    Ok(())
}
