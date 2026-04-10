use reqwest::multipart;
use serde_json::{ json, Value };
use std::env;

pub async fn transcribe_audio(
    audio_data: Vec<f32>,
    original_sample_rate: u32
) -> Result<String, String> {
    dotenvy::dotenv().ok();
    let api_key = env::var("GROQ_API_KEY").map_err(|_| "API Key missing")?;

    // Downsample logic (e.g., 48k to 16k is a factor of 3)
    let factor = (original_sample_rate / 16000).max(1) as usize;
    let downsampled: Vec<f32> = audio_data.iter().step_by(factor).cloned().collect();

    let wav_bytes = crate::audio::convert_to_wav(downsampled, 16000);

    let client = reqwest::Client::new();
    let form = multipart::Form
        ::new()
        .text("model", "whisper-large-v3")
        .part(
            "file",
            multipart::Part::bytes(wav_bytes).file_name("audio.wav").mime_str("audio/wav").unwrap()
        );

    let res = client
        .post("https://api.groq.com/openai/v1/audio/transcriptions")
        .header("Authorization", format!("Bearer {}", api_key))
        .multipart(form)
        .send().await
        .map_err(|e| e.to_string())?;

    let json: Value = res.json().await.map_err(|e| e.to_string())?;
    println!(">>> [API RESULT] {:?}", json);
    Ok(json["text"].as_str().unwrap_or("").to_string())
}

#[tauri::command]
pub async fn ask_ai(prompt: String) -> Result<String, String> {
    dotenvy::dotenv().ok();
    let api_key = env::var("GROQ_API_KEY").map_err(|_| "API Key missing".to_string())?;

    let client = reqwest::Client::new();

    let body =
        json!({
        "model": "whisper-large-v3",
        "messages": [
            {
                "role": "system",
                "content": "You are LightForth Copilot."
            },
            {
                "role": "user",
                "content": prompt
            }
        ]
    });

    let res = client
        .post("http://localhost:3000/api/se")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&body)
        .send().await
        .map_err(|e| e.to_string())?;

    let status = res.status();
    let text = res.text().await.map_err(|e| e.to_string())?;

    println!(">>> STATUS: {}", status);
    println!(">>> RAW RESPONSE: {}", text);

    let json: Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    // 🔴 CRITICAL: handle API errors
    if let Some(err) = json.get("error") {
        return Err(format!("Groq API error: {}", err));
    }

    let content = json
        .get("choices")
        .and_then(|c| c.get(0))
        .and_then(|c| c.get("message"))
        .and_then(|m| m.get("content"))
        .and_then(|c| c.as_str());

    match content {
        Some(text) if !text.is_empty() => Ok(text.to_string()),
        _ => Err(format!("Empty response: {}", json)),
    }
}
