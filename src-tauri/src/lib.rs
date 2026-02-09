mod notifications;
mod stt;
mod cdp;
mod vision;
mod context_mapper;

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

// Context Mapper Commands
#[tauri::command]
fn validate_app_context(app_name: String) -> Result<context_mapper::ContextTask, String> {
    let validator = context_mapper::ContextValidator::new();
    validator.validate_and_get_context(&app_name)
}

#[tauri::command]
fn get_search_targets(app_name: String) -> Result<Vec<String>, String> {
    let validator = context_mapper::ContextValidator::new();
    match validator.validate_and_get_context(&app_name) {
        Ok(task) => Ok(task.search_targets),
        Err(e) => Err(e),
    }
}

#[tauri::command]
fn should_process_app(app_name: String) -> bool {
    let validator = context_mapper::ContextValidator::new();
    validator.should_process_notification(&app_name, context_mapper::TaskPriority::Normal)
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
            cdp::cdp_stop_monitoring,
            vision::get_active_tab_context,
            validate_app_context,
            get_search_targets,
            should_process_app
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
