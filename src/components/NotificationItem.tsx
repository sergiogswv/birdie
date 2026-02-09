import type { NotificationWithId } from '../types/notification';

interface Props {
  notification: NotificationWithId;
  isCurrent?: boolean;
}

export default function NotificationItem({ notification, isCurrent = false }: Props) {
  const formattedTime = new Date(notification.timestamp).toLocaleTimeString();

  return (
    <div className={`notification-item ${isCurrent ? 'current' : ''}`}>
      <div className="notification-header">
        <span className="app-name">{notification.app_name}</span>
        <span className="timestamp">{formattedTime}</span>
      </div>
      <div className="notification-sender">{notification.sender}</div>
      <div className="notification-message">{notification.message}</div>
    </div>
  );
}
