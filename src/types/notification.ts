export interface NotificationEvent {
  app_name: string;
  sender: string;
  message: string;
  timestamp: string;
  app_icon?: string;
}

export interface NotificationWithId extends NotificationEvent {
  id: string; // Unique ID for React keys
}
