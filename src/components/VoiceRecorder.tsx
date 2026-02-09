import React, { useState, useEffect, useRef } from 'react';
import { useVoiceRecorder } from '../hooks/useVoiceRecorder';
import { useSpeechToText } from '../hooks/useSpeechToText';
import { useGlobalShortcut } from '../hooks/useGlobalShortcut';

interface VoiceRecorderProps {
  apiKey: string;
  languageCode?: string;
}

const VoiceRecorder: React.FC<VoiceRecorderProps> = ({ apiKey, languageCode = 'es-ES' }) => {
  const [isCopied, setIsCopied] = useState(false);
  const voiceRecorder = useVoiceRecorder();
  const { transcribe, copyToClipboard } = useSpeechToText();
  const copyTimeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  // Register global shortcut
  useGlobalShortcut(
    'ctrl+shift+v',
    () => {
      if (!voiceRecorder.isRecording) {
        voiceRecorder.toggleRecording();
      }
    },
    !!apiKey // Only enable if API key is set
  );

  // Auto-transcribe when recording stops
  useEffect(() => {
    if (voiceRecorder.state === 'recording' && !voiceRecorder.isRecording) {
      const performTranscription = async () => {
        const audioBlob = voiceRecorder.getAudioBlob();
        if (!audioBlob) return;

        // Update state to transcribing
        // We need to manually set isProcessing since we don't have direct state control here
        const transcriptionArea = document.getElementById('transcription-area');
        if (transcriptionArea) {
          transcriptionArea.textContent = 'Transcribiendo...';
        }

        const result = await transcribe(audioBlob, apiKey, languageCode);

        if (result.success) {
          // Update the transcription area
          if (transcriptionArea) {
            transcriptionArea.textContent = result.text;
          }
        } else {
          if (transcriptionArea) {
            transcriptionArea.textContent = `Error: ${result.error || 'Unknown error'}`;
          }
        }
      };

      performTranscription();
    }
  }, [voiceRecorder.state, voiceRecorder.isRecording, transcribe, apiKey, languageCode]);

  const handleCopy = async () => {
    const transcriptionArea = document.getElementById('transcription-area') as HTMLDivElement;
    if (transcriptionArea && transcriptionArea.textContent) {
      const success = await copyToClipboard(transcriptionArea.textContent);
      if (success) {
        setIsCopied(true);
        if (copyTimeoutRef.current) {
          clearTimeout(copyTimeoutRef.current);
        }
        copyTimeoutRef.current = setTimeout(() => {
          setIsCopied(false);
        }, 2000);
      }
    }
  };

  const formatDuration = (ms: number) => {
    const seconds = Math.floor(ms / 1000);
    const minutes = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${minutes}:${secs.toString().padStart(2, '0')}`;
  };

  if (!apiKey) {
    return (
      <div className="voice-recorder voice-recorder--disabled">
        <div className="voice-recorder__message">
          <p>⚙️ Configure su API key de Google Cloud en la sección de configuración para usar el grabador de voz.</p>
        </div>
      </div>
    );
  }

  return (
    <div className="voice-recorder">
      <div className="voice-recorder__header">
        <h3>🎤 Grabadora de Voz</h3>
        <p>Presione el botón o Ctrl+Shift+V para grabar</p>
      </div>

      <div className="voice-recorder__controls">
        <button
          className={`voice-recorder__button ${voiceRecorder.isRecording ? 'recording' : ''}`}
          onClick={() => voiceRecorder.toggleRecording()}
          disabled={voiceRecorder.state === 'transcribing'}
        >
          {voiceRecorder.isRecording ? (
            <>
              <span className="pulse"></span>
              Grabando... {formatDuration(voiceRecorder.duration)}
            </>
          ) : voiceRecorder.state === 'transcribing' ? (
            <>
              <span className="spinner"></span>
              Transcribiendo...
            </>
          ) : (
            'Grabar'
          )}
        </button>

        {voiceRecorder.state !== 'idle' && (
          <button
            className="voice-recorder__reset"
            onClick={() => voiceRecorder.reset()}
            disabled={voiceRecorder.isRecording || voiceRecorder.state === 'transcribing'}
          >
            Limpiar
          </button>
        )}
      </div>

      {voiceRecorder.error && (
        <div className="voice-recorder__error">
          <p>❌ {voiceRecorder.error}</p>
        </div>
      )}

      <div className="voice-recorder__transcription">
        <label>Texto transcrito:</label>
        <div id="transcription-area" className="voice-recorder__text">
          {voiceRecorder.state === 'idle'
            ? 'El texto transcrito aparecerá aquí...'
            : voiceRecorder.state === 'recording'
            ? 'Grabando...'
            : ''}
        </div>

        {voiceRecorder.state === 'completed' && (
          <button className={`voice-recorder__copy ${isCopied ? 'copied' : ''}`} onClick={handleCopy}>
            {isCopied ? '✓ Copiado' : 'Copiar al portapapeles'}
          </button>
        )}
      </div>

      <div className="voice-recorder__hint">
        <p>💡 Después de grabar, el texto se copiará automáticamente al portapapeles para que pueda pegarlo en WhatsApp, Teams, Discord, etc.</p>
      </div>
    </div>
  );
};

export default VoiceRecorder;
