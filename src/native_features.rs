// src/native_features.rs
use std::path::Path;
use std::process::Command;

pub struct NativeFeatures;

impl NativeFeatures {
    pub fn show_download_notification(
        title: &str,
        body: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(target_os = "windows")]
        {
            // Use PowerShell for notifications on Windows
            let _result = Command::new("powershell")
                .arg("-Command")
                .arg(format!(
                    "Add-Type -AssemblyName System.Windows.Forms; [System.Windows.Forms.MessageBox]::Show('{}', '{}', 'Information', 'OK')",
                    body.replace("'", "''"),
                    title.replace("'", "''")
                ))
                .output()?;
        }

        #[cfg(target_os = "macos")]
        {
            use std::ffi::CString;

            let title_c = CString::new(title)?;
            let body_c = CString::new(body)?;

            // macOS notification implementation
            let _result = Command::new("osascript")
                    .arg("-e")
                    .arg(format!(
                        "display notification \"{}\" with title \"{}\"",
                        body_c.to_str().unwrap_or(body),
                        title_c.to_str().unwrap_or(title)
                    ))
                    .output()?;
        }

        #[cfg(target_os = "linux")]
        {
            let _result = Command::new("notify-send").arg(title).arg(body).output()?;
        }

        Ok(())
    }

    pub fn open_file_location(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let path = Path::new(file_path);

        #[cfg(target_os = "windows")]
        {
            let _result = Command::new("explorer")
                .arg("/select,")
                .arg(path.canonicalize()?)
                .output()?;
        }

        #[cfg(target_os = "macos")]
        {
            let _result = Command::new("open")
                .arg("-R")
                .arg(path.canonicalize()?)
                .output()?;
        }

        #[cfg(target_os = "linux")]
        {
            let _result = Command::new("xdg-open")
                .arg(path.parent().unwrap_or(path))
                .output()?;
        }

        Ok(())
    }

    pub fn set_auto_start(enabled: bool) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(target_os = "windows")]
        {
            // Use PowerShell for auto-start on Windows
            let exe_path = std::env::current_exe()?;
            let app_name = "SwingMusic";

            if enabled {
                let _result = Command::new("powershell")
                    .arg("-Command")
                    .arg(format!(
                        "New-ItemProperty -Path 'HKCU:\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run' -Name '{}' -Value '{}' -PropertyType String -Force",
                        app_name,
                        exe_path.to_string_lossy()
                    ))
                    .output()?;
            } else {
                let _result = Command::new("powershell")
                    .arg("-Command")
                    .arg(format!(
                        "Remove-ItemProperty -Path 'HKCU:\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run' -Name '{}' -ErrorAction SilentlyContinue",
                        app_name
                    ))
                    .output()?;
            }
        }

        #[cfg(target_os = "macos")]
        {
            use std::fs;
            let home_dir = std::env::var("HOME")?;
            let launch_agents_dir = format!("{}/Library/LaunchAgents", home_dir);
            let plist_file = format!("{}/com.swingmusic.plist", launch_agents_dir);

            if enabled {
                fs::create_dir_all(&launch_agents_dir)?;
                let plist_content = format!(
                    r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.swingmusic</string>
    <key>ProgramArguments</key>
    <array>
        <string>{}</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
</dict>
</plist>
"#,
                    std::env::current_exe()?.to_string_lossy()
                );

                fs::write(&plist_file, plist_content)?;
            } else {
                fs::remove_file(plist_file)?;
            }
        }

        #[cfg(target_os = "linux")]
        {
            use std::fs;
            let home_dir = std::env::var("HOME")?;
            let autostart_dir = format!("{}/.config/autostart", home_dir);
            let desktop_file = format!("{}/swingmusic.desktop", autostart_dir);

            if enabled {
                fs::create_dir_all(&autostart_dir)?;
                let desktop_content = format!(
                    r#"[Desktop Entry]
Type=Application
Name=SwingMusic
Exec={}
Icon=swingmusic
Terminal=false
Categories=AudioVideo;Audio;
X-GNOME-Autostart-enabled=true
"#,
                    std::env::current_exe()?.to_string_lossy()
                );

                fs::write(&desktop_file, desktop_content)?;
            } else {
                fs::remove_file(desktop_file)?;
            }
        }

        Ok(())
    }
}
