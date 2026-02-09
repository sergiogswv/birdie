# Ejemplos de Uso del Módulo Vision

## 🚀 Quick Start - Copia y Pega

### 1. Verifica que ya esté registrado en `lib.rs`:

```rust
// src-tauri/src/lib.rs - Ya debe estar así:

mod notifications;
mod stt;
mod cdp;
mod vision;  // ✅ Agrégalo aquí si no existe

// En la macro generate_handler:
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
    vision::get_active_tab_context  // ✅ Agrégalo aquí si no existe
])
```

---

## 📍 Ejemplo 1: Extraer contenido de Google Meet

```typescript
// src/utils/visionAPI.ts
import { invoke } from '@tauri-apps/api/core';

export async function extractMeetContent() {
  try {
    const result = await invoke('get_active_tab_context', {
      targetName: 'Google Meet'
    });

    return result;
  } catch (error) {
    console.error('Error:', error);
    throw error;
  }
}
```

**Uso en componente:**
```typescript
import { extractMeetContent } from '../utils/visionAPI';
import { useState } from 'react';

export function MeetExtractor() {
  const [content, setContent] = useState('');
  const [loading, setLoading] = useState(false);

  const handleClick = async () => {
    setLoading(true);
    const result = await extractMeetContent();
    setContent(result.content);
    setLoading(false);
  };

  return (
    <div>
      <button onClick={handleClick} disabled={loading}>
        {loading ? 'Extrayendo...' : 'Extraer de Meet'}
      </button>
      <pre>{content}</pre>
    </div>
  );
}
```

---

## 📍 Ejemplo 2: Función genérica para cualquier plataforma

```typescript
// src/utils/visionAPI.ts
import { invoke } from '@tauri-apps/api/core';

export interface ContextResult {
  success: boolean;
  content: string;
  tab_title: string;
  tab_url: string;
  error?: string;
}

export async function extractTabContent(
  tabName: string
): Promise<ContextResult> {
  try {
    const result: ContextResult = await invoke(
      'get_active_tab_context',
      { targetName: tabName }
    );

    if (!result.success) {
      throw new Error(result.error || 'Unknown error');
    }

    return result;
  } catch (error) {
    console.error(`Error extrayendo de ${tabName}:`, error);
    throw error;
  }
}

// Casos de uso específicos
export const extractContent = {
  meet: () => extractTabContent('Google Meet'),
  teams: () => extractTabContent('Teams'),
  discord: () => extractTabContent('Discord'),
  whatsapp: () => extractTabContent('WhatsApp'),
  telegram: () => extractTabContent('Telegram'),
  custom: (name: string) => extractTabContent(name),
};
```

**Uso:**
```typescript
// Más simple
const meetContent = await extractContent.meet();
const teamsContent = await extractContent.teams();
const customContent = await extractContent.custom('Mi App');
```

---

## 📍 Ejemplo 3: Hook personalizado

```typescript
// src/hooks/useVisionExtract.ts
import { useState, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface ContextResult {
  success: boolean;
  content: string;
  tab_title: string;
  tab_url: string;
  error?: string;
}

interface UseVisionState {
  content: string;
  loading: boolean;
  error: string | null;
  tabTitle: string;
  tabUrl: string;
}

export function useVisionExtract() {
  const [state, setState] = useState<UseVisionState>({
    content: '',
    loading: false,
    error: null,
    tabTitle: '',
    tabUrl: '',
  });

  const extract = useCallback(async (targetName: string) => {
    setState((s) => ({ ...s, loading: true, error: null }));

    try {
      const result: ContextResult = await invoke(
        'get_active_tab_context',
        { targetName }
      );

      if (result.success) {
        setState({
          content: result.content,
          loading: false,
          error: null,
          tabTitle: result.tab_title,
          tabUrl: result.tab_url,
        });
      } else {
        setState((s) => ({
          ...s,
          loading: false,
          error: result.error || 'Error desconocido',
        }));
      }
    } catch (error) {
      setState((s) => ({
        ...s,
        loading: false,
        error: error instanceof Error ? error.message : 'Error desconocido',
      }));
    }
  }, []);

  return {
    ...state,
    extract,
  };
}
```

**Uso en componente:**
```typescript
export function MyComponent() {
  const { content, loading, error, extract } = useVisionExtract();

  return (
    <div>
      <button onClick={() => extract('Google Meet')}>
        {loading ? '⏳ Cargando...' : '🎬 Extraer Meet'}
      </button>
      {error && <div className="error">{error}</div>}
      {content && <pre>{content}</pre>}
    </div>
  );
}
```

---

## 📍 Ejemplo 4: Panel completo de Vision

