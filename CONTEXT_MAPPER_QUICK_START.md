# Context Mapper - Quick Start Guide

## 🎯 En 2 Minutos

### Paso 1: Abrir Chrome con Debug Port

**Windows:**
```powershell
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

---

### Paso 2: Verificar Conexión

Abre en tu navegador:
```
http://localhost:9222
```

Deberías ver JSON con "Browser" info. ✅

---

### Paso 3: Usar en TypeScript

```typescript
import { invoke } from '@tauri-apps/api/core';

// Validar si una app está soportada
const isSupported = await invoke('should_process_app', {
  appName: 'Google Meet'  // ✅ true
});

// Obtener targets de búsqueda
const targets = await invoke('get_search_targets', {
  appName: 'Google Meet'  // Retorna: ["Meet", "Google Meet"]
});

// Usar con Vision Module
const content = await invoke('get_active_tab_context', {
  targetName: targets[0]
});

console.log(content.content);  // Contenido extraído!
```

---

## 🌐 Apps Soportadas

```
✅ Google Meet      → Buscar en "Meet"
✅ Microsoft Teams  → Buscar en "Teams"
✅ Discord          → Buscar en "Discord"
✅ Slack            → Buscar en "Slack"
✅ WhatsApp Web     → Buscar en "WhatsApp"
✅ Telegram Web     → Buscar en "Telegram"
✅ Google Chat      → Buscar en "Chat"
```

---

## 🔄 Flujo Completo

```typescript
async function processNotification(app: string, message: string) {
  // 1️⃣ Validar en Context Mapper
  const isSupported = await invoke('should_process_app', { appName: app });

  if (!isSupported) {
    // ✅ Solo mostrar notificación visual
    showVisualNotification(app, message);
    return;
  }

  // 2️⃣ Obtener targets
  const targets = await invoke('get_search_targets', { appName: app });

  // 3️⃣ Extraer contenido con Vision
  const vision = await invoke('get_active_tab_context', {
    targetName: targets[0]
  });

  if (!vision.success) {
    // ✅ Pestaña no encontrada - mostrar solo visual
    showVisualNotification(app, message);
    return;
  }

  // 4️⃣ Reproducir con TTS
  await invoke('plugin:tts|speak', {
    payload: {
      text: `Notificación de ${app}: ${message}`,
      lang: 'es'
    }
  });
}
```

---

## 🎨 Hook Personalizado

```typescript
// src/hooks/useContextValidation.ts
import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

export function useContextValidation() {
  const [result, setResult] = useState(null);
  const [loading, setLoading] = useState(false);

  const validate = async (appName: string) => {
    setLoading(true);
    try {
      const context = await invoke('validate_app_context', { appName });
      const targets = await invoke('get_search_targets', { appName });
      setResult({ context, targets });
    } catch (error) {
      setResult(null);
    }
    setLoading(false);
  };

  return { result, loading, validate };
}
```

---

## 📊 Tabla de Referencia Rápida

| Función | Retorna | Uso |
|---------|---------|-----|
| `should_process_app(app)` | `bool` | ¿Procesar o solo visual? |
| `get_search_targets(app)` | `Vec<String>` | ¿Qué tab buscar? |
| `validate_app_context(app)` | `ContextTask` | Contexto completo |

---

## ✅ Checklist (5 min)

- [ ] Chrome abierto con `--remote-debugging-port=9222`
- [ ] Verificado `http://localhost:9222`
- [ ] Importado `invoke` en TypeScript
- [ ] Probado `should_process_app('Teams')` → true
- [ ] Probado `get_search_targets('Teams')` → ["Teams"]
- [ ] Probado Vision Module con targets
- [ ] ✅ ¡Funciona!

---

## 🚀 Casos Reales

### Notificación de Teams
```typescript
const isSupported = await invoke('should_process_app', { appName: 'Microsoft Teams' });
// ✅ true → Proceder con Vision + TTS
```

### Notificación de Spotify
```typescript
const isSupported = await invoke('should_process_app', { appName: 'Spotify' });
// ❌ false → Solo notificación visual
```

### Notificación de Discord
```typescript
const targets = await invoke('get_search_targets', { appName: 'Discord' });
// ["Discord"]

const content = await invoke('get_active_tab_context', { targetName: 'Discord' });
// Extrae mensajes del chat
```

---

## 📚 Documentación Completa

| Documento | Para... |
|-----------|---------|
| **CONTEXT_MAPPER_GUIDE.md** | Entender la arquitectura |
| **CONTEXT_MAPPER_EXAMPLES.md** | Ver 6 ejemplos prácticos |
| **CHROME_SETUP_GUIDE.md** | Resolver problemas de Chrome |
| **CONTEXT_MAPPER_QUICK_START.md** | Este resumen rápido |

---

## 🔧 Agregar Tu Propia App (Futuro)

El Context Mapper es extensible. Para futuras versiones:

```rust
// Se puede exponer como comando Tauri
#[tauri::command]
fn register_custom_app(
    app_name: String,
    search_targets: Vec<String>,
    url_patterns: Vec<String>,
    css_selector: String,
) -> Result<(), String> {
    // Registrar app personalizada
    Ok(())
}
```

---

## 💡 Tips

1. **Browser DevTools**: Presiona F12 en Chrome para ver estructura HTML real
2. **Test selectors**: Usa console: `document.querySelectorAll('[selector]').length`
3. **Debug Context**: Loguea resultado de `validate_app_context()` para ver qué se configura
4. **Cachejar**: Cachejar resultado de `get_search_targets()` para no llamar cada vez

---

## 🐛 Problemas Comunes

**P: "No se pudo conectar a Chrome"**
R: Ver CHROME_SETUP_GUIDE.md para abrir Chrome correctamente

**P: "App no está en el mapeo"**
R: Normal. Solo apps soportadas (Meet, Teams, Discord, etc) funcionan

**P: "No se encontró la pestaña"**
R: Asegúrate que la pestaña esté abierta en Chrome en ese momento

---

## 🎯 Siguiente Paso

Lee **CONTEXT_MAPPER_EXAMPLES.md** para ver cómo integrar en tu aplicación.

---

**Estado:** ✅ Listo para usar
**Apps:** 7 pre-configuradas
**Extensible:** Sí
**Compilado:** ✅ Sin errores

