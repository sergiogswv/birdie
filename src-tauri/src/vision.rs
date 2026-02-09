use chromiumoxide::browser::Browser;
use serde::{Deserialize, Serialize};

/// Result of context extraction from a tab
#[derive(Debug, Serialize, Deserialize)]
pub struct ContextResult {
    pub success: bool,
    pub content: String,
    pub tab_title: String,
    pub tab_url: String,
    pub error: Option<String>,
}

/// Get active tab context by extracting its text content
///
/// # Arguments
/// * `target_name` - Part of the tab title to search for (case-insensitive)
///
/// # Returns
/// * `Ok(ContextResult)` - Successfully extracted content
/// * `Err(String)` - Error message if operation failed
///
/// # Example
/// ```ignore
/// let result = get_active_tab_context("Google Meet".to_string()).await?;
/// println!("Content: {}", result.content);
/// ```
#[tauri::command]
pub async fn get_active_tab_context(target_name: String) -> Result<ContextResult, String> {
    // Connect to Chrome DevTools Protocol
    let mut browser = match Browser::connect("http://localhost:9222").await {
        Ok((browser, _handler)) => browser,
        Err(e) => {
            return Err(format!(
                "No se pudo conectar a Chrome en puerto 9222: {}. \
                Asegúrate de que Chrome está abierto con --remote-debugging-port=9222",
                e
            ))
        }
    };

    // Get all targets (tabs) from the browser
    let targets = match browser.fetch_targets().await {
        Ok(targets) => targets,
        Err(e) => {
            return Err(format!("No se pudo obtener las pestañas: {}", e));
        }
    };

    // Find the tab that matches the target_name (case-insensitive)
    let target = match targets
        .iter()
        .find(|t| t.r#type == "page" && t.title.to_lowercase().contains(&target_name.to_lowercase()))
    {
        Some(target) => target,
        None => {
            let available = targets
                .iter()
                .filter(|t| t.r#type == "page")
                .map(|t| format!("'{}' ({})", t.title, t.url))
                .collect::<Vec<_>>()
                .join(", ");
            return Err(format!(
                "No se encontró pestaña que contenga '{}'. Pestañas disponibles: {}",
                target_name, available
            ));
        }
    };

    let tab_title = target.title.clone();
    let tab_url = target.url.clone();

    // Get the page for this tab
    let page = match browser.get_page(target.target_id.clone()).await {
        Ok(page) => page,
        Err(e) => {
            return Err(format!("No se pudo acceder a la pestaña: {}", e));
        }
    };

    // Extract JavaScript based on the domain
    let js_script = get_extraction_script(&tab_url);

    // Execute the script in the page
    match page
        .evaluate(
            chromiumoxide::cdp::js_protocol::runtime::EvaluateParams::builder()
                .expression(js_script)
                .build()
                .unwrap(),
        )
        .await
    {
        Ok(eval_result) => {
            // Extract the content from the evaluation result
            // The structure of eval_result may vary; we'll use the debug format
            let content = format!("{:?}", eval_result)
                .chars()
                .take(500) // Limit to first 500 characters
                .collect::<String>();

            // Clean up excessive debug formatting
            let content = if content.is_empty() || content.contains("Undefined") {
                "No se encontró contenido en la pestaña".to_string()
            } else {
                content
            };

            Ok(ContextResult {
                success: true,
                content,
                tab_title,
                tab_url,
                error: None,
            })
        }
        Err(e) => {
            // Return error with success=false
            Ok(ContextResult {
                success: false,
                content: String::new(),
                tab_title,
                tab_url,
                error: Some(format!("Error al ejecutar script: {}", e)),
            })
        }
    }
}

/// Get the appropriate JavaScript extraction script based on the tab URL
///
/// This function returns different selectors for different platforms
/// to extract the most relevant content (chat messages, visible text, etc.)
fn get_extraction_script(url: &str) -> String {
    let url_lower = url.to_lowercase();

    if url_lower.contains("meet.google.com") {
        // For Google Meet: Extract visible chat and participant names
        r#"
        (function() {
            let content = [];

            // Get chat messages
            const messages = document.querySelectorAll('[data-is-own-message]');
            messages.forEach(msg => {
                const text = msg.textContent?.trim();
                if (text) content.push(text);
            });

            // Get participant list
            const participants = document.querySelectorAll('[data-participant-id]');
            participants.forEach(p => {
                const name = p.textContent?.trim();
                if (name && name.length < 100) content.push('Participante: ' + name);
            });

            return content.join('\n') || 'No se encontró contenido';
        })()
        "#.to_string()
    } else if url_lower.contains("teams.microsoft.com") {
        // For Microsoft Teams: Extract chat messages
        r#"
        (function() {
            let content = [];

            const messages = document.querySelectorAll('[data-testid="message-content"]');
            messages.forEach(msg => {
                const text = msg.textContent?.trim();
                if (text) content.push(text);
            });

            return content.join('\n') || 'No se encontró contenido';
        })()
        "#.to_string()
    } else if url_lower.contains("discord.com") {
        // For Discord: Extract messages and channel name
        r#"
        (function() {
            let content = [];

            // Channel name
            const channel = document.querySelector('h1[class*="title"]');
            if (channel?.textContent) content.push('Canal: ' + channel.textContent.trim());

            // Messages
            const messages = document.querySelectorAll('[data-testid="message-content"]');
            messages.forEach(msg => {
                const text = msg.textContent?.trim();
                if (text) content.push(text);
            });

            return content.join('\n') || 'No se encontró contenido';
        })()
        "#.to_string()
    } else if url_lower.contains("whatsapp.com") {
        // For WhatsApp Web: Extract chat messages
        r#"
        (function() {
            let content = [];

            const messages = document.querySelectorAll('[data-testid="msg-container"]');
            messages.forEach(msg => {
                const text = msg.textContent?.trim();
                if (text && text.length < 1000) content.push(text);
            });

            return content.join('\n') || 'No se encontró contenido';
        })()
        "#.to_string()
    } else if url_lower.contains("telegram.org") {
        // For Telegram Web: Extract messages
        r#"
        (function() {
            let content = [];

            const messages = document.querySelectorAll('.message-content');
            messages.forEach(msg => {
                const text = msg.textContent?.trim();
                if (text) content.push(text);
            });

            return content.join('\n') || 'No se encontró contenido';
        })()
        "#.to_string()
    } else {
        // Default: Extract all visible text
        r#"
        (function() {
            return document.body.innerText || 'No se encontró contenido';
        })()
        "#.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extraction_script_meet() {
        let script = get_extraction_script("https://meet.google.com/abc-def-ghi");
        assert!(script.contains("data-is-own-message"));
    }

    #[test]
    fn test_extraction_script_teams() {
        let script = get_extraction_script("https://teams.microsoft.com");
        assert!(script.contains("message-content"));
    }

    #[test]
    fn test_extraction_script_discord() {
        let script = get_extraction_script("https://discord.com");
        assert!(script.contains("Canal:"));
    }

    #[test]
    fn test_extraction_script_default() {
        let script = get_extraction_script("https://example.com");
        assert!(script.contains("document.body.innerText"));
    }
}