```typescript
// src/components/VisionPanel.tsx
import { useState } from 'react';
import { useVisionExtract } from '../hooks/useVisionExtract';

const PLATFORMS = [
  { name: 'Google Meet', icon: '🎬' },
  { name: 'Teams', icon: '💼' },
  { name: 'Discord', icon: '🎮' },
  { name: 'WhatsApp', icon: '💬' },
  { name: 'Telegram', icon: '📱' },
];

export function VisionPanel() {
  const { content, loading, error, tabTitle, extract } = useVisionExtract();
  const [customPlatform, setCustomPlatform] = useState('');

  return (
    <div className="vision-panel">
      <h2>👁️ Vision - Extractor de Contenido</h2>

      <div className="platform-buttons">
        {PLATFORMS.map((platform) => (
          <button
            key={platform.name}
            onClick={() => extract(platform.name)}
            disabled={loading}
          >
            {platform.icon} {platform.name}
          </button>
        ))}
      </div>

      <div className="custom-input">
        <input
          value={customPlatform}
          onChange={(e) => setCustomPlatform(e.target.value)}
          placeholder="O escribe el nombre de tu pestaña..."
        />
        <button
          onClick={() => extract(customPlatform)}
          disabled={loading || !customPlatform}
        >
          Extraer
        </button>
      </div>

      {error && (
        <div className="error-banner">
          <strong>❌ Error:</strong> {error}
        </div>
      )}

      {content && (
        <div className="result-panel">
          <div className="result-header">
            <h3>📄 Resultado</h3>
            <small>
              De: <strong>{tabTitle}</strong>
            </small>
          </div>
          <pre className="result-content">{content}</pre>
          <button
            onClick={() => navigator.clipboard.writeText(content)}
            className="copy-btn"
          >
            📋 Copiar al portapapeles
          </button>
        </div>
      )}

      {loading && (
        <div className="loading-spinner">
          ⏳ Extrayendo contenido...
        </div>
      )}
    </div>
  );
}
```

**CSS:**
```css
.vision-panel {
  padding: 2rem;
  border-radius: 1rem;
  background: white;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.vision-panel h2 {
  margin-bottom: 1rem;
  color: #333;
}

.platform-buttons {
  display: flex;
  gap: 1rem;
  flex-wrap: wrap;
  margin-bottom: 1rem;
}

.platform-buttons button {
  padding: 0.75rem 1.5rem;
  border: 1px solid #ddd;
  border-radius: 0.5rem;
  cursor: pointer;
  transition: all 0.2s;
}

.platform-buttons button:hover:not(:disabled) {
  border-color: #667eea;
  background: #f5f7ff;
}

.platform-buttons button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.custom-input {
  display: flex;
  gap: 0.5rem;
  margin-bottom: 1rem;
}

.custom-input input {
  flex: 1;
  padding: 0.75rem;
  border: 1px solid #ddd;
  border-radius: 0.5rem;
  font-size: 1rem;
}

.custom-input button {
  padding: 0.75rem 1.5rem;
  background: #667eea;
  color: white;
  border: none;
  border-radius: 0.5rem;
  cursor: pointer;
}

.error-banner {
  padding: 1rem;
  background: #fee2e2;
  border: 1px solid #fecaca;
  border-radius: 0.5rem;
  color: #991b1b;
  margin-bottom: 1rem;
}

.result-panel {
  margin-top: 1rem;
  border: 1px solid #e5e7eb;
  border-radius: 0.5rem;
  overflow: hidden;
}

.result-header {
  padding: 1rem;
  background: #f9fafb;
  border-bottom: 1px solid #e5e7eb;
}

.result-content {
  padding: 1rem;
  max-height: 400px;
  overflow-y: auto;
  background: #f5f5f5;
  margin: 0;
  white-space: pre-wrap;
  word-wrap: break-word;
}

.copy-btn {
  width: 100%;
  padding: 0.75rem;
  background: #10b981;
  color: white;
  border: none;
  cursor: pointer;
  font-weight: 600;
}

.copy-btn:hover {
  background: #059669;
}

.loading-spinner {
  text-align: center;
  padding: 2rem;
  color: #666;
}
```

---

## 📍 Ejemplo 5: Integración con notificaciones

```typescript
// Combinar Vision con el sistema de notificaciones existente

import { useVisionExtract } from '../hooks/useVisionExtract';
import { useNotifications } from '../hooks/useNotifications';

export function AutoExtractOnNotification() {
  const { queue, currentNotification } = useNotifications();
  const { extract } = useVisionExtract();

  // Cuando llega una notificación de Teams, extrae automáticamente el contenido
  useEffect(() => {
    if (currentNotification?.app === 'Teams') {
      extract('Teams');
    }
  }, [currentNotification]);

  return null; // Hook silencioso
}
```

---

## 🔧 Solución de Problemas

### Error: "No se pudo conectar a Chrome en puerto 9222"

**Solución:**
1. Abre Chrome con debug mode:
   ```bash
   "C:\Program Files\Google\Chrome\Application\chrome.exe" --remote-debugging-port=9222
   ```

2. Verifica que el puerto 9222 sea accesible:
   ```bash
   netstat -an | findstr 9222  # Windows
   lsof -i :9222                # macOS/Linux
   ```

### Error: "No se encontró pestaña que contenga..."

**Solución:**
- Verifica que la pestaña esté abierta en Chrome
- Usa un nombre más genérico (ej: "Meet" en lugar de "Google Meet")
- Abre el DevTools y copia el título exacto de la pestaña

### Script falla silenciosamente

**Solución:**
- Los selectores CSS pueden haber cambiado en las web apps
- Usa el inspector del navegador para encontrar los selectores correctos
- Reporta en el módulo para actualizar los selectores

---

## ✅ Checklist de Integración

- [ ] Archivo `vision.rs` creado en `src-tauri/src/`
- [ ] `mod vision;` agregado a `lib.rs`
- [ ] `vision::get_active_tab_context` agregado a `invoke_handler!`
- [ ] `cargo check` compila sin errores
- [ ] Chrome abierto con `--remote-debugging-port=9222`
- [ ] TypeScript types creados
- [ ] Hook `useVisionExtract` creado
- [ ] Componente de UI integrado
- [ ] Pruebas manuales exitosas

---

**¿Necesitas ayuda con algún ejemplo?** Pregunta y te lo adapto.
