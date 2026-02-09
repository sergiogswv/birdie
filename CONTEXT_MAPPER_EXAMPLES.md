# Context Mapper - Ejemplos de Integración

## 📍 Ejemplo 1: Validación Simple (Más recomendado para empezar)

```typescript
// src/utils/notificationProcessor.ts
import { invoke } from '@tauri-apps/api/core';

export async function shouldProcessNotification(appName: string): Promise<boolean> {
  try {
    return await invoke('should_process_app', { appName });
  } catch (error) {
    console.error('Error validating app:', error);
    return false; // Si hay error, no procesar
  }
}
```

**Uso en componente:**
```typescript
import { shouldProcessNotification } from '../utils/notificationProcessor';
import { useNotifications } from '../hooks/useNotifications';

export function NotificationListener() {
  const { currentNotification } = useNotifications();

  useEffect(() => {
    if (!currentNotification) return;

    (async () => {
      const shouldProcess = await shouldProcessNotification(
        currentNotification.app
      );

      if (shouldProcess) {
        console.log('✅ Procesando con Vision + TTS');
        // Aquí llamar a Vision module
        // const content = await invoke('get_active_tab_context', ...)
      } else {
        console.log('ℹ️ Solo mostrar notificación visual');
      }
    })();
  }, [currentNotification]);

  return null;
}
```

---

## 📍 Ejemplo 2: Flujo Completo (Notificación → Context → Vision → TTS)

```typescript
// src/services/notificationService.ts
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

interface NotificationEvent {
  app: string;
  sender: string;
  message: string;
  timestamp: string;
}

export async function startNotificationProcessor() {
  // Escuchar notificaciones
  await listen('notification-received', async (event) => {
    const notification = event.payload as NotificationEvent;

    // ✅ PASO 1: Validar en Context Mapper
    const shouldProcess = await invoke('should_process_app', {
      appName: notification.app
    }).catch(() => false);

    if (!shouldProcess) {
      console.log(`⏭️ ${notification.app} no está soportado, mostrar solo visual`);
      return;
    }

    // ✅ PASO 2: Obtener targets de búsqueda
    const searchTargets = await invoke('get_search_targets', {
      appName: notification.app
    }).catch(() => []);

    if (!searchTargets || searchTargets.length === 0) {
      console.log('⚠️ No hay targets para', notification.app);
      return;
    }

    // ✅ PASO 3: Buscar pestaña con Vision
    const visionResult = await invoke('get_active_tab_context', {
      targetName: searchTargets[0]  // Usar el primer target sugerido
    }).catch((error) => {
      console.error('Vision error:', error);
      return null;
    });

    if (!visionResult || !visionResult.success) {
      console.log('❌ No se pudo extraer contenido de la pestaña');
      return;
    }

    // ✅ PASO 4: Reproducir con TTS
    const textToSpeak = `Notificación de ${notification.app}. ${notification.sender} dice: ${notification.message}`;

    try {
      await invoke('plugin:tts|speak', {
        payload: {
          text: textToSpeak,
          lang: 'es'
        }
      });
    } catch (error) {
      console.error('TTS error:', error);
    }
  });
}
```

**Inicializar en App.tsx:**
```typescript
import { startNotificationProcessor } from './services/notificationService';
import { useEffect } from 'react';

export function App() {
  useEffect(() => {
    startNotificationProcessor();
  }, []);

  return <div>...</div>;
}
```

---

## 📍 Ejemplo 3: Hook Personalizado

```typescript
// src/hooks/useNotificationProcessor.ts
import { useState, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface ProcessResult {
  app: string;
  isSupported: boolean;
  searchTargets: string[];
  content?: string;
  tabTitle?: string;
  error?: string;
}

export function useNotificationProcessor() {
  const [result, setResult] = useState<ProcessResult | null>(null);
  const [loading, setLoading] = useState(false);

  const processNotification = useCallback(async (appName: string) => {
    setLoading(true);
    try {
      // Paso 1: Validar
      const isSupported = await invoke('should_process_app', { appName });

      if (!isSupported) {
        setResult({
          app: appName,
          isSupported: false,
          searchTargets: [],
          error: 'App no soportada',
        });
        return;
      }

      // Paso 2: Obtener targets
      const searchTargets = await invoke('get_search_targets', { appName });

      // Paso 3: Buscar pestaña
      const content = await invoke('get_active_tab_context', {
        targetName: searchTargets[0],
      });

      setResult({
        app: appName,
        isSupported: true,
        searchTargets,
        content: content.content,
        tabTitle: content.tab_title,
      });
    } catch (error) {
      setResult({
        app: appName,
        isSupported: false,
        searchTargets: [],
        error: error instanceof Error ? error.message : 'Unknown error',
      });
    } finally {
      setLoading(false);
    }
  }, []);

  return {
    processNotification,
    result,
    loading,
  };
}
```

