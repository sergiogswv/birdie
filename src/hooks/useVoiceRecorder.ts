import { useState, useRef, useCallback } from 'react';
import { VoiceRecorderState } from '../types/stt';

export const useVoiceRecorder = () => {
  const [state, setState] = useState<VoiceRecorderState>({
    isRecording: false,
    state: 'idle',
    transcription: '',
    error: null,
    duration: 0,
    isProcessing: false,
  });

  const mediaRecorderRef = useRef<MediaRecorder | null>(null);
  const audioChunksRef = useRef<Blob[]>([]);
  const streamRef = useRef<MediaStream | null>(null);
  const durationIntervalRef = useRef<ReturnType<typeof setInterval> | null>(null);

  // Start recording
  const startRecording = useCallback(async () => {
    try {
      setState((prev) => ({ ...prev, state: 'recording', isRecording: true, error: null }));

      const stream = await navigator.mediaDevices.getUserMedia({
        audio: {
          echoCancellation: true,
          noiseSuppression: true,
          autoGainControl: true,
        },
      });

      streamRef.current = stream;
      audioChunksRef.current = [];

      const mediaRecorder = new MediaRecorder(stream, {
        mimeType: 'audio/webm;codecs=opus',
      });

      mediaRecorder.ondataavailable = (event) => {
        if (event.data.size > 0) {
          audioChunksRef.current.push(event.data);
        }
      };

      mediaRecorder.start();
      mediaRecorderRef.current = mediaRecorder;

      // Start duration counter
      let duration = 0;
      durationIntervalRef.current = setInterval(() => {
        duration += 100;
        setState((prev) => ({ ...prev, duration }));
      }, 100);
    } catch (error) {
      const errorMsg = error instanceof Error ? error.message : 'Failed to access microphone';
      setState((prev) => ({
        ...prev,
        state: 'error',
        isRecording: false,
        error: errorMsg,
      }));
    }
  }, []);

  // Stop recording and return audio blob
  const stopRecording = useCallback(async (): Promise<Blob | null> => {
    return new Promise((resolve) => {
      if (!mediaRecorderRef.current) {
        resolve(null);
        return;
      }

      const mediaRecorder = mediaRecorderRef.current;

      mediaRecorder.onstop = () => {
        // Clear duration interval
        if (durationIntervalRef.current) {
          clearInterval(durationIntervalRef.current);
          durationIntervalRef.current = null;
        }

        // Stop all audio tracks
        if (streamRef.current) {
          streamRef.current.getTracks().forEach((track) => track.stop());
          streamRef.current = null;
        }

        // Create blob from chunks
        const audioBlob = new Blob(audioChunksRef.current, { type: 'audio/webm;codecs=opus' });
        audioChunksRef.current = [];

        resolve(audioBlob);
      };

      mediaRecorder.stop();
      setState((prev) => ({ ...prev, isRecording: false }));
    });
  }, []);

  // Toggle recording
  const toggleRecording = useCallback(async () => {
    if (state.isRecording) {
      await stopRecording();
    } else {
      await startRecording();
    }
  }, [state.isRecording, startRecording, stopRecording]);

  // Reset state
  const reset = useCallback(() => {
    if (durationIntervalRef.current) {
      clearInterval(durationIntervalRef.current);
      durationIntervalRef.current = null;
    }
    if (streamRef.current) {
      streamRef.current.getTracks().forEach((track) => track.stop());
      streamRef.current = null;
    }
    setState({
      isRecording: false,
      state: 'idle',
      transcription: '',
      error: null,
      duration: 0,
      isProcessing: false,
    });
  }, []);

  return {
    ...state,
    startRecording,
    stopRecording,
    toggleRecording,
    reset,
    getAudioBlob: () => {
      if (audioChunksRef.current.length === 0) return null;
      return new Blob(audioChunksRef.current, { type: 'audio/webm;codecs=opus' });
    },
  };
};
