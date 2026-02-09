import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type {
  ConnectionResult,
  TabInfo,
  MonitoringStatus,
  CDPMessage,
  ScriptResult,
} from '../types/cdp';

export function useCDP() {
  const [connected, setConnected] = useState<boolean>(false);
  const [tabs, setTabs] = useState<TabInfo[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [errorHelpUrl, setErrorHelpUrl] = useState<string | null>(null);
  const [monitoring, setMonitoring] = useState<boolean>(false);
  const [monitoringStatus, setMonitoringStatus] = useState<MonitoringStatus | null>(null);
  const [messages, setMessages] = useState<CDPMessage[]>([]);
  const [tabsMonitored, setTabsMonitored] = useState<number>(0);

  // Connect to Chrome
  const connect = useCallback(async (port: number = 9222) => {
    setError(null);
    setErrorHelpUrl(null);
    try {
      const result: ConnectionResult = await invoke('cdp_connect', { port });
      if (result.success) {
        setConnected(true);
        setError(null);
        setErrorHelpUrl(null);
        // Refresh tabs list
        refreshTabs();
      } else {
        setConnected(false);
        setError(result.message);
        setErrorHelpUrl(result.error_help_url || null);
      }
    } catch (err) {
      setConnected(false);
      setError(err instanceof Error ? err.message : 'Error desconocido');
    }
  }, []);

  // Refresh tabs list
  const refreshTabs = useCallback(async () => {
    try {
      const tabsList: TabInfo[] = await invoke('cdp_get_tabs');
      setTabs(tabsList);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Error al obtener pestañas');
    }
  }, []);

  // Find tab by title
  const findTab = useCallback(async (titleContains: string): Promise<TabInfo | null> => {
    try {
      const tab: TabInfo | null = await invoke('cdp_find_tab', {
        title_contains: titleContains,
      });
      return tab;
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Error al buscar pestaña');
      return null;
    }
  }, []);

  // Execute JavaScript in a tab
  const executeScript = useCallback(
    async (tabId: string, script: string): Promise<ScriptResult | null> => {
      try {
        const result: ScriptResult = await invoke('cdp_execute_script', {
          tab_id: tabId,
          script,
        });
        return result;
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Error al ejecutar script');
        return null;
      }
    },
    [],
  );

  // Start monitoring
  const startMonitoring = useCallback(async (intervalMs: number = 2000) => {
    try {
      const status: MonitoringStatus = await invoke('cdp_start_monitoring', {
        app_handle: undefined, // Tauri handles this internally
        interval_ms: intervalMs,
      });
      setMonitoring(true);
      setMonitoringStatus(status);
      setTabsMonitored(status.tabs_monitored);
      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Error al iniciar monitoreo');
      setMonitoring(false);
    }
  }, []);

  // Stop monitoring
  const stopMonitoring = useCallback(async () => {
    try {
      const status: MonitoringStatus = await invoke('cdp_stop_monitoring');
      setMonitoring(false);
      setMonitoringStatus(status);
      setTabsMonitored(0);
      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Error al detener monitoreo');
    }
  }, []);

  // Listen for CDP messages
  useEffect(() => {
    let unlistener: (() => void) | null = null;

    const setupListener = async () => {
      try {
        unlistener = await listen<CDPMessage>('cdp-message-detected', (event) => {
          setMessages((prev) => {
            // Keep only last 10 messages
            const updated = [event.payload, ...prev];
            return updated.slice(0, 10);
          });
        });
      } catch (err) {
        console.error('Error setting up CDP listener:', err);
      }
    };

    setupListener();

    return () => {
      if (unlistener) {
        unlistener();
      }
    };
  }, []);

  return {
    connected,
    tabs,
    error,
    errorHelpUrl,
    monitoring,
    monitoringStatus,
    messages,
    tabsMonitored,
    connect,
    refreshTabs,
    findTab,
    executeScript,
    startMonitoring,
    stopMonitoring,
  };
}
