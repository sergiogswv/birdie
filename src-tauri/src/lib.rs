mod notifications;
mod stt;
mod cdp;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn transcribe_audio(
    audio_base64: String,
    api_key: String,
    language_code: String,
) -> Result<stt::TranscriptionResult, String> {
    stt::transcribe_audio(audio_base64, api_key, language_code).await
}

#[tauri::command]
fn copy_to_clipboard(text: String) -> Result<(), String> {
    stt::copy_to_clipboard(text)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_tts::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            let handle = app.handle().clone();

            // Spawn notification listener in background
            tauri::async_runtime::spawn(async move {
                notifications::start_notification_listener(handle).await;
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            transcribe_audio,
            copy_to_clipboard,
            cdp::cdp_connect,
            cdp::cdp_get_tabs,
            cdp::cdp_find_tab,
            cdp::cdp_execute_script,
            cdp::cdp_start_monitoring,
            cdp::cdp_stop_monitoring
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
