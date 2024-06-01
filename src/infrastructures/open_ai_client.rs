use reqwest::Client;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::domains::{character::Character, infra_trait::TextGenerator};

#[derive(Debug, Clone)]
pub struct ApiKey(String);
impl ApiKey {
    pub fn new(key: &str) -> Self {
        Self(key.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct ChatRequest {
    personality_message: String,
    content_message: String,
}
impl ChatRequest {
    pub fn new(personality_message: &str, content_message: &str) -> Self {
        Self {
            personality_message: personality_message.to_string(),
            content_message: content_message.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
struct ResponseFormat {
    #[serde(rename = "type")]
    type_: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChatCompletionsRequest {
    model: ModelName,
    messages: Vec<ChatCompletionsMessage>,
    response_format: Option<ResponseFormat>,
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
        Self::new_with_base_url(api_key, &url::Url::parse("https://api.openai.com").unwrap())
    }

    pub fn new_with_base_url(api_key: &ApiKey, base_url: &Url) -> Self {
        Self {
            api_key: api_key.clone(),
            client: Client::new(),
            base_url: base_url.clone(),
        }
    }

    pub async fn chat(&self, message: &ChatRequest) -> anyhow::Result<ChatResponse> {
        let response = self.chat_completions(&ChatCompletionsRequest {
            model: ModelName::Gpt4o,
            messages: vec![ChatCompletionsMessage {
                role: Role::System,
                content: Content(message.personality_message.clone()),
            },
                ChatCompletionsMessage {
                role: Role::User,
                content: Content(message.content_message.clone()),
            }],
            response_format: Some(ResponseFormat {
                type_: "json_object".to_string(),
            }),
        }).await?;

        let response: ChatResponse = serde_json::from_str(&response.choices[0].message.content.0)?;

        Ok(response)
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

    fn system_prompt(&self) -> String {
        r#"
        あなたはこれから、幅広いトピックについて会話をする可愛いキャラクターのチャットボットとして振る舞います。ユーザーから話題が提供されたら、json形式に則って以下の要領でその話題について会話を進めてください。

        {
            "message": "あなたの返答文字列"
        }

        - クールなキャラクター風の口調を使う
        - 与えられた話題について、行ったり来たりの会話を続ける
        - 会話を盛り上げるためにフォローアップの質問をする 
        - 関連する知識、意見、経験などを共有する
        - フレンドリーで共感的な態度を示し、温かい絆を築くことを目指す

        会話を通して、ずっとキャラクターを演じ続けるのを忘れないでください。
        "#.to_string()
    }
}
impl TextGenerator for OpenAiClient {
    async fn generate(&self, target: Character, request: String) -> anyhow::Result<String> {
        let personality_message: String = target.personality.into();
        let response = self.chat(&ChatRequest::new(personality_message.as_str(), &request)).await?;
        Ok(response.message)
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
                            "content": "{\n  \"message\": \"Hello, from the other side!\"\n}"
                        }
                    }
                ]
            }"#)
            .create();

        let api_key = ApiKey("test_api_key".to_string());
        let client = OpenAiClient::new_with_base_url(&api_key, &Url::parse(&server.url()).unwrap());
        let request = ChatRequest {
            personality_message: "I am tester".to_string(),
            content_message: "Hello, world!".to_string(),
        };

        let response = client.chat(&request).await.expect("Failed to get response");

        assert_eq!(response.message, "Hello, from the other side!");
    }
}