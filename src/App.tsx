import { useNotifications } from './hooks/useNotifications';
import CurrentNotification from './components/CurrentNotification';
import NotificationList from './components/NotificationList';
import PlaybackControls from './components/PlaybackControls';
import './App.css';

function App() {
  const { queue, currentNotification, isPlaying, playNext, stop, skip } = useNotifications();

  return (
    <main className="app-container">
      <header className="app-header">
        <h1>🐦 Birdie - Asistente de Notificaciones</h1>
        <p>Escucha tus notificaciones en tiempo real</p>
      </header>

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

      <section className="queue-section">
        <NotificationList notifications={queue} />
      </section>
    </main>
  );
}

export default App;
