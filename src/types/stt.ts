export interface TranscriptionResult {
  text: string;
  success: boolean;
  error?: string;
}

export type RecordingState = 'idle' | 'recording' | 'transcribing' | 'completed' | 'error';

export interface VoiceRecorderState {
  isRecording: boolean;
  state: RecordingState;
  transcription: string;
  error: string | null;
  duration: number;
  isProcessing: boolean;
}
