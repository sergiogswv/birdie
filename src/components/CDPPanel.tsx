import { useState } from 'react';
import { useCDP } from '../hooks/useCDP';

export default function CDPPanel() {
  const {
    connected,
    tabs,
    error,
    errorHelpUrl,
    monitoring,
    monitoringStatus,
    messages,
    connect,
    refreshTabs,
    startMonitoring,
    stopMonitoring,
  } = useCDP();

  const [port, setPort] = useState<number>(9222);
  const [interval, setInterval] = useState<number>(2000);
  const [showAdvanced, setShowAdvanced] = useState(false);

  const handleConnect = async () => {
    await connect(port);
  };

  const handleStartMonitoring = async () => {
    await startMonitoring(interval);
  };

  const handleStopMonitoring = async () => {
    await stopMonitoring();
  };

  const monitoredTabsInfo = tabs.filter((t) => t.has_selector);

  return (
    <section className="cdp-section">
      <div className="cdp-panel">
        {/* Connection Section */}
        <div className="cdp-connection">
          <h3>🌐 Chrome DevTools Protocol</h3>

          <div className="cdp-connection-form">
            <label htmlFor="cdp-port">Puerto de depuración:</label>
            <div className="cdp-port-input">
              <input
                id="cdp-port"
                type="number"
                min="1024"
                max="65535"
                value={port}
                onChange={(e) => setPort(parseInt(e.target.value) || 9222)}
                placeholder="9222"
                disabled={connected}
              />
              <button
                className={`btn-primary ${connected ? 'btn-connected' : ''}`}
                onClick={handleConnect}
              >
                {connected ? '✅ Conectado' : 'Conectar'}
              </button>
            </div>
          </div>

          {/* Error Banner */}
          {error && (
            <div className="cdp-error">
              <p>
                <strong>❌ Error:</strong> {error}
              </p>
              {errorHelpUrl && (
                <a href={errorHelpUrl} target="_blank" rel="noopener noreferrer">
                  📖 Ver guía de configuración
                </a>
              )}
            </div>
          )}

          {/* Connection Help */}
          {!connected && !error && (
            <div className="cdp-help">
              <p>
                <strong>💡 Cómo habilitar depuración en Chrome:</strong>
              </p>
              <code className="code-block">
                chrome.exe --remote-debugging-port=9222
              </code>
            </div>
          )}
        </div>

        {/* Monitoring Section */}
        {connected && (
          <div className="cdp-monitoring">
            <h4>🎯 Monitoreo</h4>

            <div className="cdp-monitoring-controls">
              <label htmlFor="cdp-interval">Intervalo (ms):</label>
              <input
                id="cdp-interval"
                type="number"
                min="500"
                max="10000"
                step="500"
                value={interval}
                onChange={(e) => setInterval(parseInt(e.target.value) || 2000)}
                disabled={monitoring}
              />

              {!monitoring ? (
                <button className="btn-primary" onClick={handleStartMonitoring}>
                  ▶️ Iniciar Monitoreo ({monitoredTabsInfo.length})
                </button>
              ) : (
                <button className="btn-danger" onClick={handleStopMonitoring}>
                  ⏹️ Detener Monitoreo
                </button>
              )}

              <button className="btn-secondary" onClick={refreshTabs}>
                🔄 Actualizar
              </button>
            </div>

            {monitoring && monitoringStatus && (
              <div className="cdp-monitoring-status">
                <p>
                  <strong>✅ Monitoreo activo:</strong> {monitoringStatus.tabs_monitored} pestaña
                  {monitoringStatus.tabs_monitored !== 1 ? 's' : ''} monitoreada
                  {monitoringStatus.tabs_monitored !== 1 ? 's' : ''}
                </p>
              </div>
            )}
          </div>
        )}

        {/* Tabs List */}
        {connected && tabs.length > 0 && (
          <div className="cdp-tabs">
            <h4>📑 Pestañas Abiertas ({tabs.length})</h4>

            {monitoredTabsInfo.length > 0 && (
              <div className="cdp-monitored-section">
                <h5>Monitoreadas</h5>
                <div className="cdp-tabs-list">
                  {monitoredTabsInfo.map((tab) => (
                    <div key={tab.id} className="tab-card tab-monitored">
                      <div className="tab-header">
                        <span className="tab-badge">✓</span>
                        <span className="tab-title">{tab.title || 'Sin título'}</span>
                      </div>
                      <div className="tab-details">
                        <small className="tab-domain">{tab.domain}</small>
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            )}

            {tabs.length > monitoredTabsInfo.length && (
              <div className="cdp-unmoniored-section">
                <h5>No configuradas</h5>
                <div className="cdp-tabs-list">
                  {tabs
                    .filter((t) => !t.has_selector)
                    .map((tab) => (
                      <div key={tab.id} className="tab-card">
                        <div className="tab-header">
                          <span className="tab-title">{tab.title || 'Sin título'}</span>
                        </div>
                        <div className="tab-details">
                          <small className="tab-domain">{tab.domain}</small>
                        </div>
                      </div>
                    ))}
                </div>
              </div>
            )}
          </div>
        )}

        {/* Messages Log */}
        {monitoring && messages.length > 0 && (
          <div className="cdp-messages">
            <h4>💬 Últimos Mensajes Detectados</h4>
            <div className="cdp-messages-list">
              {messages.map((msg, idx) => (
                <div key={idx} className="cdp-message-item">
                  <div className="message-header">
                    <span className="message-time">{msg.timestamp}</span>
                    <span className="message-source">{msg.source}</span>
                  </div>
                  <div className="message-content">
                    <p className="message-text">{msg.message}</p>
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}

        {/* Advanced Section */}
        <div className="cdp-advanced">
          <button
            className="btn-text"
            onClick={() => setShowAdvanced(!showAdvanced)}
          >
            {showAdvanced ? '▼' : '▶'} Opciones Avanzadas
          </button>
          {showAdvanced && (
            <div className="cdp-advanced-content">
              <p>
                <strong>Información del Sistema:</strong>
              </p>
              <ul>
                <li>Conexión: {connected ? '✅ Conectado' : '❌ Desconectado'}</li>
                <li>Pestañas totales: {tabs.length}</li>
                <li>Pestañas monitoreadas: {monitoredTabsInfo.length}</li>
                <li>Monitoreo activo: {monitoring ? '✅ Sí' : '❌ No'}</li>
                <li>Mensajes detectados: {messages.length}</li>
              </ul>
            </div>
          )}
        </div>
      </div>
    </section>
  );
}
