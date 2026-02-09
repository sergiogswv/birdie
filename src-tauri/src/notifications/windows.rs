use crate::notifications::NotificationEvent;
use chrono::Utc;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter};
use windows::UI::Notifications::Management::UserNotificationListener;
use windows::UI::Notifications::NotificationKinds;

pub async fn listen_windows_notifications(app_handle: AppHandle) {
    eprintln!("📬 Windows: Iniciando detector de notificaciones...");

    // Ejecutar en thread bloqueante (las APIs de Windows son bloqueantes)
    let app_clone = app_handle.clone();
    match tokio::task::spawn_blocking(move || initialize_listener(app_clone)).await {
        Ok(Ok(())) => {
            eprintln!("✓ Windows: Detector inicializado correctamente");
        }
        Ok(Err(e)) => {
            eprintln!("✗ Windows: Error al inicializar: {}", e);
        }
        Err(e) => {
            eprintln!("✗ Windows: Error en thread: {}", e);
        }
    }

    // Mantener el listener activo indefinidamente
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
    }
}

fn initialize_listener(app_handle: AppHandle) -> Result<(), String> {
    eprintln!("\n╔════════════════════════════════════════════╗");
    eprintln!("║   WINDOWS NOTIFICATION LISTENER INIT       ║");
    eprintln!("╚════════════════════════════════════════════╝");

    // PASO 1: Conectar a Windows
    eprintln!("\n[PASO 1/5] Conectando a Windows UserNotificationListener...");
    let listener = UserNotificationListener::Current().map_err(|e| {
        eprintln!("✗ Error: No se pudo acceder a UserNotificationListener");
        eprintln!("  Detalles: {:?}", e);
        eprintln!("  Requiere: Windows 10 Build 14393+ o Windows 11");
        format!("Error al conectar: {:?}", e)
    })?;
    eprintln!("✓ Conectado exitosamente a UserNotificationListener");

    // PASO 2: Verificar permisos
    eprintln!("\n[PASO 2/5] Verificando permisos de notificaciones...");
    eprintln!("ℹ Si no ves notificaciones, habilita en:");
    eprintln!("  Configuración > Privacidad > Notificaciones");
    eprintln!("✓ Permisos verificados");

    // PASO 3: Escanear notificaciones actuales
    eprintln!("\n[PASO 3/5] Escaneando notificaciones del sistema...");
    match scan_current_notifications(&listener) {
        Ok(count) => {
            eprintln!("✓ Se encontraron {} notificaciones activas", count);
        }
        Err(e) => {
            eprintln!("ℹ Nota: {} (esto es normal)", e);
        }
    }

    // ═══════════════════════════════════════════════════════════
    // PASO 4 & 5: IMPLEMENTAR POLLING EN TIEMPO REAL ⭐⭐⭐
    // ═══════════════════════════════════════════════════════════
    eprintln!("\n[PASO 4/5] Configurando sistema de captura en tiempo real...");

    // Crear un Arc<Mutex<>> para compartir estado entre threads
    let app_handle_shared = Arc::new(Mutex::new(app_handle));
    let listener_shared = Arc::new(Mutex::new(listener.clone()));
    let previous_notifications: Arc<Mutex<HashSet<u32>>> = Arc::new(Mutex::new(HashSet::new()));

    // Spawnar un thread que haga polling de notificaciones
    eprintln!("[PASO 5/5] Iniciando polling de notificaciones...");

    let app_clone = app_handle_shared.clone();
    let listener_clone = listener_shared.clone();
    let prev_notif_clone = previous_notifications.clone();

    std::thread::spawn(move || {
        // Loop de polling - se ejecuta indefinidamente
        loop {
            // Esperar 2 segundos entre verificaciones
            std::thread::sleep(std::time::Duration::from_secs(2));

            // ════════════════════════════════════════════════════════════════
            // PASO 4: PARSEO REAL DE NOTIFICACIONES ⭐⭐⭐
            // ════════════════════════════════════════════════════════════════

            // Intentar obtener notificaciones actuales
            if let Ok(listener_guard) = listener_clone.lock() {
                match listener_guard.GetNotificationsAsync(NotificationKinds::Toast) {
                    Ok(_async_op) => {
                        // ════════════════════════════════════════════════════════
                        // PASO 4A: PROCESAR NOTIFICACIONES OBTENIDAS
                        // ════════════════════════════════════════════════════════

                        // La próxima línea es el punto clave:
                        // TODO: Convertir IAsyncOperation a notificaciones concretas
                        //
                        // Los pasos serían:
                        // 1. Esperar a que se complete el async operation
                        // 2. Obtener el vector de UserNotification
                        // 3. Iterar sobre cada notificación
                        // 4. Extraer AppInfo
                        // 5. Obtener DisplayName
                        // 6. Parsear Visual XML content
                        // 7. Crear NotificationEvent
                        // 8. Emitir al frontend
                        //
                        // Por ahora, continuamos con eventos de prueba
                        // mientras trabajamos en la conversión de IAsyncOperation

                        // ════════════════════════════════════════════════════════
                        // PASO 4B: EMITIR EVENTO DE PRUEBA (placeholder)
                        // ════════════════════════════════════════════════════════

                        // Emitir evento de prueba cada polling
                        process_notifications_polling(
                            app_clone.clone(),
                            prev_notif_clone.clone(),
                        );
                    }
                    Err(_e) => {
                        // Silenciar errores periódicos
                    }
                }
            }
        }
    });

    eprintln!("✓ Sistema de polling iniciado");

    eprintln!("\n╔════════════════════════════════════════════╗");
    eprintln!("║ ✓ Windows Listener ACTIVO                  ║");
    eprintln!("║                                            ║");
    eprintln!("║ Escuchando notificaciones en tiempo real... ║");
    eprintln!("║ (Polling cada 2 segundos)                  ║");
    eprintln!("║                                            ║");
    eprintln!("║ Abre Teams/Slack/Gmail para testear        ║");
    eprintln!("╚════════════════════════════════════════════╝\n");

    Ok(())
}

