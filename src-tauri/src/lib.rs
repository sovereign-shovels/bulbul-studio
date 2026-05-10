use serde::{Deserialize, Serialize};
use tauri::State;

mod config;

use config::Config;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtsResult {
    pub audio_bytes: Vec<u8>,
    pub format: String,
}

#[derive(Debug, Serialize)]
struct BulbulRequest {
    inputs: Vec<String>,
    target_language_code: String,
    speaker: String,
    model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pitch: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pace: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    loudness: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    speech_sample_rate: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_preprocessing: Option<bool>,
}

struct AppState {
    config: Config,
}

#[tauri::command]
async fn get_config(state: State<'_, AppState>) -> Result<Config, String> {
    Ok(state.config.clone())
}

#[tauri::command]
async fn generate_speech(
    state: State<'_, AppState>,
    text: String,
    language: Option<String>,
    speaker: Option<String>,
) -> Result<TtsResult, String> {
    let api_key = state.config.api_key().ok_or("BULBUL_API_KEY not set")?;
    let endpoint = state.config.endpoint.clone().unwrap_or_default();
    let language = language.unwrap_or_else(|| state.config.language.clone().unwrap_or_default());
    let speaker = speaker.unwrap_or_else(|| state.config.speaker.clone().unwrap_or_default());
    let model = state.config.model.clone().unwrap_or_else(|| "bulbul:v3".into());

    let body = BulbulRequest {
        inputs: vec![text],
        target_language_code: language,
        speaker,
        model,
        pitch: None,
        pace: None,
        loudness: None,
        speech_sample_rate: Some(22050),
        enable_preprocessing: Some(true),
    };

    let client = reqwest::Client::new();
    let response = client
        .post(&endpoint)
        .header("api-subscription-key", api_key)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let body_text = response.text().await.unwrap_or_default();
        return Err(format!("Bulbul API error ({}): {}", status, body_text));
    }

    let audio_bytes = response
        .bytes()
        .await
        .map_err(|e| format!("Failed to read audio: {}", e))?
        .to_vec();

    Ok(TtsResult {
        audio_bytes,
        format: "wav".into(),
    })
}

#[tauri::command]
async fn save_audio(path: String, audio_bytes: Vec<u8>) -> Result<(), String> {
    tokio::fs::write(&path, audio_bytes)
        .await
        .map_err(|e| format!("Failed to save audio: {}", e))?;
    Ok(())
}

pub fn run() {
    let config = Config::load();
    let state = AppState { config };

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            get_config,
            generate_speech,
            save_audio,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
