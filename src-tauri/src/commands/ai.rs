use reqwest::multipart;
use serde_json::Value;
use std::env;

pub async fn transcribe_audio(
    audio_data: Vec<f32>,
    original_sample_rate: u32,
) -> Result<String, String> {
    dotenvy::dotenv().ok();
    let api_key = env::var("GROQ_API_KEY").map_err(|_| "API Key missing")?;
    println!(">>> [API Key] {:?}", api_key);

    // Downsample logic (e.g., 48k to 16k is a factor of 3)
    let factor = (original_sample_rate / 16000).max(1) as usize;
    let downsampled: Vec<f32> = audio_data.iter().step_by(factor).cloned().collect();

    let wav_bytes = crate::audio::convert_to_wav(downsampled, 16000);

    let client = reqwest::Client::new();
    let form = multipart::Form::new()
        .text("model", "whisper-large-v3")
        .part(
            "file",
            multipart::Part::bytes(wav_bytes)
                .file_name("audio.wav")
                .mime_str("audio/wav")
                .unwrap(),
        );

    let res = client
        .post("https://api.groq.com/openai/v1/audio/transcriptions")
        .header("Authorization", format!("Bearer {}", api_key))
        .multipart(form)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let json: Value = res.json().await.map_err(|e| e.to_string())?;
    println!(">>> [API RESULT] {:?}", json);
    Ok(json["text"].as_str().unwrap_or("").to_string())
}
