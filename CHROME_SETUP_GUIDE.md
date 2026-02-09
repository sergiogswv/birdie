# 🌐 Chrome Setup Guide - Conectar Chrome a Birdie

## 🚨 El Problema

Cuando intentas usar Vision Module, obtienes:
```
Error: No se pudo conectar a Chrome en puerto 9222
```

**Razón:** Chrome no está escuchando en el puerto 9222 (debug port).

---

## ✅ Solución Rápida (1 minuto)

### Windows

**Opción 1: Línea de comando (Recomendado)**

1. Abre **PowerShell** o **CMD** como administrador
2. Ejecuta:

```powershell
"C:\Program Files\Google\Chrome\Application\chrome.exe" --remote-debugging-port=9222
```

**Opción 2: Crear un atajo (.bat)**

Crea un archivo `chrome-debug.bat` en tu escritorio:

```batch
@echo off
"C:\Program Files\Google\Chrome\Application\chrome.exe" --remote-debugging-port=9222
```

Luego solo haz doble clic.

---

### macOS

```bash
/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome --remote-debugging-port=9222
```

O crea un script `chrome-debug.sh`:
```bash
#!/bin/bash
/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome --remote-debugging-port=9222
```

---

### Linux

```bash
google-chrome --remote-debugging-port=9222
```

O en algunas distribuciones:
```bash
chromium --remote-debugging-port=9222
```

---

## 🔍 Verificar Conexión

### Opción 1: Acceder vía navegador

Una vez abierto Chrome con debug port, abre en CUALQUIER navegador:

```
http://localhost:9222
```

Deberías ver algo como:

```json
{
  "Browser": "Chrome/...",
  "Protocol-Version": "1.3",
  "User-Agent": "Mozilla/...",
  "V8-Version": "...",
  "WebKit-Version": "...",
  "webSocketDebuggerUrl": "ws://localhost:9222/devtools/browser"
}
```

✅ Si ves esto → Chrome está correctamente configurado

---

### Opción 2: Usar netstat (Windows)

```bash
netstat -ano | findstr :9222
```

Deberías ver una línea con estado `LISTENING`:

```
  TCP    127.0.0.1:9222            0.0.0.0:0              LISTENING       12345
```

✅ Ahora puedes usar Vision Module

---

### Opción 3: Usar lsof (macOS/Linux)

```bash
lsof -i :9222
```

Deberías ver:

```
COMMAND   PID  USER   FD  TYPE DEVICE SIZE/OFF NODE NAME
chrome  12345 user  100u  IPv4  0x...      0t0  TCP localhost:9222 (LISTEN)
```

---

## 🎯 Cosas Importantes

### ⚠️ Debes abrir CADA VEZ

Cada vez que reinicies tu computadora o cierres Chrome, debes abrir Chrome nuevamente con el flag. No es una configuración permanente.

### ✅ Puedes navegar normalmente

Chrome funciona completamente normal con `--remote-debugging-port=9222`. Puedes:
- Abrir múltiples pestañas
- Navegar a cualquier sitio
- Usar extensiones
- Todo sigue siendo privado y seguro

### 🔒 Solo localhost

Por defecto, el debug port solo escucha en `localhost` (127.0.0.1), no es accesible desde internet.

### 📌 Puedes agregar más flags

```bash
# Windows - ejemplo con múltiples flags
"C:\Program Files\Google\Chrome\Application\chrome.exe" --remote-debugging-port=9222 --new-window https://google.com
```

---

## 🧪 Probar con Vision Module

### 1. Abrir Chrome con debug port

```bash
# Windows
"C:\Program Files\Google\Chrome\Application\chrome.exe" --remote-debugging-port=9222
```

### 2. Navegar a una página soportada

Abre cualquiera de estas en Chrome:
- https://meet.google.com (Google Meet)
- https://teams.microsoft.com (Teams)
- https://discord.com (Discord)
- https://web.whatsapp.com (WhatsApp)
- https://web.telegram.org (Telegram)
- https://chat.google.com (Google Chat)

### 3. Llamar desde Birdie

En TypeScript:
```typescript
const result = await invoke('get_active_tab_context', {
  targetName: 'Meet'  // O el nombre de tu tab
});

console.log(result.content);  // Texto extraído
```

Si ves el contenido extraído → ✅ ¡Funciona!

---

## 🐛 Solución de Problemas

### Problema: "Address already in use"

```
Error: Address already in use
```

**Solución:** Ya tienes Chrome abierto en ese puerto. Cierra Chrome y vuelve a intentar:

```bash
# Windows - Buscar procesos Chrome
tasklist | findstr chrome

# Matar proceso
taskkill /PID <number> /F
```

---

### Problema: "Permission denied" (macOS/Linux)

```
Error: Permission denied
```

**Solución:** Necesitas permisos. Prueba:

```bash
# macOS/Linux
sudo /Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome --remote-debugging-port=9222
```

---

### Problema: Chrome no abre

Si nada sucede cuando ejecutas el comando, puede ser:

1. **Ruta incorrecta** - Verifica dónde está Chrome:
   ```bash
   # Windows - Buscar Chrome
   where chrome
   dir "C:\Program Files\Google\Chrome\Application"

   # macOS
   ls /Applications/ | grep Chrome

   # Linux
   which chromium
   which google-chrome
   ```

2. **Chrome ya abierto** - Ciérralo completamente primero

3. **Ruta con espacios** - Asegúrate de usar comillas:
   ```bash
   "C:\Program Files\Google\Chrome\Application\chrome.exe"
   # Correcto ✅

   C:\Program Files\Google\Chrome\Application\chrome.exe
   # Incorrecto ❌ (causa error por espacio en "Program Files")
   ```

---

## 📋 Checklist de Setup

- [ ] Chrome instalado en tu computadora
- [ ] Terminal/PowerShell abierto
- [ ] Ejecutado comando con `--remote-debugging-port=9222`
- [ ] Nueva ventana de Chrome abierta (diferente de otras)
- [ ] Verificado que localhost:9222 funciona
- [ ] Navegué a una página soportada (Meet, Teams, etc)
- [ ] Probé Vision Module desde Birdie
- [ ] ✅ ¡Funciona!

---

## 🚀 Automatizar (Opcional)

### Script para Windows (.bat)

Crea `start-chrome-debug.bat`:

```batch
@echo off
echo Iniciando Chrome con Debug Port 9222...
start "" "C:\Program Files\Google\Chrome\Application\chrome.exe" --remote-debugging-port=9222
echo Chrome iniciado en ws://localhost:9222
timeout /t 3
```

Luego haz doble clic desde tu escritorio.

### Script para macOS/Linux (.sh)

Crea `start-chrome-debug.sh`:

```bash
#!/bin/bash
echo "Iniciando Chrome con Debug Port 9222..."

if [ "$(uname)" = "Darwin" ]; then
    # macOS
    /Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome --remote-debugging-port=9222 &
else
    # Linux
    google-chrome --remote-debugging-port=9222 &
fi

echo "Chrome iniciado en ws://localhost:9222"
```

Hazlo ejecutable:
```bash
chmod +x start-chrome-debug.sh
./start-chrome-debug.sh
```

---

## 💡 Tips Pro

1. **Usar perfil separado** (opcional):
   ```bash
   "C:\Program Files\Google\Chrome\Application\chrome.exe" --remote-debugging-port=9222 --user-data-dir="C:\temp\chrome-debug"
   ```
   Esto abre Chrome con un perfil separado, sin tu historial normal.

2. **Acceso remoto** (NUNCA en producción):
   ```bash
   chrome.exe --remote-debugging-port=9222 --remote-allow-origins=*
   ```
   Solo para testing local.

3. **Ver info en Console de Chrome**:
   - Abre DevTools (F12)
   - Console
   - Verás mensajes de debug

---

## 📚 Referencia Rápida

| Sistema | Comando |
|---------|---------|
| Windows | `"C:\Program Files\Google\Chrome\Application\chrome.exe" --remote-debugging-port=9222` |
| macOS | `/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome --remote-debugging-port=9222` |
| Linux | `google-chrome --remote-debugging-port=9222` |
| Verificar | `http://localhost:9222` en navegador |

---

## ✅ Ya Configurado

Una vez lo hayas hecho la primera vez:
1. Simplemente abre Chrome cada día con ese comando
2. Ya todo el Context Mapper + Vision Module funcionará
3. Birdie detectará automáticamente tus pestañas

---

## 🎯 Próximo Paso

Una vez Chrome esté corriendo con debug port:

```typescript
// En Birdie
const result = await invoke('validate_app_context', {
  appName: 'Google Meet'
});

const visionResult = await invoke('get_active_tab_context', {
  targetName: 'Meet'
});

console.log(visionResult.content);  // ¡Contenido extraído!
```

---

**Status:** ✅ Chrome está listo para Birdie
**Debug Port:** `9222`
**WebSocket:** `ws://localhost:9222`
**Seguridad:** Solo localhost, completamente privado

