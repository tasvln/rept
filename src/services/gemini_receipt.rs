use base64::{Engine as _, engine::general_purpose};
use reqwest::Client;
use serde::{Deserialize, Serialize};

// Gemini expects a "contents" array -> So Gemini Request
#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<Content>,
}

// each content has "parts" — text or image
#[derive(Serialize)]
struct Content {
    parts: Vec<Parts>,
}

#[derive(Serialize)]
#[serde(untagged)]
enum Parts {
    Text { text: String },
    Image { inline_data: InlineData },
}

#[derive(Serialize)]
struct InlineData {
    mime_type: String,
    data: String,
}

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
struct Candidate {
    content: CandidateContent,
}

#[derive(Deserialize)]
struct CandidateContent {
    parts: Vec<TextPart>,
}

#[derive(Deserialize)]
struct TextPart {
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReceiptData {
    pub store_name: Option<String>,
    pub purchase_date: Option<String>,
    pub total: Option<f64>,
    pub return_by: Option<String>,
    pub warranty_until: Option<String>,
}

pub async fn extract_receipt(image_bytes: Vec<u8>, gemini_key: &str) -> ReceiptData {
    let client = Client::new();
    let base64_image = general_purpose::STANDARD.encode(&image_bytes);

    let request = GeminiRequest {
        contents: vec![Content {
            parts: vec![
                Parts::Image {
                    inline_data: InlineData {
                        mime_type: "image/jpeg".to_string(),
                        data: base64_image,
                    },
                },
                Parts::Text {
                    text: r#"Extract the following from this receipt and respond ONLY in JSON with no markdown:
                    {
                        "store_name": "string or null",
                        "purchase_date": "YYYY-MM-DD or null",
                        "total": number or null,
                        "return_by": "YYYY-MM-DD or null",
                        "warranty_until": "YYYY-MM-DD or null"
                    }"#.to_string(),
                },
            ],
        }],
    };

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-3-flash-preview:generateContent?key={}",
        gemini_key
    );

    let response = client.post(&url).json(&request).send().await.unwrap();

    let raw = response.text().await.unwrap();
    println!("Gemini response: {}", raw);

    let gemini_response: GeminiResponse = serde_json::from_str(&raw).unwrap();
    let text = &gemini_response.candidates[0].content.parts[0].text;

    serde_json::from_str::<ReceiptData>(text).unwrap_or(ReceiptData {
        store_name: None,
        purchase_date: None,
        total: None,
        return_by: None,
        warranty_until: None,
    })
}
