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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_prompt() {
        let diff = "diff --git ...";
        let prompt = build_prompt(diff);
        assert!(prompt.contains(diff));
        assert!(prompt.contains("Conventional Commits"));
    }

    #[test]
    fn test_build_request_body() {
        let prompt = "test prompt";
        let body = build_request_body(prompt);
        assert_eq!(body["contents"][0]["parts"][0]["text"], prompt);
    }

    #[test]
    fn test_parse_gemini_response_success() {
        let json = serde_json::json!({
            "candidates": [{
                "content": {
                    "parts": [{
                        "text": "Fixed a bug"
                    }]
                }
            }]
        });
        let result = parse_gemini_response(&json);
        assert_eq!(result, Ok("Fixed a bug".to_string()));
    }

    #[test]
    fn test_parse_gemini_response_error() {
        let json = serde_json::json!({});
        let result = parse_gemini_response(&json);
        assert!(result.is_err());
    }
}
