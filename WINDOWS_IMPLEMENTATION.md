# Windows Notification Listener - Plan de Implementación

## ✅ LO QUE HICIMOS (FASE 1 - ESTRUCTURA)

### Estado Actual
```
✓ Conectar a Windows UserNotificationListener
✓ Verificar permisos del usuario
✓ Escanear notificaciones actuales
✓ El código compila sin errores
⏳ FALTA: Capturar notificaciones EN TIEMPO REAL
⏳ FALTA: Parsear datos de la notificación
⏳ FALTA: Enviar al frontend
```

### Archivo Modificado
- `src-tauri/src/notifications/windows.rs` (92 líneas)

### Flujo Actual
```
┌─────────────────────────────────────────┐
│  Windows Notification System            │
└────────────┬────────────────────────────┘
             ↓
       [CONECTADO] ✓
       UserNotificationListener::Current()
             ↓
       [PERMISO VERIFICADO] ✓
       CheckPermissions()
             ↓
       [ESCANEANDO] ✓
       GetNotificationsAsync(Toast)
             ↓
       ⏳ PRÓXIMO: ESCUCHAR EN TIEMPO REAL
       EventHandler::NotificationChanged
             ↓
       ⏳ PRÓXIMO: PARSEAR DATOS
       ⏳ PRÓXIMO: ENVIAR AL FRONTEND
```

---

## 📋 PRÓXIMOS PASOS (EN ORDEN)

### PASO 1: Implementar Event Handler para Notificaciones en Tiempo Real
**Objetivo:** Cuando llegue una notificación NUEVA, capturarla

**Archivo:** `windows.rs`
**Función:** Agregar handler para `NotificationChanged` event

**Pseudo-código:**
```rust
// Escuchar cuando llega una NUEVA notificación
listener.NotificationChanged += |args| {
    // args contiene:
    // - UserNotificationId
    // - ChangeKind (Added, Removed, Updated)

    // Obtener la notificación
    let notification = listener.GetNotification(id);

    // Parsear datos
    let app_name = notification.AppInfo.DisplayName;
    let content = notification.Notification.Visual;

    // Enviar al frontend
    app_handle.emit("notification-received", evento)
}
```

**Dificultad:** ⭐⭐⭐ (Media - Manejo de callbacks WinRT)

---

### PASO 2: Parsear Contenido XML de Notificaciones
**Objetivo:** Extraer app_name, sender, message del XML

**Archivo:** `windows.rs`
**Función:** Mejorar `create_notification_event()`

**Las notificaciones vienen en XML:**
```xml
<toast>
  <visual>
    <binding template="ToastText02">
      <text id="1">Microsoft Teams</text>
      <text id="2">Juan: ¡Hola! ¿Cómo estás?</text>
    </binding>
  </visual>
</toast>
```

**Pseudo-código:**
```rust
fn parse_xml(xml: &str) -> NotificationEvent {
    // Buscar <text id="1"> → app_name
    // Buscar <text id="2"> → Dividir por ":" → sender y message

    // Retornar NotificationEvent
}
```

**Dificultad:** ⭐⭐ (Fácil - Solo parsing de XML)

---

### PASO 3: Emitir Eventos al Frontend
**Objetivo:** Enviar notificaciones capturadas a React

**Archivo:** `windows.rs`
**Función:** Usar `app_handle.emit()`

**Código:**
```rust
let evento = NotificationEvent {
    app_name: "Microsoft Teams".to_string(),
    sender: "juan@empresa.com".to_string(),
    message: "¡Hola!".to_string(),
    timestamp: Utc::now().to_rfc3339(),
    app_icon: None,
};

// Enviar al frontend
app_handle.emit("notification-received", &evento)?;
```

**Dificultad:** ⭐ (Muy fácil)

---

## 🎯 DIAGRAMA COMPLETO (CÓMO SERÁ)

```
SISTEMA WINDOWS
    ↓
Teams/Slack/Outlook/etc envía notificación
    ↓
[1] NotificationChanged EVENT FIRES
    ↓
[2] Obtener notification data
    ↓
[3] PARSEAR XML
    Extraer:
    - app_name: "Microsoft Teams"
    - sender: "juan@empresa.com"
    - message: "¡Hola!"
    ↓
[4] CREAR NotificationEvent struct
    ↓
[5] app_handle.emit("notification-received", evento)
    ↓
FRONTEND (React) RECIBE EL EVENTO
    ↓
Mostrar en la UI:
┌──────────────────────────────┐
│ Microsoft Teams              │
│ juan@empresa.com             │
│ ¡Hola!                       │
│ 14:30:45                     │
└──────────────────────────────┘
```

---

## 🔧 CÓDIGO ACTUAL (RESUMEN)

### `listen_windows_notifications()` - La función principal
```rust
pub async fn listen_windows_notifications(app_handle: AppHandle) {
    // 1. Inicia el listener en un thread bloqueante
    tokio::task::spawn_blocking(move || {
        initialize_listener(app_clone)
    })

    // 2. Lo mantiene vivo indefinidamente
    loop {
        tokio::time::sleep(Duration::from_secs(30)).await;
    }
}
```

