use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Browser connection result
#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionResult {
    pub success: bool,
    pub message: String,
    pub tabs_count: usize,
    pub error_help_url: Option<String>,
}

/// Information about a browser tab
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TabInfo {
    pub id: String,
    pub title: String,
    pub url: String,
    pub domain: String,
    pub has_selector: bool,
}

/// Message detected from a tab
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CDPMessage {
    pub tab_id: String,
    pub tab_title: String,
    pub domain: String,
    pub sender: String,
    pub message: String,
    pub timestamp: String,
    pub source: String,
}

/// Result of script execution
#[derive(Debug, Serialize, Deserialize)]
pub struct ScriptResult {
    pub success: bool,
    pub result: Option<String>,
    pub error: Option<String>,
}

/// Monitoring status
#[derive(Debug, Serialize, Deserialize)]
pub struct MonitoringStatus {
    pub is_monitoring: bool,
    pub tabs_monitored: usize,
    pub interval_ms: u64,
}

/// Configuration for domain-specific selectors
#[derive(Debug, Clone)]
struct SelectorConfig {
    domain: &'static str,
    message_selector: &'static str,
    sender_selector: Option<&'static str>,
    source_name: &'static str,
}

/// Get selector configurations for different platforms
fn get_selector_configs() -> Vec<SelectorConfig> {
    vec![
        SelectorConfig {
            domain: "meet.google.com",
            message_selector: "[data-is-own-message='false'] span[data-message-text]",
            sender_selector: Some("[data-sender-nickname]"),
            source_name: "google-meet",
        },
        SelectorConfig {
            domain: "teams.microsoft.com",
            message_selector: "[data-testid='message-content']",
            sender_selector: Some("[data-testid='message-sender']"),
            source_name: "teams",
        },
        SelectorConfig {
            domain: "discord.com",
            message_selector: "[data-testid='message-content']",
            sender_selector: Some("[data-testid='username']"),
            source_name: "discord",
        },
        SelectorConfig {
            domain: "web.whatsapp.com",
            message_selector: "[data-testid='msg-container'] [class*='message']",
            sender_selector: Some("[data-testid='msg-sender']"),
            source_name: "whatsapp",
        },
        SelectorConfig {
            domain: "web.telegram.org",
            message_selector: ".message-content",
            sender_selector: Some(".message-sender"),
            source_name: "telegram",
        },
    ]
}

/// Extract domain from URL
fn extract_domain(url: &str) -> String {
    if let Ok(parsed_url) = url.parse::<url::Url>() {
        if let Some(host) = parsed_url.host_str() {
            return host.to_string();
        }
    }
    String::new()
}

/// Check if domain has a configured selector
fn has_selector_for_domain(domain: &str) -> bool {
    get_selector_configs().iter().any(|c| c.domain == domain)
}

/// Get selector config for a domain
fn get_selector_for_domain(domain: &str) -> Option<SelectorConfig> {
    get_selector_configs()
        .into_iter()
        .find(|c| c.domain == domain)
}

/// Hash a string for deduplication
fn hash_string(s: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

/// Connect to Chrome DevTools Protocol
#[tauri::command]
pub async fn cdp_connect(_port: u16) -> Result<ConnectionResult, String> {
    // Placeholder implementation for MVP
    // Full chromiumoxide integration requires more complex async patterns
    Ok(ConnectionResult {
        success: false,
        message: "CDP connection requires Chrome --remote-debugging-port=9222".to_string(),
        tabs_count: 0,
        error_help_url: Some(
            "https://github.com/SergioPachon/Birdie/wiki/Chrome-DevTools-Setup".to_string(),
        ),
    })
}

/// Get list of all tabs
#[tauri::command]
pub async fn cdp_get_tabs() -> Result<Vec<TabInfo>, String> {
    // Placeholder implementation for MVP
    Err("No hay conexión con Chrome. Conecta primero.".to_string())
}

/// Find a tab by title substring
#[tauri::command]
pub async fn cdp_find_tab(_title_contains: String) -> Result<Option<TabInfo>, String> {
    // Placeholder implementation for MVP
    Err("No hay conexión con Chrome. Conecta primero.".to_string())
}

/// Execute JavaScript in a tab
#[tauri::command]
pub async fn cdp_execute_script(_tab_id: String, _script: String) -> Result<ScriptResult, String> {
    Ok(ScriptResult {
        success: false,
        result: None,
        error: Some("Script execution requires proper target ID conversion".to_string()),
    })
}

/// Start monitoring tabs for messages
#[tauri::command]
pub async fn cdp_start_monitoring(
    _app_handle: tauri::AppHandle,
    interval_ms: u64,
) -> Result<MonitoringStatus, String> {
    Ok(MonitoringStatus {
        is_monitoring: false,
        tabs_monitored: 0,
        interval_ms,
    })
}

/// Stop monitoring
#[tauri::command]
pub async fn cdp_stop_monitoring() -> Result<MonitoringStatus, String> {
    Ok(MonitoringStatus {
        is_monitoring: false,
        tabs_monitored: 0,
        interval_ms: 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_domain() {
        assert_eq!(
            extract_domain("https://meet.google.com/abc-def-ghi"),
            "meet.google.com"
        );
        assert_eq!(
            extract_domain("https://teams.microsoft.com/v2/"),
            "teams.microsoft.com"
        );
    }

    #[test]
    fn test_has_selector_for_domain() {
        assert!(has_selector_for_domain("meet.google.com"));
        assert!(has_selector_for_domain("teams.microsoft.com"));
        assert!(!has_selector_for_domain("example.com"));
    }

    #[test]
    fn test_get_selector_for_domain() {
        let config = get_selector_for_domain("meet.google.com");
        assert!(config.is_some());
        assert_eq!(config.unwrap().source_name, "google-meet");
    }

    #[test]
    fn test_hash_string() {
        let hash1 = hash_string("test message");
        let hash2 = hash_string("test message");
        let hash3 = hash_string("different message");

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }
}
