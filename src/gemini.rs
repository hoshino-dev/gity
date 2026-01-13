pub async fn generate_commit_message(
    diff_text: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let api_key = crate::config::get_api_key()?;

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-3-flash-preview:generateContent?key={}",
        api_key
    );

    let client = reqwest::Client::new();
    let prompt = build_prompt(diff_text);
    let request_body = build_request_body(&prompt);

    let response = client.post(&url).json(&request_body).send().await?;

    if !response.status().is_success() {
        return Err(format!("API request failed with status: {}", response.status()).into());
    }

    let response_json: serde_json::Value = response.json().await?;
    let generated_text = parse_gemini_response(&response_json)?;

    Ok(generated_text)
}

fn build_prompt(diff_text: &str) -> String {
    format!(
        "Generate a git commit message for the following diff following Conventional Commits specification.
Rules:
- Use imperative mood (e.g., 'Add feature' not 'Added feature').
- First line: <type>: <description> (max 50 chars).
- Leave one blank line after the title.
- Then provide a concise bulleted list of details.
- Output ONLY the commit message, no markdown code blocks or conversational text.

Diff:
{}",
        diff_text
    )
}

fn build_request_body(prompt: &str) -> serde_json::Value {
    serde_json::json!({
        "contents": [{
            "parts": [{
                "text": prompt
            }]
        }]
    })
}

fn parse_gemini_response(response_json: &serde_json::Value) -> Result<String, String> {
    response_json["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .ok_or_else(|| "Failed to parse generated text from response".to_string())
        .map(|s| s.trim().to_string())
}