### `initialize_listener()` - Inicialización
```rust
fn initialize_listener(_app_handle: AppHandle) -> Result<(), String> {
    // PASO 1: Conectar a Windows
    let listener = UserNotificationListener::Current()?;

    // PASO 2: Verificar permisos
    // (mostramos mensajes al usuario)

    // PASO 3: Escanear notificaciones
    scan_current_notifications(&listener)?;

    // TODO: Aquí agregamos el event handler
    Ok(())
}
```

---

## 🚀 CÓMO TESTEAR

### 1. Compilar
```bash
cd src-tauri
cargo build
```

### 2. Correr en Debug
```bash
cargo tauri dev
```

### 3. Deberías Ver en la Consola
```
📬 Windows: Iniciando detector de notificaciones...

╔════════════════════════════════════════════╗
║   WINDOWS NOTIFICATION LISTENER INIT       ║
╚════════════════════════════════════════════╝

[PASO 1/3] Conectando a Windows UserNotificationListener...
✓ Conectado exitosamente a UserNotificationListener

[PASO 2/3] Verificando permisos de notificaciones...
✓ Permisos verificados

[PASO 3/3] Escaneando notificaciones del sistema...
✓ Se encontraron 0 notificaciones activas

╔════════════════════════════════════════════╗
║ ✓ Windows Listener Listo                   ║
║                                            ║
║ Escuchando nuevas notificaciones...        ║
╚════════════════════════════════════════════╝
```

### 4. Trigger una Notificación
Abre Teams, Slack, Gmail, etc. y envía un mensaje de prueba

**Resultado esperado AHORA:**
- ❌ Notificación NO se captura (porque falta implementar el event handler)

**Resultado esperado DESPUÉS:**
- ✅ Verás en la consola: `📬 Notificación capturada: Microsoft Teams`
- ✅ El frontend recibirá el evento

---

## 📊 CHECKLIST PARA COMPLETAR

```
PASO 1: Event Handler
  ☐ Implementar listener para NotificationChanged
  ☐ Obtener ID de notificación
  ☐ Obtener datos de la notificación
  ☐ Compilar sin errores

PASO 2: Parsear XML
  ☐ Extraer información del XML
  ☐ Obtener app_name (Teams, Slack, etc)
  ☐ Obtener sender (quién envía)
  ☐ Obtener message (qué dice)
  ☐ Obtener timestamp

PASO 3: Enviar al Frontend
  ☐ Crear NotificationEvent
  ☐ Llamar app_handle.emit()
  ☐ Compilar sin errores
  ☐ Testear que el frontend recibe el evento

FASE FINAL: Testing
  ☐ Testear con Teams
  ☐ Testear con Slack
  ☐ Testear con Outlook
  ☐ Testear con Gmail
  ☐ Verificar que funciona en background
```

---

## 💾 DEPENDENCIAS USADAS

```toml
# Ya están en Cargo.toml:

# Windows WinRT APIs
windows = { version = "0.58", features = [
    "UI_Notifications",
    "UI_Notifications_Management",
    "Foundation",
    "Foundation_Collections",
    "Data_Xml_Dom"
] }

# Async runtime
tokio = { version = "1", features = ["full"] }

# Timestamps
chrono = "0.4"

# Tauri (para emit)
tauri = "2"
```

---

## ⚠️ NOTAS IMPORTANTES

### Windows Specifics
- ✅ Funciona en Windows 10 Build 14393+
- ✅ Funciona en Windows 11
- ✅ Las APIs son COM (Component Object Model)
- ✅ Necesitan un thread bloqueante (por eso usamos `spawn_blocking`)

### Limitaciones Conocidas
- ❌ Solo captura notificaciones tipo "Toast"
- ❌ No captura notificaciones de apps en Store (UWP) con sandbox
- ❌ Algunas apps antiguas pueden usar sistemas propios de notificaciones
- ✅ Teams, Slack, Outlook, Gmail → SÍ FUNCIONA

### Permisos Requeridos
El usuario debe habilitar:
```
Configuración
  → Privacidad
    → Notificaciones
      → "Allow apps to access your notifications"
```

---

## 📚 RECURSOS

- **Windows UserNotificationListener:** https://learn.microsoft.com/en-us/uwp/api/windows.ui.notifications.management.usernotificationlistener
- **WinRT APIs:** https://docs.rs/windows/latest/
- **Toast XML Format:** https://learn.microsoft.com/en-us/windows/apps/design/shell/tiles-and-notifications/adaptive-interactive-toasts

---

## 🎬 SIGUIENTE

Una vez implementado Windows completo, haremos:
1. **macOS** (NSDistributedNotificationCenter)
2. **Linux** (D-Bus org.freedesktop.Notifications)
3. **Frontend** (React UI para mostrar notificaciones)

---

**Status:** Estructura lista - Próximo: Event Handler
**Complejidad:** ⭐⭐⭐ (Media - Código WinRT requiere cuidado)
**Tiempo Estimado:** 2-3 horas para implementación completa
