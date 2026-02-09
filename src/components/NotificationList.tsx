import type { NotificationWithId } from '../types/notification';
import NotificationItem from './NotificationItem';

interface Props {
  notifications: NotificationWithId[];
}

export default function NotificationList({ notifications }: Props) {
  if (notifications.length === 0) {
    return (
      <div className="notification-list-empty">
        <p>No hay notificaciones en cola</p>
      </div>
    );
  }

  return (
    <div className="notification-list">
      <h2>Cola de notificaciones ({notifications.length})</h2>
      {notifications.map((notification) => (
        <NotificationItem key={notification.id} notification={notification} />
      ))}
    </div>
  );
}
