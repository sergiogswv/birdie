import { useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { TranscriptionResult } from '../types/stt';

export const useSpeechToText = () => {
  // Convert Blob to base64
  const blobToBase64 = useCallback((blob: Blob): Promise<string> => {
    return new Promise((resolve, reject) => {
      const reader = new FileReader();
      reader.onloadend = () => {
        const base64String = reader.result as string;
        // Remove the data:audio/webm;base64, prefix
        const base64Data = base64String.split(',')[1];
        resolve(base64Data);
      };
      reader.onerror = reject;
      reader.readAsDataURL(blob);
    });
  }, []);

  // Transcribe audio using Tauri command
  const transcribe = useCallback(
    async (
      audioBlob: Blob,
      apiKey: string,
      languageCode: string = 'es-ES'
    ): Promise<TranscriptionResult> => {
      try {
        const base64Audio = await blobToBase64(audioBlob);

        const result = await invoke<TranscriptionResult>('transcribe_audio', {
          audioBase64: base64Audio,
          apiKey,
          languageCode,
        });

        return result;
      } catch (error) {
        const errorMsg = error instanceof Error ? error.message : 'Transcription failed';
        return {
          text: '',
          success: false,
          error: errorMsg,
        };
      }
    },
    [blobToBase64]
  );

  // Copy text to clipboard
  const copyToClipboard = useCallback(async (text: string): Promise<boolean> => {
    try {
      await invoke('copy_to_clipboard', { text });
      return true;
    } catch (error) {
      console.error('Failed to copy to clipboard:', error);
      return false;
    }
  }, []);

  return {
    transcribe,
    copyToClipboard,
    blobToBase64,
  };
};