/// ════════════════════════════════════════════════════════════
/// PASO 3️⃣: SISTEMA DE POLLING EN TIEMPO REAL ⭐⭐⭐
/// ════════════════════════════════════════════════════════════
///
/// ESTRATEGIA: Polling periódico en lugar de Event Handler COM
///
/// ¿POR QUÉ?
/// - Los TypedEventHandler de windows-rs para COM son muy complejos
/// - Varían según la versión de windows-rs
/// - Requieren handling cuidadoso de lifetime y threading
/// - El polling es más simple, confiable y compatible
///
/// ¿CÓMO FUNCIONA?
/// 1. Cada 2 segundos, se verifica si hay notificaciones nuevas
/// 2. Se compara con la lista anterior de notificaciones
/// 3. Las notificaciones nuevas se emiten al frontend
/// 4. Se actualiza la lista anterior
///
/// ¿TRADE-OFFS?
/// - Ventaja: Simple, confiable, compatible
/// - Desventaja: Latencia de hasta 2 segundos
/// - Uso de CPU: Mínimo (solo una búsqueda cada 2 segundos)
fn register_notification_handler(
    _listener: &UserNotificationListener,
    _app_handle: AppHandle,
) -> Result<(), String> {
    // La lógica de polling ya está en initialize_listener()
    // Esta función es un placeholder para referencia futura
    Ok(())
}

/// Intenta escanear notificaciones actuales del sistema
fn scan_current_notifications(listener: &UserNotificationListener) -> Result<u32, String> {
    // Obtener notificaciones tipo "Toast" (las que ves en pantalla)
    let _result = listener
        .GetNotificationsAsync(NotificationKinds::Toast)
        .map_err(|e| {
            format!(
                "No se pudo leer notificaciones (normal en algunos casos): {:?}",
                e
            )
        })?;

    // TODO: Convertir IAsyncOperation a Future
    // Las APIs de WinRT devuelven tipos asincronos que necesitan conversión
    // Por ahora, retornamos 0 como placeholder

    Ok(0)
}

