import { useEffect, useRef } from 'react';
import { register, unregister, isRegistered } from '@tauri-apps/plugin-global-shortcut';

export const useGlobalShortcut = (
  shortcut: string,
  callback: () => void,
  enabled: boolean = true
) => {
  const callbackRef = useRef(callback);

  // Update callback ref when callback changes
  useEffect(() => {
    callbackRef.current = callback;
  }, [callback]);

  useEffect(() => {
    if (!enabled) {
      return;
    }

    let isMounted = true;

    const setupShortcut = async () => {
      try {
        // Check if already registered
        const alreadyRegistered = await isRegistered(shortcut);

        if (!alreadyRegistered && isMounted) {
          await register(shortcut, () => {
            callbackRef.current();
          });
        }
      } catch (error) {
        console.error(`Failed to register shortcut ${shortcut}:`, error as Error);
      }
    };

    setupShortcut();

    // Cleanup: unregister shortcut when component unmounts or disabled
    return () => {
      isMounted = false;
      unregister(shortcut).catch((error) => {
        console.warn(`Failed to unregister shortcut ${shortcut}:`, error);
      });
    };
  }, [shortcut, enabled]);
};
