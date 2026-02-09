# Vision Module - Quick Reference Card

## 📍 3 pasos para usar

### ✅ Paso 1: Verificar Registro (YA HECHO)
```rust
// src-tauri/src/lib.rs
mod vision;  // ✅ Ya existe

.invoke_handler(tauri::generate_handler![
    ...
    vision::get_active_tab_context  // ✅ Ya está registrado
])
```

### ✅ Paso 2: Llamar desde TypeScript
```typescript
import { invoke } from '@tauri-apps/api/core';

const result = await invoke('get_active_tab_context', {
  targetName: 'Google Meet'  // O "Teams", "Discord", etc.
});

console.log(result.content);  // Texto extraído
console.log(result.success);  // true/false
```

### ✅ Paso 3: Abrir Chrome en debug mode
```bash
# Windows
"C:\Program Files\Google\Chrome\Application\chrome.exe" --remote-debugging-port=9222

# macOS
/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome --remote-debugging-port=9222

# Linux
google-chrome --remote-debugging-port=9222
```

---

## 🎯 Respuesta de la función

```typescript
interface ContextResult {
  success: boolean;           // ✅ o ❌
  content: string;           // Texto extraído
  tab_title: string;        // Título de la pestaña
  tab_url: string;          // URL de la pestaña
  error?: string;           // Mensaje de error (si success=false)
}
```

---

## 🌐 Plataformas soportadas

| Plataforma | Buscar por | Qué extrae |
|-----------|-----------|-----------|
| 🎬 Google Meet | `"Meet"` o `"Google Meet"` | Mensajes de chat + Participantes |
| 💼 Teams | `"Teams"` | Mensajes de chat |
| 🎮 Discord | `"Discord"` | Canal + Mensajes |
| 💬 WhatsApp | `"WhatsApp"` | Mensajes |
| 📱 Telegram | `"Telegram"` | Mensajes |
| 🌍 Otras | Cualquier nombre | Todo el texto visible |

---

## 💻 Copiar-Pega (React Component)

```typescript
import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

export function QuickVision() {
  const [content, setContent] = useState('');
  const [loading, setLoading] = useState(false);

  const extract = async (platform: string) => {
    setLoading(true);
    const result = await invoke('get_active_tab_context', { targetName: platform });
    setContent(result.content);
    setLoading(false);
  };

  return (
    <div>
      <button onClick={() => extract('Meet')} disabled={loading}>
        📥 Extraer
      </button>
      <pre>{content}</pre>
    </div>
  );
}
```

---

## 🔧 Configuración en lib.rs

**Ya está hecho** ✅, pero si no existe, agrega:

```rust
// Línea 4 (después de mod cdp;)
mod vision;

// En invoke_handler (última línea de generate_handler!)
vision::get_active_tab_context
```

---

## ❌ Errores comunes

| Error | Solución |
|-------|----------|
| "No se pudo conectar a Chrome en puerto 9222" | Abre Chrome con `--remote-debugging-port=9222` |
| "No se encontró pestaña que contenga..." | Verifica que la pestaña esté abierta en Chrome |
| "Error al ejecutar script" | Los selectores CSS pueden haber cambiado |

---

## 📊 Archivos incluidos

```
src-tauri/src/vision.rs          ← Implementación Rust (~260 líneas)
VISION_MODULE_GUIDE.md           ← Guía completa detallada
VISION_EXAMPLES.md               ← 5 ejemplos listos para copiar
VISION_QUICK_REFERENCE.md        ← Este archivo (resumen)
```

---

## 🚀 Próximos pasos

```typescript
// 1. Crear hook personalizado
const { content, extract, loading } = useVisionExtract();

// 2. Integrar en panel
<VisionPanel />

// 3. Combinar con notificaciones
// Cuando llega una notificación de Teams, extrae automáticamente

// 4. Guardar historia de extracciones
const [history, setHistory] = useState([]);
```

---

## ✨ Tips

- ⚡ **Rápido:** Ejecución en paralelo con otras operaciones
- 🔒 **Seguro:** Solo accede a tabs locales en el mismo dispositivo
- 📱 **Flexible:** Soporta cualquier plataforma con selectores CSS
- 🎯 **Preciso:** Búsqueda case-insensitive de pestañas

---

## 📚 Referencias rápidas

| Necesito... | Archivo |
|------------|---------|
| Ver código Rust | `src-tauri/src/vision.rs` |
| Entender cómo funciona | `VISION_MODULE_GUIDE.md` |
| Copiar código TypeScript | `VISION_EXAMPLES.md` |
| Resumen rápido | `VISION_QUICK_REFERENCE.md` (aquí) |

---

## ✅ Checklist (2 minutos)

- [ ] Chrome abierto con `--remote-debugging-port=9222`
- [ ] `cargo check` compila ✅
- [ ] Importar `invoke` en TypeScript
- [ ] Llamar `invoke('get_active_tab_context', { targetName: 'Meet' })`
- [ ] ¡Funciona! 🎉

---

**¿Lista para usar? Abre Chrome y prueba ahora.**
