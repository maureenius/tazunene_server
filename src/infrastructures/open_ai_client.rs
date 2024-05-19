use reqwest::Client;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone)]
pub struct ApiKey(String);
impl ApiKey {
    pub fn new(key: &str) -> Self {
        Self(key.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct ChatRequest {
    message: String,
}
impl ChatRequest {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChatResponse {
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum ModelName {
    #[serde(rename = "gpt-4o")]
    Gpt4o,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Role {
    System,
    User,
    Assistant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Content(String);

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChatCompletionsMessage {
    role: Role,
    content: Content,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChatCompletionsRequest {
    model: ModelName,
    messages: Vec<ChatCompletionsMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChatCompletionsChoice {
    message: ChatCompletionsMessage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChatCompletionsResponse {
    choices: Vec<ChatCompletionsChoice>
}

pub struct OpenAiClient {
    api_key: ApiKey,
    client: Client,
    base_url: Url,
}
impl OpenAiClient {
    pub fn new(api_key: &ApiKey) -> Self {
        Self { 
            api_key: api_key.clone(),
            client: Client::new(),
            base_url: url::Url::parse("https://api.openai.com").unwrap(),
        }
    }

    pub async fn chat(&self, message: &ChatRequest) -> anyhow::Result<ChatResponse> {
        let response = self.chat_completions(&ChatCompletionsRequest {
            model: ModelName::Gpt4o,
            messages: vec![ChatCompletionsMessage {
                role: Role::User,
                content: Content(message.message.clone()),
            }],
        }).await?;

        Ok(ChatResponse {
            message: response.choices[0].message.content.0.clone(),
        })
    }

    async fn chat_completions(&self, request: &ChatCompletionsRequest) -> anyhow::Result<ChatCompletionsResponse> {
        let url = self.base_url.join("/v1/chat/completions").unwrap();

        let response = self.client.post(url)
            .header("Authorization", format!("Bearer {}", &self.api_key.0))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let chat_response: ChatCompletionsResponse = response
                .json()
                .await?;
            Ok(chat_response)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(anyhow::anyhow!("Request failed with status {}: {}", status, error_text))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_chat() {
        let mut server = mockito::Server::new_async().await;

        let _m = server
            .mock("POST", "/v1/chat/completions")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{
                "choices": [
                    {
                        "message": {
                            "role": "assistant",
                            "content": "Hello, from the other side!"
                        }
                    }
                ]
            }"#)
            .create();

        let api_key = ApiKey("test_api_key".to_string());
        let client = OpenAiClient::new(&api_key);
        let request = ChatRequest {
            message: "Hello, world!".to_string(),
        };

        let response = client.chat(&request).await.expect("Failed to get response");

        assert_eq!(response.message, "Hello, from the other side!");
    }
}