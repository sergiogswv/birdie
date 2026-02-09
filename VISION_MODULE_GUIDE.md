# Vision Module Guide - Extracción de Contenido de Pestañas con chromiumoxide

## 📋 Resumen

El módulo `vision.rs` proporciona funcionalidad para conectarse a Chrome via Chrome DevTools Protocol (CDP) y extraer contenido de texto de pestañas abiertas usando `chromiumoxide`.

**Archivo:** `src-tauri/src/vision.rs`

## 🎯 Función Principal

### `get_active_tab_context(target_name: String)`

Busca una pestaña del navegador por nombre y extrae su contenido de texto.

**Parámetros:**
```rust
target_name: String  // Parte del título de la pestaña (case-insensitive)
```

**Retorna:**
```rust
Result<ContextResult, String>
```

**Estructura `ContextResult`:**
```rust
pub struct ContextResult {
    pub success: bool,           // Indica si la operación fue exitosa
    pub content: String,         // Texto extraído de la pestaña
    pub tab_title: String,       // Título completo de la pestaña
    pub tab_url: String,         // URL de la pestaña
    pub error: Option<String>,   // Mensaje de error si success=false
}
```

## 🔧 Cómo Registrar en `lib.rs`

### Paso 1: Agregar el módulo

En `src-tauri/src/lib.rs`, añade:

```rust
mod notifications;
mod stt;
mod cdp;
mod vision;  // ← NUEVO
```

### Paso 2: Registrar el comando

En la macro `generate_handler!`, agrega:

```rust
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
    vision::get_active_tab_context  // ← NUEVO
])
```

## 📍 Ubicación Actual en `lib.rs`

```rust
// src-tauri/src/lib.rs
mod notifications;
mod stt;
mod cdp;
mod vision;  // ← Línea 4

// ... resto del código ...

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
    vision::get_active_tab_context  // ← Línea en invoke_handler
])
```

## 💻 Cómo Usarlo desde TypeScript/React

### Importar y usar en frontend:

```typescript
import { invoke } from '@tauri-apps/api/core';

// Interfaz de TypeScript
interface ContextResult {
  success: boolean;
  content: string;
  tab_title: string;
  tab_url: string;
  error?: string;
}

// Llamar la función
async function extractMeetContent() {
  try {
    const result: ContextResult = await invoke('get_active_tab_context', {
      targetName: 'Google Meet'  // Buscar "Google Meet" en el título
    });

    if (result.success) {
      console.log('Contenido extraído:', result.content);
      console.log('Pestaña:', result.tab_title);
      console.log('URL:', result.tab_url);
    } else {
      console.error('Error:', result.error);
    }
  } catch (error) {
    console.error('Fallo en la invocación:', error);
  }
}
```

## 🌐 Plataformas Soportadas

El módulo tiene selectores CSS específicos para:

1. **Google Meet** (`meet.google.com`)
   - Extrae: Mensajes de chat + participantes
   - Selector: `[data-is-own-message]`, `[data-participant-id]`

2. **Microsoft Teams** (`teams.microsoft.com`)
   - Extrae: Mensajes de chat
   - Selector: `[data-testid="message-content"]`

3. **Discord** (`discord.com`)
   - Extrae: Nombre del canal + mensajes
   - Selector: `[data-testid="message-content"]`

4. **WhatsApp Web** (`web.whatsapp.com`)
   - Extrae: Mensajes de chat
   - Selector: `[data-testid="msg-container"]`

5. **Telegram Web** (`web.telegram.org`)
   - Extrae: Mensajes
   - Selector: `.message-content`

6. **Por defecto** (cualquier otra URL)
   - Extrae: Todo el texto visible
   - Selector: `document.body.innerText`

## 🚀 Ejemplo Completo de Uso

### 1. Desde React Component:

```typescript
import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

export function VisionPanel() {
  const [targetName, setTargetName] = useState('Google Meet');
  const [result, setResult] = useState(null);
  const [loading, setLoading] = useState(false);

  const handleExtract = async () => {
    setLoading(true);
    try {
      const contextResult = await invoke('get_active_tab_context', {
        targetName
      });
      setResult(contextResult);
    } catch (error) {
      console.error('Error:', error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div>
      <input
        value={targetName}
        onChange={(e) => setTargetName(e.target.value)}
        placeholder="Nombre de la pestaña"
      />
      <button onClick={handleExtract} disabled={loading}>
        {loading ? 'Extrayendo...' : 'Extraer Contenido'}
      </button>

      {result && (
        <div>
          <h3>Resultado</h3>
          <p>
            <strong>Pestaña:</strong> {result.tab_title}
          </p>
          <p>
            <strong>URL:</strong> {result.tab_url}
          </p>
          <pre>{result.content}</pre>
        </div>
      )}
    </div>
  );
}
```

