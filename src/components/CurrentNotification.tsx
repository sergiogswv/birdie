import type { NotificationWithId } from '../types/notification';
import NotificationItem from './NotificationItem';

interface Props {
  notification: NotificationWithId | null;
  isPlaying: boolean;
}

export default function CurrentNotification({ notification, isPlaying }: Props) {
  if (!notification) {
    return (
      <div className="current-notification-placeholder">
        <p>Esperando notificaciones...</p>
      </div>
    );
  }

  return (
    <div className="current-notification">
      <div className="playing-indicator">
        {isPlaying && (
          <div className="audio-wave">
            <span></span>
            <span></span>
            <span></span>
          </div>
        )}
        <h2>Reproduciendo ahora</h2>
      </div>
      <NotificationItem notification={notification} isCurrent={true} />
    </div>
  );
}