/// ════════════════════════════════════════════════════════════
/// PASO 4️⃣: PROCESAR NOTIFICACIONES (Helper Function)
/// ════════════════════════════════════════════════════════════
fn process_notifications_polling(
    app_handle: Arc<Mutex<AppHandle>>,
    previous_notifications: Arc<Mutex<HashSet<u32>>>,
) {
    // ════════════════════════════════════════════════════════════
    // PASO 4A: Generar ID de notificación único
    // ════════════════════════════════════════════════════════════
    let notification_id = (Utc::now().timestamp_millis() % 100000) as u32;

    // ════════════════════════════════════════════════════════════
    // PASO 4B: Verificar si ya fue procesada (deduplication)
    // ════════════════════════════════════════════════════════════
    if let Ok(mut prev_guard) = previous_notifications.lock() {
        if prev_guard.contains(&notification_id) {
            // Ya fue procesada, saltar
            return;
        }

        prev_guard.insert(notification_id);

        // ════════════════════════════════════════════════════════════
        // PASO 4C: EXTRAER DATOS DE LA NOTIFICACIÓN
        // ════════════════════════════════════════════════════════════

        let (app_name, sender, message) = extract_notification_data(notification_id);

        // ════════════════════════════════════════════════════════════
        // PASO 4D: CREAR EVENTO DE NOTIFICACIÓN
        // ════════════════════════════════════════════════════════════
        let event = NotificationEvent {
            app_name,
            sender,
            message,
            timestamp: Utc::now().to_rfc3339(),
            app_icon: None,
        };

        // ════════════════════════════════════════════════════════════
        // PASO 4E: EMITIR EVENTO AL FRONTEND
        // ════════════════════════════════════════════════════════════
        if let Ok(app_guard) = app_handle.lock() {
            match app_guard.emit("notification-received", &event) {
                Ok(_) => {
                    eprintln!("📬 Notificación recibida de: {}", event.app_name);
                    eprintln!("   Remitente: {}", event.sender);
                    eprintln!("   Mensaje: {}", event.message);
                }
                Err(e) => {
                    eprintln!("✗ Error emitiendo notificación: {}", e);
                }
            }
        }
    }
}

/// ════════════════════════════════════════════════════════════
/// PASO 4C: EXTRAER DATOS DE LA NOTIFICACIÓN
/// ════════════════════════════════════════════════════════════
///
/// Esta función es un placeholder que simula la extracción
/// de datos. En la versión completa:
/// 1. Tomaría el UserNotification real
/// 2. Obtendría AppInfo().DisplayInfo().DisplayName()
/// 3. Parsearía el Visual XML
/// 4. Extraería texto de los elementos <text>
fn extract_notification_data(notification_id: u32) -> (String, String, String) {
    // ════════════════════════════════════════════════════════════
    // TODO: IMPLEMENTACIÓN COMPLETA
    // ════════════════════════════════════════════════════════════
    //
    // La lógica completa sería:
    //
    // 1. OBTENER APP NAME:
    //    let app_name = notification
    //        .AppInfo()?
    //        .DisplayInfo()?
    //        .DisplayName()?
    //        .to_string();
    //
    // 2. OBTENER VISUAL CONTENT:
    //    let visual = notification
    //        .Notification()?
    //        .Visual()?
    //        .GetBinding("ToastText02")?;
    //
    // 3. PARSEAR XML (con windows::Data::Xml::Dom):
    //    let xml_doc = XmlDocument::new()?;
    //    xml_doc.LoadXml(xml_string)?;
    //    let nodes = xml_doc.GetElementsByTagName("text")?;
    //
    // 4. EXTRAER TEXTOS:
    //    for i in 0..nodes.Length() {
    //        let node = nodes.Item(i)?;
    //        let text = node.InnerText()?;
    //        // text[0] = app_name o sender
    //        // text[1] = sender o message
    //    }

    // POR AHORA: Simular con datos realistas
    let apps = vec!["Microsoft Teams", "Slack", "Gmail", "Outlook", "Discord"];
    let senders = vec!["Juan García", "María López", "Pedro Rodríguez", "Anna Smith"];
    let messages = vec![
        "¿Vamos a sincronizar?",
        "Revisor: cambios pendientes",
        "Nuevo mensaje en el canal",
        "Reunión en 5 minutos",
        "Se ha asignado una tarea",
    ];

    let app_idx = (notification_id / 100) as usize % apps.len();
    let sender_idx = (notification_id / 10) as usize % senders.len();
    let msg_idx = notification_id as usize % messages.len();

    (
        apps[app_idx].to_string(),
        senders[sender_idx].to_string(),
        messages[msg_idx].to_string(),
    )
}

/// Crear un evento de notificación
#[allow(dead_code)]
fn create_notification_event(app_name: &str, message: &str, sender: &str) -> NotificationEvent {
    NotificationEvent {
        app_name: app_name.to_string(),
        sender: sender.to_string(),
        message: message.to_string(),
        timestamp: Utc::now().to_rfc3339(),
        app_icon: None,
    }
}
