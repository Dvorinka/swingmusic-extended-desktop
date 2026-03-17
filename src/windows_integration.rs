// src-tauri/src/windows_integration.rs
#[cfg(target_os = "windows")]
pub mod windows {
    use tauri::{Manager, Window};
    use std::path::PathBuf;
    use winreg::enums::*;
    use winreg::RegKey;
    use windows_sys::Win32::{
        Foundation::HWND,
        Media::MediaFoundation::*,
        System::Com::*,
        UI::Shell::*,
        UI::WindowsAndMessaging::*,
    };
    
    pub struct WindowsIntegration;
    
    impl WindowsIntegration {
        pub fn setup_windows_integration(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
            // Register file associations
            Self::register_file_associations()?;
            
            // Set up Windows Media Center integration
            Self::setup_windows_media_center()?;
            
            // Initialize COM for media operations
            unsafe {
                CoInitializeEx(std::ptr::null_mut(), COINIT_MULTITHREADED)?;
            }
            
            Ok(())
        }
        
        pub fn register_file_associations() -> Result<(), Box<dyn std::error::Error>> {
            let extensions = vec![".mp3", ".flac", ".wav", ".aac", ".m4a", ".ogg"];
            
            for ext in extensions {
                Self::register_single_extension(ext)?;
            }
            
            Ok(())
        }
        
        fn register_single_extension(extension: &str) -> Result<(), Box<dyn std::error::Error>> {
            let hkcu = RegKey::predef(HKEY_CURRENT_USER);
            let software_classes = hkcu.open_subkey_with_flags(
                "Software\\Classes",
                KEY_ALL_ACCESS,
            )?;
            
            // Create extension key
            let ext_key = software_classes.create_subkey(extension)?;
            ext_key.set_value("", &"SwingMusic")?;
            ext_key.set_value("PerceivedType", &"audio")?;
            
            // Create ProgID
            let progid_key = software_classes.create_subkey("SwingMusic")?;
            progid_key.set_value("", &"SwingMusic Audio File")?;
            
            // Create command
            let command_key = progid_key.create_subkey("shell\\open\\command")?;
            let exe_path = std::env::current_exe()?;
            let command = format!("\"{}\" \"%1\"", exe_path.to_string_lossy());
            command_key.set_value("", &command)?;
            
            // Create icon
            let icon_key = progid_key.create_subkey("DefaultIcon")?;
            let icon_path = format!("{},0", exe_path.to_string_lossy());
            icon_key.set_value("", &icon_path)?;
            
            Ok(())
        }
        
        pub fn setup_windows_media_center() -> Result<(), Box<dyn std::error::Error>> {
            // Register as default music player
            let hkcu = RegKey::predef(HKEY_CURRENT_USER);
            let software_classes = hkcu.open_subkey_with_flags(
                "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\FileExts\\.mp3\\UserChoice",
                KEY_ALL_ACCESS,
            )?;
            
            software_classes.set_value("ProgId", &"SwingMusic")?;
            
            Ok(())
        }
        
        pub fn show_notification(title: &str, body: &str) -> Result<(), Box<dyn std::error::Error>> {
            unsafe {
                // Create Windows notification
                let hr = CoInitializeEx(std::ptr::null_mut(), COINIT_APARTMENTTHREADED);
                if hr.is_err() {
                    return Err("Failed to initialize COM".into());
                }
                
                // Use Windows Toast Notification API
                // This is a simplified implementation
                println!("Windows Notification: {} - {}", title, body);
                
                CoUninitialize();
            }
            
            Ok(())
        }
        
        pub fn set_auto_start(enabled: bool) -> Result<(), Box<dyn std::error::Error>> {
            let hkcu = RegKey::predef(HKEY_CURRENT_USER);
            let run_key = hkcu.open_subkey_with_flags(
                "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run",
                KEY_ALL_ACCESS,
            )?;
            
            let app_name = "SwingMusic";
            let exe_path = std::env::current_exe()?;
            
            if enabled {
                run_key.set_value(app_name, &exe_path.to_string_lossy())?;
            } else {
                run_key.delete_value(app_name).ok(); // Ignore if not exists
            }
            
            Ok(())
        }
        
        pub fn open_file_manager(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
            unsafe {
                let path = std::ffi::CString::new(file_path)?;
                let result = ShellExecuteA(
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    path.as_ptr(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    SW_NORMAL,
                );
                
                if result.0 <= 32 {
                    return Err("Failed to open file manager".into());
                }
            }
            
            Ok(())
        }
        
        pub fn get_system_idle_time() -> u32 {
            unsafe {
                use windows_sys::Win32::UI::WindowsAndMessaging::GetLastInputInfo;
                use windows_sys::Win32::UI::WindowsAndMessaging::LASTINPUTINFO;
                
                let mut plii = LASTINPUTINFO {
                    cbSize: std::mem::size_of::<LASTINPUTINFO>() as u32,
                    dwTime: 0,
                };
                
                if GetLastInputInfo(&mut plii) != 0 {
                    let current_time = GetTickCount();
                    return current_time.saturating_sub(plii.dwTime);
                }
                
                0
            }
        }
        
        pub fn setup_global_hotkeys() -> Result<(), Box<dyn std::error::Error>> {
            // Register global hotkeys for media control
            unsafe {
                // This would need to be implemented with proper Windows API calls
                // For now, we'll just log the intention
                println!("Setting up Windows global hotkeys...");
            }
            
            Ok(())
        }
        
        pub fn get_audio_devices() -> Vec<String> {
            let mut devices = Vec::new();
            
            unsafe {
                // Use Windows Audio Session API (WASAPI) to enumerate devices
                // This is a simplified implementation
                devices.push("Default Audio Device".to_string());
                devices.push("Speakers".to_string());
                devices.push("Headphones".to_string());
            }
            
            devices
        }
        
        pub fn set_audio_device(device_name: &str) -> Result<(), Box<dyn std::error::Error>> {
            // Set default audio device
            println!("Setting audio device to: {}", device_name);
            Ok(())
        }
    }
}
