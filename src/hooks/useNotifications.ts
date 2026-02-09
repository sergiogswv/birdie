import { useState, useEffect, useCallback, useRef } from 'react';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import type { NotificationEvent, NotificationWithId } from '../types/notification';

export function useNotifications() {
  const [queue, setQueue] = useState<NotificationWithId[]>([]);
  const [currentNotification, setCurrentNotification] = useState<NotificationWithId | null>(null);
  const [isPlaying, setIsPlaying] = useState(false);
  const isPlayingRef = useRef(false);
  const queueRef = useRef<NotificationWithId[]>([]);
  const shouldAutoPlayFirstRef = useRef(true);

  // Keep queue ref in sync with state
  useEffect(() => {
    queueRef.current = queue;
  }, [queue]);

  // Listen to notification events from backend
  useEffect(() => {
    console.log('%c🔴 [BIRDIE] Iniciando escucha de notificaciones...', 'color: red; font-weight: bold; font-size: 14px');

    let unlistenPromise: Promise<() => void> | null = null;

    const setupListener = async () => {
      unlistenPromise = listen<NotificationEvent>('notification-received', (event) => {
        console.log('%c📬 [EN TIEMPO REAL] Notificación recibida desde:', 'color: blue; font-weight: bold; font-size: 12px', event.payload.app_name);
        console.log('%c   De: %c' + event.payload.sender, 'color: blue; font-weight: bold', 'color: green; font-size: 11px');
        console.log('%c   Mensaje: %c' + event.payload.message, 'color: blue; font-weight: bold', 'color: green; font-size: 11px');
        console.log('%c   Hora: %c' + new Date(event.payload.timestamp).toLocaleTimeString(), 'color: blue; font-weight: bold', 'color: gray; font-size: 10px');

        const notificationWithId: NotificationWithId = {
          ...event.payload,
          id: crypto.randomUUID(),
        };

        setQueue((prev) => {
          const newQueue = [...prev, notificationWithId];
          const isFirstNotification = prev.length === 0 && shouldAutoPlayFirstRef.current;

          if (isFirstNotification) {
            // Auto-play first notification only
            console.log('%c▶️ [TTS] Primera notificación. Iniciando reproducción automática...', 'color: orange; font-weight: bold; font-size: 12px');
            shouldAutoPlayFirstRef.current = false; // Don't auto-play any more
            playNextInternal(newQueue);
          } else {
            // Just add to queue, don't auto-play
            console.log('%c📥 [COLA] Notificación agregada a la cola. Total en cola:', 'color: purple; font-weight: bold; font-size: 11px', newQueue.length);
          }

          return newQueue;
        });
      });
    };

    setupListener();

    return () => {
      unlistenPromise?.then((fn) => fn());
    };
  }, []);

  // Internal play function that doesn't depend on state
  const playNextInternal = async (currentQueue: NotificationWithId[]) => {
    if (currentQueue.length === 0) {
      console.log('%c✅ [COLA] Cola vacía. Deteniendo reproducción.', 'color: green; font-weight: bold; font-size: 11px');
      setCurrentNotification(null);
      setIsPlaying(false);
      isPlayingRef.current = false;
      return;
    }

    const [next, ...rest] = currentQueue;
    setCurrentNotification(next);
    setIsPlaying(true);
    isPlayingRef.current = true;

    // Construct speech text in Spanish - use commas instead of periods to avoid TTS cutting off
    const speechText = `Nueva notificación de ${next.app_name}, de ${next.sender}: ${next.message}`;

    console.log('%c🔊 [TTS] Iniciando reproducción de voz...', 'color: #FF6B00; font-weight: bold; font-size: 13px');
    console.log('%c   Texto: %c' + speechText, 'color: #FF6B00; font-weight: bold', 'color: #FFB347; font-size: 12px');

    try {
      // Invoke TTS plugin
      console.log('%c⏳ [TTS] Llamando al plugin con texto:', 'color: #FF6B00; font-weight: bold; font-size: 11px', speechText);

      // Attempt to speak - tauri-plugin-tts expects payload with text and lang
      const result = await invoke<string>('plugin:tts|speak', {
        payload: {
          text: speechText,
          lang: "es"
        }
      });
      console.log('%c   Resultado TTS:', 'color: #FF6B00; font-weight: bold', result);

      // Calculate approximate speech duration: ~250 ms per word in Spanish + response time
      const wordCount = speechText.split(/\s+/).length;
      const speechDuration = wordCount * 250; // 250ms per word for Spanish
      const responseTime = 5000; // 5 seconds for user to respond
      const estimatedDuration = Math.max(speechDuration + responseTime, 7000); // Min 7 seconds total

      console.log('%c⏱️ [TTS] Esperando', 'color: #FF6B00; font-weight: bold; font-size: 11px', `${estimatedDuration}ms (${wordCount} palabras + tiempo de respuesta)...`);

      // Wait for speech to complete
      await new Promise(resolve => setTimeout(resolve, estimatedDuration));

      console.log('%c✅ [TTS] Reproducción completada. Esperando acciones del usuario...', 'color: green; font-weight: bold; font-size: 12px');

      // Stop playback, keep current notification displayed
      setIsPlaying(false);
      isPlayingRef.current = false;

      // Don't update queue here - let user press Siguiente to control what plays next
      console.log('%c📋 [COLA] Quedan', 'color: purple; font-weight: bold; font-size: 11px', rest.length, 'notificaciones en la cola');
    } catch (error) {
      console.error('%c❌ [TTS ERROR]', 'color: red; font-weight: bold; font-size: 13px', error);
      console.log('%c📋 Detalles del error:', 'color: red; font-weight: bold; font-size: 11px');
      console.log(error);
      setIsPlaying(false);
      isPlayingRef.current = false;
    }
  };

  // Public playNext function
  const playNext = useCallback(() => {
    if (isPlayingRef.current) {
      console.log('%c⚠️ [CONTROLES] Ya se está reproduciendo', 'color: #FFD700; font-weight: bold; font-size: 11px');
      return;
    }

    if (queueRef.current.length === 0) {
      console.log('%c⚠️ [CONTROLES] Cola vacía. No hay nada que reproducir.', 'color: #FFD700; font-weight: bold; font-size: 11px');
      return;
    }

    console.log('%c▶️ [CONTROLES] Usuario presionó REPRODUCIR. Total en cola:', 'color: #1E90FF; font-weight: bold; font-size: 12px', queueRef.current.length);
    playNextInternal(queueRef.current);
  }, []);

  // Stop current playback
  const stop = useCallback(async () => {
    try {
      console.log('%c⏹️ [CONTROLES] Usuario presionó DETENER', 'color: #DC143C; font-weight: bold; font-size: 12px');
      await invoke('plugin:tts|stop', {});
      setIsPlaying(false);
      setCurrentNotification(null);
      isPlayingRef.current = false;
      console.log('%c✅ [CONTROLES] Audio detenido', 'color: #DC143C; font-weight: bold; font-size: 11px');
    } catch (error) {
      console.error('%c❌ [CONTROLES] Error al detener:', 'color: #DC143C; font-weight: bold; font-size: 11px', error);
    }
  }, []);

  // Skip to next notification
  const skip = useCallback(async () => {
    console.log('%c⏭️ [CONTROLES] Usuario presionó SIGUIENTE', 'color: #9370DB; font-weight: bold; font-size: 12px');
    await stop();

    // After stopping, if there are items in queue, play the next one
    if (queueRef.current.length > 0) {
      // Remove first item since we just skipped it
      const remaining = queueRef.current.slice(1);
      console.log('%c⏭️ [COLA] Saltando. Quedan:', 'color: #9370DB; font-weight: bold; font-size: 11px', remaining.length);
      setQueue(remaining);
      if (remaining.length > 0) {
        console.log('%c▶️ [CONTROLES] Reproduciendo siguiente de la cola...', 'color: #9370DB; font-weight: bold; font-size: 11px');
        playNextInternal(remaining);
      }
    }
  }, [stop]);

  return {
    queue,
    currentNotification,
    isPlaying,
    playNext,
    stop,
    skip,
  };
}
