@echo off
REM Script para iniciar Chrome con Debug Port 9222
REM Uso: Haz doble clic en este archivo

echo.
echo ====================================
echo   Iniciando Chrome Debug Port 9222
echo ====================================
echo.

REM Cerrar Chrome si está abierto
taskkill /F /IM chrome.exe >nul 2>&1

REM Esperar un poco
timeout /t 2 /nobreak

REM Abrir Chrome con debug port
start "" "C:\Program Files\Google\Chrome\Application\chrome.exe" --remote-debugging-port=9222

REM Información
echo.
echo ✅ Chrome iniciado en modo debug
echo.
echo 📍 Verifica la conexión en: http://localhost:9222
echo.
echo 🔌 Birdie ahora puede conectarse a Chrome
echo.
pause