**Uso:**
```typescript
export function MyComponent() {
  const { processNotification, result, loading } = useNotificationProcessor();

  const handleClick = async () => {
    await processNotification('Google Meet');
  };

  return (
    <div>
      <button onClick={handleClick} disabled={loading}>
        {loading ? '⏳' : '📥'} Procesar Teams
      </button>

      {result && (
        <div>
          <p>App soportada: {result.isSupported ? '✅' : '❌'}</p>
          {result.content && <pre>{result.content}</pre>}
          {result.error && <p className="error">{result.error}</p>}
        </div>
      )}
    </div>
  );
}
```

---

## 📍 Ejemplo 4: Integración con Sistema de Notificaciones Existente

```typescript
// src/components/NotificationHandler.tsx
import { useNotifications } from '../hooks/useNotifications';
import { useNotificationProcessor } from '../hooks/useNotificationProcessor';
import { useEffect } from 'react';

export function NotificationHandler() {
  const { currentNotification } = useNotifications();
  const { processNotification, result, loading } = useNotificationProcessor();

  useEffect(() => {
    if (!currentNotification) return;

    // Procesar automáticamente cuando llega notificación
    processNotification(currentNotification.app);
  }, [currentNotification?.id]);

  if (!currentNotification) {
    return null;
  }

  return (
    <div className="notification-handler">
      <h3>{currentNotification.app}</h3>
      <p>Desde: {currentNotification.sender}</p>
      <p>Mensaje: {currentNotification.message}</p>

      {loading && <p>⏳ Validando y extrayendo contenido...</p>}

      {result && result.isSupported && result.content && (
        <div className="extracted-content">
          <h4>📄 Contenido extraído:</h4>
          <pre>{result.content.substring(0, 200)}...</pre>
        </div>
      )}

      {result && !result.isSupported && (
        <p className="info">ℹ️ Esta aplicación no tiene mapeo para Context Scanner</p>
      )}

      {result?.error && (
        <p className="error">❌ {result.error}</p>
      )}
    </div>
  );
}
```

---

## 📍 Ejemplo 5: Panel de Control de Apps

```typescript
// src/components/ContextManagerPanel.tsx
import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

const AVAILABLE_APPS = [
  'Google Meet',
  'Microsoft Teams',
  'Discord',
  'Slack',
  'WhatsApp',
  'Telegram',
  'Google Chat',
];

interface AppStatus {
  name: string;
  isSupported: boolean;
  searchTargets: string[];
}

export function ContextManagerPanel() {
  const [appStatuses, setAppStatuses] = useState<AppStatus[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    (async () => {
      const statuses = await Promise.all(
        AVAILABLE_APPS.map(async (app) => ({
          name: app,
          isSupported: await invoke('should_process_app', { appName: app }).catch(() => false),
          searchTargets: await invoke('get_search_targets', { appName: app }).catch(() => []),
        }))
      );
      setAppStatuses(statuses);
      setLoading(false);
    })();
  }, []);

  return (
    <div className="context-manager-panel">
      <h2>⚙️ Aplicaciones Soportadas</h2>

      {loading ? (
        <p>⏳ Cargando...</p>
      ) : (
        <div className="app-list">
          {appStatuses.map((app) => (
            <div key={app.name} className="app-item">
              <h3>
                {app.isSupported ? '✅' : '❌'} {app.name}
              </h3>
              {app.isSupported && app.searchTargets.length > 0 && (
                <p>
                  <strong>Búsqueda:</strong> {app.searchTargets.join(', ')}
                </p>
              )}
              {!app.isSupported && (
                <p className="disabled">No configurada para Context Scanning</p>
              )}
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
```

**Estilos:**
```css
.context-manager-panel {
  padding: 2rem;
  background: white;
  border-radius: 1rem;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.app-list {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
  gap: 1rem;
  margin-top: 1rem;
}

.app-item {
  padding: 1rem;
  border: 1px solid #ddd;
  border-radius: 0.5rem;
  background: #f9fafb;
}

.app-item h3 {
  margin: 0 0 0.5rem 0;
  font-size: 1.1rem;
}

.app-item.disabled {
  opacity: 0.6;
}
```

---

## 📍 Ejemplo 6: Flujo de Error Robusto

