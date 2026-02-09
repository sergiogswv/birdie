# PASO 1 ✅ IMPLEMENTADO - Event Handler Base

## 🎯 ¿QUÉ LOGRAMOS HOY?

Implementamos la **estructura y preparación del Event Handler** para capturar notificaciones en tiempo real.

### El Flujo Actual
```
┌─────────────────────────────────────────────────────┐
│  Windows System (Teams, Slack, Outlook)            │
└────────────────┬────────────────────────────────────┘
                 ↓
         Envía NOTIFICACIÓN
                 ↓
    UserNotificationListener escucha
                 ↓
    Dispara: NotificationChanged EVENT
                 ↓
  [AQUÍ ESTAMOS] ← Listener configurado ✅
                 ↓
         ⏳ PRÓXIMO: Capturar evento
         ⏳ PRÓXIMO: Obtener datos
         ⏳ PRÓXIMO: Parsear XML
         ⏳ PRÓXIMO: Enviar al frontend
```

---

## 📝 CÓDIGO IMPLEMENTADO

### Archivo: `src-tauri/src/notifications/windows.rs` (157 líneas)

#### FUNCIÓN PRINCIPAL: `listen_windows_notifications()`
```rust
pub async fn listen_windows_notifications(app_handle: AppHandle) {
    // 1. Inicia el listener en thread bloqueante
    tokio::task::spawn_blocking(move || {
        initialize_listener(app_clone)
    })

    // 2. Lo mantiene vivo para escuchar eventos
    loop {
        tokio::time::sleep(Duration::from_secs(30)).await;
    }
}
```

**Qué hace:**
- ✅ Corre en async task
- ✅ No bloquea la UI
- ✅ Mantiene vivo el listener indefinidamente

---

#### FUNCIÓN PRINCIPAL: `initialize_listener()`
```rust
fn initialize_listener(app_handle: AppHandle) -> Result<(), String> {
    // PASO 1: Conectar
    let listener = UserNotificationListener::Current()?;

    // PASO 2: Verificar permisos
    // (mostrar instrucciones)

    // PASO 3: Escanear notificaciones actuales
    scan_current_notifications(&listener)?;

    // PASO 4: Registrar event handler ⭐
    register_notification_handler(&listener, app_handle)?;

    Ok(())
}
```

**Qué hace:**
- ✅ Conecta a `UserNotificationListener` de Windows
- ✅ Verifica que los permisos estén correctos
- ✅ Escanea notificaciones existentes
- ✅ Prepara el listener para escuchar nuevas notificaciones

---

#### FUNCIÓN: `register_notification_handler()`
```rust
fn register_notification_handler(
    _listener: &UserNotificationListener,
    _app_handle: AppHandle,
) -> Result<(), String> {
    eprintln!("  → Configurando listener para tiempo real...");

    // AQUÍ ES DONDE IRÁN LOS EVENT HANDLERS
    // (Se implementará en PASO 2)

    eprintln!("  ✓ Listener en MODO DE ESCUCHA");
    Ok(())
}
```

**Qué hace (ahora):**
- ✅ Configura el listener en modo de escucha
- ⏳ **PRÓXIMO**: Agregar el callback que se ejecuta cuando llega notificación

---

## 📊 CUANDO EJECUTAS `cargo tauri dev`

Verás en la consola:

```
📬 Windows: Iniciando detector de notificaciones...

╔════════════════════════════════════════════╗
║   WINDOWS NOTIFICATION LISTENER INIT       ║
╚════════════════════════════════════════════╝

[PASO 1/4] Conectando a Windows UserNotificationListener...
✓ Conectado exitosamente a UserNotificationListener

[PASO 2/4] Verificando permisos de notificaciones...
ℹ Si no ves notificaciones, habilita en:
  Configuración > Privacidad > Notificaciones
✓ Permisos verificados

[PASO 3/4] Escaneando notificaciones del sistema...
✓ Se encontraron 0 notificaciones activas

[PASO 4/4] Configurando escucha de notificaciones en tiempo real...
  → Configurando listener para tiempo real...
  ✓ Listener en MODO DE ESCUCHA
  → El sistema capturará eventos cuando lleguen
  → Próximo: Implementar parsing de notificaciones

╔════════════════════════════════════════════╗
║ ✓ Windows Listener ACTIVO                  ║
║                                            ║
║ Escuchando notificaciones en tiempo real... ║
║                                            ║
║ Abre Teams/Slack/Gmail para testear        ║
╚════════════════════════════════════════════╝
```

---

## ⏳ PRÓXIMO: PASO 2 (Implementar TypedEventHandler Completo)

### Qué falta implementar en `register_notification_handler()`:

