# Birdie STT Quick Start Guide

## 🎯 Getting Started with Speech-to-Text

### Step 1: Get Google Cloud API Key

1. Go to [Google Cloud Console](https://console.cloud.google.com/)
2. Create a new project (or select existing)
3. Enable **Cloud Speech-to-Text API**
4. Go to **APIs & Services → Credentials**
5. Create **API Key**
6. Copy the key

### Step 2: Launch Birdie

Run the executable:
```
C:\Users\Sergio\Documents\dev\birdie\src-tauri\target\release\birdie.exe
```

Or install via Windows installer:
```
C:\Users\Sergio\Documents\dev\birdie\src-tauri\target\release\bundle\nsis\birdie_0.1.0_x64-setup.exe
```

### Step 3: Configure API Key

1. Click **⚙️** button in top-right header
2. Paste your Google Cloud API key in the settings panel
3. Click **Guardar**

### Step 4: Start Recording

**Option A: Using Keyboard**
- Press **Ctrl+Shift+V** to start recording
- Speak clearly
- Press **Ctrl+Shift+V** again to stop

**Option B: Using Button**
- Click **Grabar** button in VoiceRecorder section
- Speak clearly
- Click **Grabar** again to stop

### Step 5: Copy & Paste

1. Wait for transcription to complete
2. See your text in the text box
3. Click **Copiar al portapapeles**
4. Paste in WhatsApp, Teams, Discord, etc. with Ctrl+V

---

## 🎤 Features

### Recording Controls
- **Duration Counter**: Shows how long you've been recording
- **Real-time Feedback**: Pulsing button during recording
- **Spinner Animation**: Indicates transcription in progress

### Transcription
- **Automatic**: Starts transcribing when recording stops
- **Error Messages**: Clear feedback if API key invalid or network issues
- **Spanish Default**: Set to `es-ES` (editable in code)

### Clipboard Integration
- **One-Click Copy**: "Copiar al portapapeles" button
- **Visual Confirmation**: Button changes to "✓ Copiado" for 2 seconds
- **System Integration**: Uses Windows clipboard directly

### Dark Mode
- Automatically follows your Windows dark mode setting
- All UI elements adapt colors

---

## 🔧 Configuration

### Change Language

Edit `src/App.tsx` (line ~80):

```typescript
// From:
<VoiceRecorder apiKey={apiKey} languageCode="es-ES" />

// To:
<VoiceRecorder apiKey={apiKey} languageCode="en-US" />
```

**Supported Languages:**
- `es-ES` - Spanish (Spain)
- `es-MX` - Spanish (Mexico)
- `en-US` - English (US)
- `en-GB` - English (UK)
- `fr-FR` - French
- `de-DE` - German
- `pt-BR` - Portuguese (Brazil)
- `it-IT` - Italian
- `ja-JP` - Japanese
- [Full list](https://cloud.google.com/speech-to-text/docs/languages)

After changing, rebuild:
```bash
npm run tauri build
```

### Store API Key Securely

⚠️ Currently stored in browser localStorage (plaintext)

For production, modify `src/App.tsx`:

```typescript
// Use Tauri secure storage instead
import { Store } from 'tauri-plugin-store';

const store = new Store('.settings.dat');
const apiKey = await store.get('google-api-key');
```

---

## ⚠️ Troubleshooting

### Issue: "API key not configured" message

**Solution**: Click ⚙️, paste your API key, click Guardar

### Issue: Microphone permission denied

**Solution**:
1. Windows Settings → Privacy & Security → Microphone
2. Allow Birdie to use microphone
3. Restart Birdie

### Issue: "Invalid API key" error

**Solution**:
1. Verify key copied correctly (no extra spaces)
2. Check key in [Google Cloud Console](https://console.cloud.google.com/)
3. Verify Speech-to-Text API is **enabled**

### Issue: "Access denied" error

**Solution**:
1. Go to Google Cloud Console
2. APIs & Services → Cloud Speech-to-Text API
3. Click **ENABLE**

### Issue: No transcription appears

**Possible causes**:
- Background noise too loud (speak clearly)
- Language code doesn't match spoken language
- Audio format not supported (should be automatic)

**Solution**:
1. Try recording in quiet environment
2. Speak slowly and clearly
3. Check language code matches your speech

### Issue: Shortcut (Ctrl+Shift+V) not working

**Solution**:
1. Restart Birdie app
2. Verify API key is set (shortcut disabled if API key empty)
3. Check if another app uses same shortcut:
   - Ctrl+Shift+V might conflict with VPN apps, Discord, etc.

---

## 📊 Performance Notes

### Recording Quality
- **Codec**: WebM/Opus
- **Sample Rate**: 48 kHz
- **Channels**: Mono (1)
- **File Size**: ~50 KB per 10 seconds of speech

### Transcription Speed
- **API Latency**: ~2-5 seconds (depends on audio length)
- **Processing**: Text appears immediately after API response
- **Max Duration**: Recommended <5 minutes per recording

### System Requirements
- **OS**: Windows 10/11
- **RAM**: 256 MB free
- **Disk**: ~200 MB for app
- **Network**: Stable internet (API calls to Google)
- **Microphone**: Any standard mic (USB, built-in, etc.)

---

## 🔐 Privacy & Security

### Data Sent to Google
When you record and transcribe:
- Audio file (WebM/Opus format)
- Language code (e.g., "es-ES")
- API key (to authorize)

**NOT sent:**
- Notification content
- App names
- Personal contacts
- History

### Recommendations
1. Review [Google Cloud Privacy Policy](https://policies.google.com/privacy)
2. Use **API key restriction** (restrict to Speech-to-Text only)
3. Consider organization/billing account separation
4. Monitor API usage in [Google Cloud Console](https://console.cloud.google.com/billing)

---

## 💾 Development & Rebuilding

### Modify Code

```bash
cd C:\Users\Sergio\Documents\dev\birdie

# Make changes to src/ or src-tauri/

# Rebuild
npm run tauri build
```

### Development Mode (with hot reload)

```bash
npm run tauri dev
```

### Clean Build

```bash
npm run tauri build -- --release
cargo clean  # Force full recompile
```

---

## 📝 Notes

- Settings are saved in **browser localStorage**
  - Location: Browser dev tools → Application → Local Storage
  - Key: `google-cloud-api-key`

- Transcriptions are **NOT saved** after recording
  - Each reset clears the text
  - Phase 2 will add history/database

- Shortcuts are **per-app instance**
  - Only works when Birdie window is focused
  - Phase 2 will make truly global

---

## 📞 Support

For issues or feature requests:
1. Check PASO8_STT_IMPLEMENTATION.md (detailed technical docs)
2. Review troubleshooting section above
3. Check Google Cloud API documentation
4. File GitHub issue (when repo is public)

---

**Version**: PASO 8 (MVP)
**Date**: 2026-02-08
**Status**: ✅ Ready for Testing