## ⚙️ Requisitos Previos

### Chrome debe estar abierto con modo debug:

**Windows:**
```bash
"C:\Program Files\Google\Chrome\Application\chrome.exe" --remote-debugging-port=9222
```

**macOS:**
```bash
/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome --remote-debugging-port=9222
```

**Linux:**
```bash
google-chrome --remote-debugging-port=9222
```

## 🔍 Cómo Funciona Internamente

### Flujo de Ejecución:

1. **Conexión CDP**
   ```rust
   Browser::connect("ws://localhost:9222/devtools/browser").await
   ```
   - Conecta al protocolo de depuración de Chrome

2. **Obtener Pestañas**
   ```rust
   browser.fetch_targets().await
   ```
   - Obtiene lista de todas las pestañas abiertas

3. **Buscar Pestaña**
   ```rust
   targets.iter().find(|t|
       t.r#type == "page" &&
       t.title.to_lowercase().contains(&target_name.to_lowercase())
   )
   ```
   - Busca case-insensitive por titulo

4. **Obtener Página**
   ```rust
   browser.get_page(target.target_id.clone()).await
   ```
   - Accede a la pestaña específica

5. **Inyectar Script**
   ```rust
   page.evaluate(EvaluateParams::builder()
       .expression(js_script)
       .build().unwrap()).await
   ```
   - Ejecuta JavaScript en la página

6. **Extraer Resultado**
   - Procesa el resultado y retorna `ContextResult`

## 📊 Estructura de JavaScript Inyectado

Cada plataforma tiene su propio script:

```javascript
// Google Meet - Ejemplo
(function() {
    let content = [];

    // Extrae mensajes de chat
    const messages = document.querySelectorAll('[data-is-own-message]');
    messages.forEach(msg => {
        const text = msg.textContent?.trim();
        if (text) content.push(text);
    });

    return content.join('\n');
})()
```

**Key Points:**
- Scripts IIFE (Immediately Invoked Function Expression)
- Selectors CSS específicos por plataforma
- Fallback a `document.body.innerText` por defecto
- Filtering de contenido innecesario

## ✅ Gestión de Errores

El módulo maneja los siguientes escenarios:

| Escenario | Respuesta |
|-----------|----------|
| Chrome no disponible en puerto 9222 | `Err("No se pudo conectar...")` |
| No hay pestañas | `Err("No se encontró pestaña...")` |
| Pestaña cerrada durante ejecución | `Err("No se pudo acceder...")` |
| Script falla | `ContextResult { success: false, error: Some(...) }` |
| Éxito | `ContextResult { success: true, content: "..." }` |

## 🎨 Integración con Birdie

Para integrar en tu UI existente:

```typescript
// src/components/VisionExtractor.tsx
import { useCDP } from '../hooks/useCDP';
import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

export function VisionExtractor() {
  const { connected } = useCDP();
  const [content, setContent] = useState('');

  if (!connected) {
    return <p>⚠️ Chrome no está conectado</p>;
  }

  const handleExtract = async (platform: string) => {
    const result = await invoke('get_active_tab_context', {
      targetName: platform
    });
    setContent(result.content);
  };

  return (
    <div className="vision-extractor">
      <button onClick={() => handleExtract('Google Meet')}>
        Extraer Meet
      </button>
      <button onClick={() => handleExtract('Teams')}>
        Extraer Teams
      </button>
      <pre>{content}</pre>
    </div>
  );
}
```

## 🧪 Tests Incluidos

El módulo incluye tests unitarios:

```bash
cargo test vision::tests
```

Tests:
- ✅ `test_extraction_script_meet` - Verifica selector Meet
- ✅ `test_extraction_script_teams` - Verifica selector Teams
- ✅ `test_extraction_script_discord` - Verifica selector Discord
- ✅ `test_extraction_script_default` - Verifica fallback

## 📝 Notas Importantes

1. **Seguridad:** Asegúrate de que solo confías en el código JavaScript inyectado
2. **Performance:** Los scripts se ejecutan síncronamente; limita el tamaño del DOM
3. **Selectors:** Los selectores CSS pueden cambiar; mantén actualizado
4. **Threading:** La función es `async` y se ejecuta en el runtime de Tokio
5. **Error Recovery:** Siempre revisa `success` en la respuesta

## 🔮 Mejoras Futuras

- [ ] Soportar selección de múltiples pestañas simultáneamente
- [ ] Cache de selectores por plataforma
- [ ] Actualización automática de selectores
- [ ] OCR para contenido de imágenes
- [ ] Integración con sistema de notificaciones
- [ ] Guardar historial de extracciones

---

**Status:** ✅ Compilado y funcional
**Dependencias:** `chromiumoxide`, `serde`, `tauri`
**Líneas de código:** 263 (incluyendo tests y comentarios)
