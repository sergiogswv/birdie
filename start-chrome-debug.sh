#!/bin/bash

# Script para iniciar Chrome con Debug Port 9222
# Uso: chmod +x start-chrome-debug.sh && ./start-chrome-debug.sh

echo ""
echo "===================================="
echo "  Iniciando Chrome Debug Port 9222"
echo "===================================="
echo ""

# Detectar sistema operativo
if [ "$(uname)" = "Darwin" ]; then
    # macOS
    echo "🍎 Detectado macOS"

    # Cerrar Chrome si está abierto
    pkill -9 "Google Chrome" 2>/dev/null || true

    # Esperar un poco
    sleep 2

    # Abrir Chrome con debug port
    /Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome --remote-debugging-port=9222 &

else
    # Linux
    echo "🐧 Detectado Linux"

    # Cerrar Chrome si está abierto
    pkill -9 chromium || pkill -9 google-chrome || true

    # Esperar un poco
    sleep 2

    # Abrir Chrome con debug port
    google-chrome --remote-debugging-port=9222 &
fi

# Información
echo ""
echo "✅ Chrome iniciado en modo debug"
echo ""
echo "📍 Verifica la conexión en: http://localhost:9222"
echo ""
echo "🔌 Birdie ahora puede conectarse a Chrome"
echo ""
