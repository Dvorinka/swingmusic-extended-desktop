// src-tauri/src/linux_integration.rs
#[cfg(target_os = "linux")]
pub mod linux {
    use tauri::{Manager, Window};
    use std::path::PathBuf;
    use std::fs;
    use std::io::Write;
    
    pub struct LinuxIntegration;
    
    impl LinuxIntegration {
        pub fn setup_linux_integration(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
            // Set up MPRIS integration
            Self::setup_mpris()?;
            
            // Register for desktop notifications
            Self::setup_linux_notifications()?;
            
            // Set up desktop entry
            Self::setup_desktop_entry()?;
            
            // Register file associations
            Self::register_file_associations()?;
            
            Ok(())
        }
        
        pub fn setup_mpris() -> Result<(), Box<dyn std::error::Error>> {
            // MPRIS2 D-Bus service implementation
            // This would require a D-Bus library like dbus-rs
            
            let mpris_service_content = r#"[D-BUS Service]
Name=org.mpris.MediaPlayer2.SwingMusic
Exec=swingmusic
"#;
            
            let home_dir = std::env::var("HOME")?;
            let dbus_services_dir = format!("{}/.local/share/dbus-1/services", home_dir);
            fs::create_dir_all(&dbus_services_dir)?;
            
            let service_file = format!("{}/org.mpris.MediaPlayer2.SwingMusic.service", dbus_services_dir);
            fs::write(service_file, mpris_service_content)?;
            
            println!("MPRIS2 service registered");
            Ok(())
        }
        
        pub fn setup_linux_notifications() -> Result<(), Box<dyn std::error::Error>> {
            // Check if notification server is available
            if std::process::Command::new("which")
                .arg("notify-send")
                .output()
                .map(|output| output.status.success())
                .unwrap_or(false)
            {
                println!("Linux notifications available via notify-send");
            } else {
                println!("Linux notifications not available");
            }
            
            Ok(())
        }
        
        pub fn show_notification(title: &str, body: &str) -> Result<(), Box<dyn std::error::Error>> {
            std::process::Command::new("notify-send")
                .args(&["-i", "swingmusic", "-t", "5000", title, body])
                .output()?;
            
            Ok(())
        }
        
        pub fn setup_desktop_entry() -> Result<(), Box<dyn std::error::Error>> {
            let home_dir = std::env::var("HOME")?;
            let applications_dir = format!("{}/.local/share/applications", home_dir);
            fs::create_dir_all(&applications_dir)?;
            
            let desktop_file = format!("{}/swingmusic.desktop", applications_dir);
            
            let exe_path = std::env::current_exe()?;
            let icon_path = exe_path.parent()
                .unwrap_or_else(|| std::path::Path::new("."))
                .join("icons")
                .join("swingmusic.png");
            
            let desktop_content = format!(
                r#"[Desktop Entry]
Version=1.0
Type=Application
Name=SwingMusic
Comment=Advanced music player with Universal Source integration
Exec={}
Icon={}
Terminal=false
Categories=AudioVideo;Audio;Player;
MimeType=audio/mp3;audio/flac;audio/wav;audio/aac;audio/x-m4a;audio/ogg;
StartupNotify=true
StartupWMClass=swingmusic
X-GNOME-Autostart-enabled=true
Keywords=music;audio;player;streaming;download;
"#,
                exe_path.to_string_lossy(),
                icon_path.to_string_lossy()
            );
            
            fs::write(desktop_file, desktop_content)?;
            
            // Update desktop database
            std::process::Command::new("update-desktop-database")
                .arg(&applications_dir)
                .output()
                .ok();
            
            println!("Desktop entry created");
            Ok(())
        }
        
        pub fn register_file_associations() -> Result<(), Box<dyn std::error::Error>> {
            let home_dir = std::env::var("HOME")?;
            let mime_dir = format!("{}/.local/share/mime", home_dir);
            let packages_dir = format!("{}/packages", mime_dir);
            fs::create_dir_all(&packages_dir)?;
            
            let mime_file = format!("{}/swingmusic.xml", packages_dir);
            
            let mime_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<mime-info xmlns="http://www.freedesktop.org/standards/shared-mime-info">
    <mime-type type="audio/x-swingmusic">
        <comment>SwingMusic Audio File</comment>
        <glob pattern="*.mp3"/>
        <glob pattern="*.flac"/>
        <glob pattern="*.wav"/>
        <glob pattern="*.aac"/>
        <glob pattern="*.m4a"/>
        <glob pattern="*.ogg"/>
    </mime-type>
</mime-info>
"#;
            
            fs::write(mime_file, mime_content)?;
            
            // Update mime database
            std::process::Command::new("update-mime-database")
                .arg(&mime_dir)
                .output()
                .ok();
            
            println!("File associations registered");
            Ok(())
        }
        
        pub fn set_auto_start(enabled: bool) -> Result<(), Box<dyn std::error::Error>> {
            let home_dir = std::env::var("HOME")?;
            let autostart_dir = format!("{}/.config/autostart", home_dir);
            fs::create_dir_all(&autostart_dir)?;
            
            let desktop_file = format!("{}/swingmusic.desktop", autostart_dir);
            
            if enabled {
                let exe_path = std::env::current_exe()?;
                let desktop_content = format!(
                    r#"[Desktop Entry]
Version=1.0
Type=Application
Name=SwingMusic
Exec={}
Icon=swingmusic
Terminal=false
Categories=AudioVideo;Audio;
X-GNOME-Autostart-enabled=true
"#,
                    exe_path.to_string_lossy()
                );
                
                fs::write(desktop_file, desktop_content)?;
            } else {
                fs::remove_file(desktop_file).ok();
            }
            
            Ok(())
        }
        
