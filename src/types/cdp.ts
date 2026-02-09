/**
 * Chrome DevTools Protocol (CDP) types and interfaces
 */

export interface ConnectionResult {
  success: boolean;
  message: string;
  tabs_count: number;
  error_help_url?: string;
}

export interface TabInfo {
  id: string;
  title: string;
  url: string;
  domain: string;
  has_selector: boolean;
}

export interface ScriptResult {
  success: boolean;
  result?: string;
  error?: string;
}

export interface MonitoringStatus {
  is_monitoring: boolean;
  tabs_monitored: number;
  interval_ms: number;
}

export interface CDPMessage {
  tab_id: string;
  tab_title: string;
  domain: string;
  sender: string;
  message: string;
  timestamp: string;
  source: string;
}
