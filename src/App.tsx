import { useState } from 'react';
import { useNotifications } from './hooks/useNotifications';
import CurrentNotification from './components/CurrentNotification';
import NotificationList from './components/NotificationList';
import PlaybackControls from './components/PlaybackControls';
import VoiceRecorder from './components/VoiceRecorder';
import './App.css';

function App() {
  const { queue, currentNotification, isPlaying, playNext, stop, skip } = useNotifications();
  const [apiKey, setApiKey] = useState<string>(localStorage.getItem('google-cloud-api-key') || '');
  const [showSettings, setShowSettings] = useState(false);
  const [tempApiKey, setTempApiKey] = useState(apiKey);

  const handleSaveSettings = () => {
    localStorage.setItem('google-cloud-api-key', tempApiKey);
    setApiKey(tempApiKey);
    setShowSettings(false);
  };

  return (
    <main className="app-container">
      <header className="app-header">
        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
          <div>
            <h1>🐦 Birdie - Asistente de Notificaciones</h1>
            <p>Escucha tus notificaciones en tiempo real</p>
          </div>
          <button
            className="settings-btn"
            onClick={() => {
              setShowSettings(!showSettings);
              setTempApiKey(apiKey);
            }}
            title="Configuración"
          >
            ⚙️
          </button>
        </div>
      </header>

      {showSettings && (
        <section className="settings-section">
          <div className="settings-panel">
            <h3>⚙️ Configuración</h3>
            <div className="settings-group">
              <label htmlFor="api-key">API Key de Google Cloud Speech-to-Text:</label>
              <input
                id="api-key"
                type="password"
                value={tempApiKey}
                onChange={(e) => setTempApiKey(e.target.value)}
                placeholder="Ingrese su API key..."
              />
              <p className="settings-hint">
                🔗 Obtenga su API key en{' '}
                <a
                  href="https://console.cloud.google.com/"
                  target="_blank"
                  rel="noopener noreferrer"
                >
                  Google Cloud Console
                </a>
              </p>
            </div>
            <div className="settings-buttons">
              <button className="btn-primary" onClick={handleSaveSettings}>
                Guardar
              </button>
              <button
                className="btn-secondary"
                onClick={() => {
                  setShowSettings(false);
                  setTempApiKey(apiKey);
                }}
              >
                Cancelar
              </button>
            </div>
          </div>
        </section>
      )}

      <section className="current-section">
        <CurrentNotification notification={currentNotification} isPlaying={isPlaying} />
      </section>

      <section className="controls-section">
        <PlaybackControls
          isPlaying={isPlaying}
          onPlay={playNext}
          onStop={stop}
          onSkip={skip}
          hasQueue={queue.length > 0 || currentNotification !== null}
        />
      </section>

      <section className="voice-recorder-section">
        <VoiceRecorder apiKey={apiKey} languageCode="es-ES" />
      </section>

      <section className="queue-section">
        <NotificationList notifications={queue} />
      </section>
    </main>
  );
}

export default App;