```typescript
// src/services/safeNotificationProcessor.ts
import { invoke } from '@tauri-apps/api/core';

interface SafeProcessResult {
  success: boolean;
  reason: 'not-supported' | 'no-tab-found' | 'tab-error' | 'success';
  content?: string;
  error?: string;
}

export async function safeProcessNotification(
  appName: string
): Promise<SafeProcessResult> {
  try {
    // Paso 1: Validar si está soportada
    const isSupported = await invoke('should_process_app', { appName }).catch(() => false);

    if (!isSupported) {
      return {
        success: false,
        reason: 'not-supported',
        error: `${appName} no está configurada en Context Mapper`,
      };
    }

    // Paso 2: Obtener targets
    const searchTargets = await invoke('get_search_targets', { appName }).catch(() => []);

    if (!searchTargets || searchTargets.length === 0) {
      return {
        success: false,
        reason: 'no-tab-found',
        error: `No hay targets de búsqueda para ${appName}`,
      };
    }

    // Paso 3: Buscar en Vision
    const visionResult = await invoke('get_active_tab_context', {
      targetName: searchTargets[0],
    }).catch((error) => ({
      success: false,
      error: error instanceof Error ? error.message : 'Unknown error',
    }));

    if (!visionResult.success) {
      return {
        success: false,
        reason: 'tab-error',
        error: visionResult.error || 'No se encontró la pestaña',
      };
    }

    return {
      success: true,
      reason: 'success',
      content: visionResult.content,
    };
  } catch (error) {
    return {
      success: false,
      reason: 'tab-error',
      error: error instanceof Error ? error.message : 'Unknown error',
    };
  }
}
```

**Uso:**
```typescript
const result = await safeProcessNotification('Teams');

if (result.success) {
  console.log('✅ Contenido extraído:', result.content);
} else {
  switch (result.reason) {
    case 'not-supported':
      console.log('ℹ️ App no soportada - mostrar solo visual');
      break;
    case 'no-tab-found':
      console.log('⚠️ No hay targets configurados');
      break;
    case 'tab-error':
      console.log('❌ Error al acceder a la pestaña:', result.error);
      break;
  }
}
```

---

## 📊 Flujo Completo Recomendado

```
Notificación recibida
  ↓
[1] ¿Está en Context Mapper? → No → Mostrar solo visual ✅
  ↓ Sí
[2] ¿Hay targets configurados? → No → Mostrar solo visual ✅
  ↓ Sí
[3] Buscar pestaña con Vision → No → Mostrar solo visual ✅
  ↓ Sí
[4] Extraer contenido
  ↓
[5] Reproducir con TTS ✅
```

**Código:**
```typescript
async function processNotificationCompletely(notification) {
  console.log(`📨 Procesando: ${notification.app}`);

  // [1] Context Mapper
  const isSupported = await invoke('should_process_app', {
    appName: notification.app
  }).catch(() => false);

  if (!isSupported) {
    console.log('ℹ️ [1] No soportada');
    showVisualNotification(notification);
    return;
  }

  // [2] Obtener targets
  const targets = await invoke('get_search_targets', {
    appName: notification.app
  }).catch(() => []);

  if (!targets?.length) {
    console.log('ℹ️ [2] Sin targets');
    showVisualNotification(notification);
    return;
  }

  // [3] Buscar pestaña
  const vision = await invoke('get_active_tab_context', {
    targetName: targets[0]
  }).catch(() => null);

  if (!vision?.success) {
    console.log('ℹ️ [3] Pestaña no encontrada');
    showVisualNotification(notification);
    return;
  }

  // [4] Extraer
  console.log('📄 [4] Contenido extraído');

  // [5] TTS
  await invoke('plugin:tts|speak', {
    payload: {
      text: notification.message,
      lang: 'es'
    }
  });

  console.log('✅ Completado');
}
```

---

## ✅ Casos de Uso Reales

### Caso A: Usuario en reunión de Teams
```
1. Llega notificación: "Juan: Hola equipo"
2. Context Mapper valida: ✅ Teams está soportada
3. Busca pestaña: ✅ teams.microsoft.com abierto
4. Extrae contexto: "El equipo discute Q2 roadmap"
5. Reproduce: "Notificación de Teams. Juan dice: Hola equipo"
```

### Caso B: Usuario recibe notificación de Spotify
```
1. Llega notificación: "Song ended"
2. Context Mapper valida: ❌ Spotify no está en el mapeo
3. Resultado: Solo mostrar notificación visual
4. NO intenta buscar en Chrome
```

### Caso C: Usuario en Discord pero solo una tab
```
1. Llega notificación: "Server alert"
2. Context Mapper valida: ✅ Discord está soportada
3. Busca pestaña: ❌ discord.com no está abierto
4. Vision retorna error
5. Resultado: Solo mostrar notificación visual
```

---

## 🎯 Integration Checklist

- [ ] Importar comandos del Context Mapper
- [ ] Crear `useNotificationProcessor` hook
- [ ] Integrar validación antes de Vision
- [ ] Agregar error handling robusto
- [ ] Crear panel de administración de apps
- [ ] Documentar selectores CSS para cada app
- [ ] Testear con múltiples apps reales

---

**Tips Finales:**
1. **Siempre validar** antes de llamar a Vision
2. **Cachejar resultados** si se repite la misma app
3. **Loguear decisiones** para debugging
4. **Mostrar razón** si algo falla al usuario

