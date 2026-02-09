use serde::{Deserialize, Serialize};
use tauri::AppHandle;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct NotificationEvent {
    pub app_name: String,
    pub sender: String,
    pub message: String,
    pub timestamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_icon: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct NotificationError {
    pub platform: String,
    pub error: String,
    pub suggestion: String,
}

// Platform-specific implementations
#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "linux")]
pub mod linux;

/// Start the notification listener for the current platform
pub async fn start_notification_listener(app_handle: AppHandle) {
    #[cfg(target_os = "windows")]
    {
        windows::listen_windows_notifications(app_handle).await;
    }

    #[cfg(target_os = "macos")]
    {
        macos::listen_macos_notifications(app_handle).await;
    }

    #[cfg(target_os = "linux")]
    {
        linux::listen_linux_notifications(app_handle).await;
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        eprintln!("Notification listener not supported on this platform");
    }
}
