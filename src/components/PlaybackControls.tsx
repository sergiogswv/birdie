interface Props {
  isPlaying: boolean;
  onPlay: () => void;
  onStop: () => void;
  onSkip: () => void;
  hasQueue: boolean;
}

export default function PlaybackControls({ isPlaying, onPlay, onStop, onSkip, hasQueue }: Props) {
  return (
    <div className="playback-controls">
      {!isPlaying && hasQueue && (
        <button onClick={onPlay} className="btn-play">
          ▶️ Reproducir
        </button>
      )}
      {isPlaying && (
        <button onClick={onStop} className="btn-stop">
          ⏹️ Detener
        </button>
      )}
      {hasQueue && (
        <button onClick={onSkip} className="btn-skip">
          ⏭️ Siguiente
        </button>
      )}
    </div>
  );
}
