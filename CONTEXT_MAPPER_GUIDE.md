# Context Mapper - Diccionario de Mapeo de Contexto para Birdie

## 🎯 ¿Qué es el Context Mapper?

Es un **diccionario centralizado** que mapea aplicaciones (Gmail, Teams, Slack, etc.) con sus contextos en el navegador (URLs, selectores CSS, prioridades).

**Problema que resuelve:**
- Sin context mapper: Birdie intenta procesar TODAS las notificaciones
- Con context mapper: Birdie **valida primero** si tiene configuración para esa app

```
Notificación llega
  ↓
Context Mapper valida si existe mapeo
  ↓
¿Mapeo existe? ✅ → Procesar (Vision + TTS)
¿Mapeo existe? ❌ → Solo mostrar notificación visual
```

---

## 📊 Estructura Principal

### `ContextTask`
Define toda la información para procesar una app:

```rust
pub struct ContextTask {
    pub app_name: String,              // "Google Meet"
    pub search_targets: Vec<String>,    // ["Meet", "Google Meet"]
    pub url_patterns: Vec<String>,      // ["meet.google.com"]
    pub css_selector: String,           // Script JS para extraer contenido
    pub priority: TaskPriority,         // High, Normal, Low, etc.
    pub enabled: bool,                  // Habilitado/Deshabilitado
}
```

### `TaskPriority`
Niveles de prioridad:

```rust
pub enum TaskPriority {
    Disabled,  // ❌ No procesar
    Low,       // 🟡 Procesar si hay recursos
    Normal,    // 🟢 Procesar siempre
    High,      // 🔵 Prioritario
    Critical,  // 🔴 Máxima prioridad
}
```

---

## 📱 Apps Pre-configuradas

El mapper viene con **7 apps** ya configuradas:

| App | Search Targets | URL Patterns | Priority |
|-----|----------------|--------------|----------|
| 🎬 Google Meet | Meet | meet.google.com | High |
| 💼 Teams | Teams | teams.microsoft.com | High |
| 🎮 Discord | Discord | discord.com | High |
| 💬 Slack | Slack | app.slack.com | High |
| 📱 WhatsApp | WhatsApp | web.whatsapp.com | Normal |
| 📞 Telegram | Telegram | web.telegram.org | Normal |
| 💬 Google Chat | Chat | chat.google.com | Normal |

---

## 🚀 Uso Básico desde TypeScript

### 1️⃣ Validar si una app debe procesarse

```typescript
import { invoke } from '@tauri-apps/api/core';

// Verificar si Teams puede procesarse
const shouldProcess = await invoke('should_process_app', {
  appName: 'Teams'
});

if (shouldProcess) {
  // ✅ Proceder con Vision + TTS
} else {
  // ❌ Solo mostrar notificación visual
}
```

### 2️⃣ Obtener contexto completo de una app

```typescript
const context = await invoke('validate_app_context', {
  appName: 'Google Meet'
});

// Resultado:
// {
//   app_name: "Google Meet",
//   search_targets: ["Meet", "Google Meet"],
//   url_patterns: ["meet.google.com"],
//   css_selector: "...",
//   priority: "High",
//   enabled: true
// }
```

### 3️⃣ Obtener targets de búsqueda sugeridos

```typescript
const targets = await invoke('get_search_targets', {
  appName: 'Slack'
});

// Resultado: ["Slack", "app.slack.com"]

// Usar con Vision module:
const result = await invoke('get_active_tab_context', {
  targetName: targets[0]  // Buscar la primera opción
});
```

---

## 🔄 Flujo Integrado: Notificación → Context Mapper → Vision → TTS