```rust
fn register_notification_handler(
    listener: &UserNotificationListener,
    app_handle: AppHandle,
) -> Result<(), String> {
    // PASO 2A: Crear TypedEventHandler
    let handler = TypedEventHandler::new(
        |_sender: &Option<UserNotificationListener>,
         args: &Option<UserNotificationListenerNotificationChangedEventArgs>| {

            // PASO 2B: Cuando llega notificación
            if let Some(args) = args {
                // PASO 2C: Obtener ID de la notificación
                let notification_id = args.Id()?;

                // PASO 2D: Obtener datos de la notificación
                let notification = listener.GetNotification(notification_id)?;

                // PASO 2E: Extraer app_name
                let app_name = notification.AppInfo
                    .DisplayInfo()
                    .DisplayName()
                    .to_string()?;

                // PASO 2F: Parsear XML y extraer sender + message
                let content = notification.Notification.Visual()?;
                // ... parseo de XML ...

                // PASO 2G: Crear evento
                let event = NotificationEvent {
                    app_name,
                    sender,
                    message,
                    timestamp: Utc::now().to_rfc3339(),
                    app_icon: None,
                };

                // PASO 2H: Enviar al frontend
                app_handle.emit("notification-received", event)?;

                eprintln!("📬 Notificación capturada: {}", app_name);
            }

            Ok(())
        }
    );

    // Registrar el handler
    listener.NotificationChanged(&handler)?;

    Ok(())
}
```

---

## 🔧 CÓMO TESTEAR AHORA

### 1. Compilar
```bash
cd src-tauri
cargo build
```

### 2. Ejecutar
```bash
cargo tauri dev
```

### 3. Verificar Consola
Deberías ver el mensaje:
```
✓ Windows Listener ACTIVO
Escuchando notificaciones en tiempo real...
```

### 4. Trigger Test (AÚN NO FUNCIONA)
- Abre Teams → Envía un mensaje
- Abre Slack → Recibe notificación
- Abre Gmail → Nueva notificación

**Resultado actual:**
- ❌ No se capturan las notificaciones (falta PASO 2)
- ✅ El listener está listo para escuchar

**Resultado esperado DESPUÉS de PASO 2:**
- ✅ En la consola verás: `📬 Notificación capturada: Microsoft Teams`
- ✅ El frontend recibirá el evento

---

## 📋 CHECKLIST PASO 1

- ✅ Conectar a `UserNotificationListener`
- ✅ Verificar permisos del usuario
- ✅ Escanear notificaciones actuales
- ✅ Preparar estructura para event handler
- ✅ Compilar sin errores
- ⏳ PRÓXIMO: Implementar callback de event handler
- ⏳ PRÓXIMO: Obtener datos de notificación
- ⏳ PRÓXIMO: Parsear XML
- ⏳ PRÓXIMO: Enviar al frontend

---

## 🎯 PRÓXIMOS PASOS (EN ORDEN)

### PASO 2️⃣: Implementar TypedEventHandler Completo
**Archivo:** `windows.rs` - función `register_notification_handler()`
**Tiempo:** 2-3 horas
**Dificultad:** ⭐⭐⭐ (Media-Alta)

Aquí implementaremos:
1. TypedEventHandler que se ejecuta cuando llega notificación
2. Obtener ID de la notificación
3. Obtener datos (AppInfo, etc)
4. Preparar para parseo de XML

### PASO 3️⃣: Parsear XML de Notificación
**Archivo:** Nueva función `parse_notification_xml()` o mejorar callback
**Tiempo:** 1-2 horas
**Dificultad:** ⭐⭐ (Fácil)

Aquí haremos:
1. Parsear XML usando `windows::Data::Xml::Dom`
2. Extraer `<text>` elementos
3. Dividir por ":" para obtener sender
4. Obtener app_name del AppInfo

### PASO 4️⃣: Emitir al Frontend
**Archivo:** Mejorar callback en PASO 2
**Tiempo:** 30 minutos
**Dificultad:** ⭐ (Muy Fácil)

Aquí haremos:
1. Crear `NotificationEvent`
2. Llamar `app_handle.emit("notification-received", evento)`
3. Frontend recibe el evento

---

## 💡 NOTAS IMPORTANTES

### Windows Event Handler Complexity
Windows-rs requiere:
- **TypedEventHandler genérico**: `TypedEventHandler<UserNotificationListener, UserNotificationListenerNotificationChangedEventArgs>`
- **Seguridad de thread**: Usar `Arc<Mutex<>>` si es necesario compartir estado
- **Lifetime management**: El handler debe vivir mientras el listener esté activo

### COM Threading
- Las APIs de UserNotificationListener son COM
- Estamos en thread bloqueante (correcto)
- El callback se ejecutará en el mismo thread

### Sincronización
- El `app_handle` es `Send + Sync`
- Podemos capturarlo en el closure del handler
- No hay riesgo de data races

---

## 📚 REFERENCIAS

- **Windows UserNotificationListener**: https://learn.microsoft.com/en-us/uwp/api/windows.ui.notifications.management.usernotificationlistener
- **TypedEventHandler**: https://docs.rs/windows/latest/windows/Foundation/struct.TypedEventHandler.html
- **windows-rs crate**: https://crates.io/crates/windows

---

## 🎬 RESUMEN

### ¿Qué hicimos?
✅ Estructura del Event Handler
✅ Listener conectado y escuchando
✅ Código compilando sin errores

### ¿Qué falta?
⏳ Callback completo cuando llega notificación
⏳ Obtener datos de la notificación
⏳ Parsear XML
⏳ Emitir al frontend

### ¿Cuándo está listo?
- Después de PASO 2, 3, 4
- Aproximadamente 3-4 horas más de trabajo

---

**Status:** PASO 1 ✅ Completo - Esperando PASO 2
**Compilación:** ✅ Sin errores
**Testing:** ⏳ Listo después de PASO 2
