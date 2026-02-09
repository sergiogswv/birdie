use serde::{Deserialize, Serialize};

/// Request structure for Google Cloud Speech-to-Text API
#[derive(Serialize)]
struct GoogleSttRequest {
    audio: AudioContent,
    config: RecognitionConfig,
}

#[derive(Serialize)]
struct AudioContent {
    content: String, // base64-encoded audio
}

#[derive(Serialize)]
struct RecognitionConfig {
    encoding: String,
    #[serde(rename = "sampleRateHertz")]
    sample_rate_hertz: u32,
    #[serde(rename = "languageCode")]
    language_code: String,
}

/// Response from Google Cloud Speech-to-Text API
#[derive(Deserialize)]
pub struct GoogleSttResponse {
    results: Option<Vec<SpeechRecognitionResult>>,
}

#[derive(Deserialize)]
struct SpeechRecognitionResult {
    alternatives: Option<Vec<Alternative>>,
}

#[derive(Deserialize)]
struct Alternative {
    transcript: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TranscriptionResult {
    pub text: String,
    pub success: bool,
    pub error: Option<String>,
}

/// Transcribe audio using Google Cloud Speech-to-Text API
pub async fn transcribe_audio(
    audio_base64: String,
    api_key: String,
    language_code: String,
) -> Result<TranscriptionResult, String> {
    // Validate API key
    if api_key.is_empty() {
        return Ok(TranscriptionResult {
            text: String::new(),
            success: false,
            error: Some("API key not configured. Please add your Google Cloud API key in settings.".to_string()),
        });
    }

    let request = GoogleSttRequest {
        audio: AudioContent {
            content: audio_base64,
        },
        config: RecognitionConfig {
            encoding: "WEBM_OPUS".to_string(),
            sample_rate_hertz: 48000,
            language_code,
        },
    };

    let client = reqwest::Client::new();
    let url = format!("https://speech.googleapis.com/v1/speech:recognize?key={}", api_key);

    match client
        .post(&url)
        .json(&request)
        .send()
        .await
    {
        Ok(response) => {
            match response.json::<GoogleSttResponse>().await {
                Ok(stt_response) => {
                    if let Some(results) = stt_response.results {
                        if let Some(result) = results.first() {
                            if let Some(alternatives) = &result.alternatives {
                                if let Some(alt) = alternatives.first() {
                                    return Ok(TranscriptionResult {
                                        text: alt.transcript.clone(),
                                        success: true,
                                        error: None,
                                    });
                                }
                            }
                        }
                    }
                    Ok(TranscriptionResult {
                        text: String::new(),
                        success: false,
                        error: Some("No transcription results found".to_string()),
                    })
                }
                Err(e) => Ok(TranscriptionResult {
                    text: String::new(),
                    success: false,
                    error: Some(format!("Failed to parse API response: {}", e)),
                }),
            }
        }
        Err(e) => {
            let error_msg = if e.status().map(|s| s.as_u16()) == Some(401) {
                "Invalid API key. Please check your Google Cloud API key.".to_string()
            } else if e.status().map(|s| s.as_u16()) == Some(403) {
                "Access denied. Please ensure Speech-to-Text API is enabled in Google Cloud.".to_string()
            } else {
                format!("API request failed: {}", e)
            };

            Ok(TranscriptionResult {
                text: String::new(),
                success: false,
                error: Some(error_msg),
            })
        }
    }
}

/// Copy text to clipboard
pub fn copy_to_clipboard(text: String) -> Result<(), String> {
    match arboard::Clipboard::new() {
        Ok(mut clipboard) => {
            clipboard
                .set_text(text)
                .map_err(|e| format!("Failed to copy to clipboard: {}", e))?;
            Ok(())
        }
        Err(e) => Err(format!("Failed to access clipboard: {}", e)),
    }
}