```
┌─────────────────────────────────────────────────────────────┐
│ Notificación llega: { app: "Teams", message: "Hey!" }      │
└────────────────────────┬──────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ PASO 1: Validar Context Mapper                              │
│ invoke('should_process_app', { appName: 'Teams' })         │
└────────────────────────┬──────────────────────────────────┘
                         ↓
              ¿Retorna true? ✅
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ PASO 2: Obtener search targets                              │
│ invoke('get_search_targets', { appName: 'Teams' })         │
│ Retorna: ["Teams", "Microsoft Teams"]                       │
└────────────────────────┬──────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ PASO 3: Buscar pestaña en Vision                            │
│ invoke('get_active_tab_context', { targetName: 'Teams' })  │
│ Retorna: { content: "actual chat messages", ... }          │
└────────────────────────┬──────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ PASO 4: Reproducir con TTS                                  │
│ invoke('plugin:tts|speak', {                                │
│   text: "Teams: hey!",                                      │
│   lang: "es"                                                │
│ })                                                           │
└─────────────────────────────────────────────────────────────┘
```

---

## 💻 Implementación Práctica

### Hook React: useContextValidation

```typescript
// src/hooks/useContextValidation.ts
import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface ValidationResult {
  isValid: boolean;
  shouldProcess: boolean;
  searchTargets: string[];
  appName: string;
  error?: string;
}

export function useContextValidation() {
  const [validation, setValidation] = useState<ValidationResult | null>(null);
  const [loading, setLoading] = useState(false);

  const validate = async (appName: string) => {
    setLoading(true);
    try {
      // Verificar si debe procesarse
      const shouldProcess = await invoke('should_process_app', {
        appName
      });

      // Obtener targets de búsqueda
      const searchTargets = await invoke('get_search_targets', {
        appName
      }).catch(() => []);

      setValidation({
        isValid: true,
        shouldProcess,
        searchTargets: searchTargets || [],
        appName,
      });
    } catch (error) {
      setValidation({
        isValid: false,
        shouldProcess: false,
        searchTargets: [],
        appName,
        error: error instanceof Error ? error.message : 'Unknown error',
      });
    } finally {
      setLoading(false);
    }
  };

  return {
    ...validation,
    validate,
    loading,
  };
}
```

**Uso:**
```typescript
export function NotificationProcessor({ notification }) {
  const { isValid, shouldProcess, searchTargets, validate } = useContextValidation();

  useEffect(() => {
    validate(notification.app);
  }, [notification.app]);

  if (!isValid) {
    return <div>⚠️ App no soportada</div>;
  }

  if (!shouldProcess) {
    return <div>📌 Solo notificación visual</div>;
  }

  return <div>✅ Procesando con Vision + TTS</div>;
}
```

---

## 🔧 Agregar una App Personalizada (Future API)

El Context Mapper es **extensible**. Para futuras versiones:

```rust
// En context_mapper.rs - se puede exponer como comando Tauri:

#[tauri::command]
fn register_custom_app(
    app_name: String,
    search_targets: Vec<String>,
    url_patterns: Vec<String>,
    css_selector: String,
    priority: String,
) -> Result<(), String> {
    let mut mapper = ContextMapper::new();

    let priority = match priority.as_str() {
        "High" => TaskPriority::High,
        "Normal" => TaskPriority::Normal,
        "Low" => TaskPriority::Low,
        _ => TaskPriority::Normal,
    };

    mapper.register_app(app_name, search_targets, url_patterns, css_selector, priority);
    Ok(())
}
```

---

## 📋 Comparación: Con vs Sin Context Mapper

### ❌ SIN Context Mapper (enfoque anterior)

```typescript
// Notificación de Spotify → ¡Intentar buscar en Chrome!
// Notificación de Outlook → ¡Intentar buscar en Chrome!
// Notificación de Whatsapp Desktop → ¡Intentar buscar en Chrome!
// Resultado: Muchos errores innecesarios
```

### ✅ CON Context Mapper

```typescript
// Notificación de Spotify
await invoke('should_process_app', { appName: 'Spotify' });
// Retorna: false → Solo mostrar notificación visual

// Notificación de Teams
await invoke('should_process_app', { appName: 'Teams' });
// Retorna: true → Procesar con Vision + TTS

// Notificación de Gmail
await invoke('should_process_app', { appName: 'Gmail' });
// Retorna: false → Solo mostrar notificación visual
```

---

## 🎛️ Filtro por Prioridad

El Context Mapper permite **filtrar por prioridad**. Ejemplo:

```typescript
// En futuras versiones, el backend podría hacer:
// "Procesar SOLO apps con prioridad High o Critical"

// Mientras tanto, el frontend puede decidir:
if (shouldProcess && priority === 'High') {
  // ⚡ Procesar inmediatamente con TTS
  speakNotification();
} else if (shouldProcess) {
  // 🔄 Poner en cola para procesar después
  queueNotification();
}
```

---

## 📊 Casos de Uso Reales

### Caso 1: Usuario recibe notificación de Teams
```typescript
const notification = {
  app: 'Microsoft Teams',
  title: 'John Doe',
  message: 'Hello team!'
};

// Validar
const isSupported = await invoke('should_process_app', {
  appName: notification.app
}); // ✅ true

// Si Teams está abierto → procesar con Vision
// Si Teams NO está abierto → solo mostrar visual
```

### Caso 2: Usuario recibe notificación de Spotify
```typescript
const notification = {
  app: 'Spotify',
  title: 'Song ended',
  message: 'Next song playing...'
};

// Validar
const isSupported = await invoke('should_process_app', {
  appName: notification.app
}); // ❌ false (Spotify no está en el mapper)

// Resultado: Solo mostrar notificación visual
// No intentar buscar Spotify en Chrome
```

### Caso 3: Usuario recibe notificación de Google Chat
```typescript
const notification = {
  app: 'Google Chat',
  message: 'New message'
};

// Validar
const isSupported = await invoke('should_process_app', {
  appName: notification.app
}); // ✅ true

// Obtener targets
const targets = await invoke('get_search_targets', {
  appName: notification.app
}); // ["Chat", "Google Chat"]

// Buscar pestaña con Vision
const content = await invoke('get_active_tab_context', {
  targetName: targets[0]
});
```

---

## 🧪 Testing

El módulo incluye tests:

```bash
cargo test context_mapper::tests
```

Tests incluidos:
- ✅ `test_context_mapper_creation` - Mapeos por defecto
- ✅ `test_find_context_case_insensitive` - Búsqueda insensible a mayúsculas
- ✅ `test_get_enabled_contexts` - Obtener solo habilitados
- ✅ `test_priority_filtering` - Filtrado por prioridad
- ✅ `test_validator_should_process` - Validador funciona
- ✅ `test_validator_get_search_targets` - Obtener targets
- ✅ `test_custom_app_registration` - Registrar apps personalizadas

---

## 🔐 Seguridad

- ✅ **Validación de entrada:** Cada app se valida antes de procesar
- ✅ **Selectores CSS seguros:** Pre-validados, no eval() dinámico
- ✅ **Sin ejecución de código:** Los selectores JS son estáticos
- ✅ **Aislamiento:** Solo accede a pestañas que coinciden con patrones

---

## 📚 Ficheros

| Archivo | Descripción |
|---------|-------------|
| `src-tauri/src/context_mapper.rs` | Implementación (~360 líneas) |
| `src-tauri/src/lib.rs` | Registración de módulo y comandos |
| `CONTEXT_MAPPER_GUIDE.md` | Este documento |
| `CONTEXT_MAPPER_EXAMPLES.md` | Ejemplos prácticos |

---

## 📈 Roadmap

- [ ] Panel de administración de apps
- [ ] Editar prioridades desde UI
- [ ] Agregar/remover apps dinámicamente
- [ ] Persistir configuración personalizada
- [ ] Sincronizar con servidor
- [ ] Machine Learning para detectar nuevas apps

---

## ✅ Checklist de Integración

- [x] Context Mapper creado
- [x] Módulo registrado en lib.rs
- [x] Comandos Tauri expuestos (3 comandos)
- [x] Tests incluidos
- [x] Documentación completa
- [ ] Integrar en flujo de notificaciones
- [ ] Crear UI para administrar apps
- [ ] Añadir más apps según necesidad

---

**Status:** ✅ Implementado y compilado
**Apps soportadas:** 7 (Meet, Teams, Discord, Slack, WhatsApp, Telegram, Google Chat)
**Extensible:** Sí, mediante `register_app()`
**Thread-safe:** Sí (HashMap + métodos inmutables por defecto)

---

## 🎯 Próximo Paso

Lee **CONTEXT_MAPPER_EXAMPLES.md** para ver ejemplos prácticos de integración.