        pub fn open_file_manager(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
            // Try different file managers
            let file_managers = vec![
                ("nautilus", vec!["--select", file_path]),
                ("dolphin", vec!["--select", file_path]),
                ("thunar", vec![file_path]),
                ("pcmanfm", vec![file_path]),
                ("xdg-open", vec![file_path]),
            ];
            
            for (fm, args) in file_managers {
                if std::process::Command::new(fm)
                    .args(&args)
                    .output()
                    .map(|output| output.status.success())
                    .unwrap_or(false)
                {
                    return Ok(());
                }
            }
            
            Err("No suitable file manager found".into())
        }
        
        pub fn setup_global_hotkeys() -> Result<(), Box<dyn std::error::Error>> {
            // Use xdotool or similar for global hotkeys
            if std::process::Command::new("which")
                .arg("xdotool")
                .output()
                .map(|output| output.status.success())
                .unwrap_or(false)
            {
                println!("Global hotkeys available via xdotool");
            } else {
                println!("Global hotkeys not available (install xdotool)");
            }
            
            Ok(())
        }
        
        pub fn get_audio_devices() -> Vec<String> {
            let mut devices = Vec::new();
            
            // Use ALSA or PulseAudio to enumerate audio devices
            if let Ok(output) = std::process::Command::new("pactl")
                .args(&["list", "sinks"])
                .output()
            {
                let output_str = String::from_utf8_lossy(&output.stdout);
                for line in output_str.lines() {
                    if line.contains("Sink #") {
                        if let Some(name_start) = line.find("Description: ") {
                            let name = line[name_start + 13..].trim();
                            devices.push(name.to_string());
                        }
                    }
                }
            }
            
            // Fallback to common device names
            if devices.is_empty() {
                devices.push("Default Audio Device".to_string());
                devices.push("Speakers".to_string());
                devices.push("Headphones".to_string());
            }
            
            devices
        }
        
        pub fn set_audio_device(device_name: &str) -> Result<(), Box<dyn std::error::Error>> {
            // Use PulseAudio to set default sink
            if let Ok(output) = std::process::Command::new("pactl")
                .args(&["list", "sinks"])
                .output()
            {
                let output_str = String::from_utf8_lossy(&output.stdout);
                
                for line in output_str.lines() {
                    if line.contains(device_name) {
                        if let Some(sink_start) = line.find("Sink #") {
                            let sink_part = &line[sink_start..];
                            if let Some(space_pos) = sink_part.find(' ') {
                                let sink_id = &sink_part[6..space_pos]; // Skip "Sink #"
                                
                                std::process::Command::new("pactl")
                                    .args(&["set-default-sink", sink_id])
                                    .output()?;
                                
                                return Ok(());
                            }
                        }
                    }
                }
            }
            
            Err("Audio device not found".into())
        }
        
        pub fn get_system_idle_time() -> u32 {
            // Use xprintidle or similar
            if let Ok(output) = std::process::Command::new("xprintidle")
                .output()
            {
                if output.status.success() {
                    let idle_ms = String::from_utf8_lossy(&output.stdout)
                        .trim()
                        .parse::<u32>()
                        .unwrap_or(0);
                    return idle_ms / 1000; // Convert to seconds
                }
            }
            
            0
        }
        
        pub fn get_desktop_environment() -> String {
            // Detect desktop environment
            if let Ok(desktop) = std::env::var("XDG_CURRENT_DESKTOP") {
                return desktop.to_uppercase();
            }
            
            // Fallback detection
            if std::env::var("GNOME_DESKTOP_SESSION_ID").is_ok() {
                return "GNOME".to_string();
            }
            
            if std::env::var("KDE_FULL_SESSION").is_ok() {
                return "KDE".to_string();
            }
            
            if std::process::Command::new("ps")
                .arg("-e")
                .arg("xfce4-session")
                .output()
                .map(|output| output.status.success())
                .unwrap_or(false)
            {
                return "XFCE".to_string();
            }
            
            "UNKNOWN".to_string()
        }
        
        pub fn is_wayland() -> bool {
            std::env::var("WAYLAND_DISPLAY").is_ok() ||
            std::env::var("XDG_SESSION_TYPE")
                .map(|s| s.to_lowercase() == "wayland")
                .unwrap_or(false)
        }
        
        pub fn setup_system_tray() -> Result<(), Box<dyn std::error::Error>> {
            // Check if system tray is available
            let desktop_env = Self::get_desktop_environment();
            let is_wayland = Self::is_wayland();
            
            println!("Desktop environment: {}", desktop_env);
            println!("Wayland: {}", is_wayland);
            
            if is_wayland {
                // Wayland tray support varies by compositor
                println!("Wayland detected - tray support may be limited");
            } else {
                println!("X11 detected - tray should work normally");
            }
            
            Ok(())
        }
    }
}
