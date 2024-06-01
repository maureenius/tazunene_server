use crate::domains::{character::Character, infra_trait::TextGenerator};

pub struct ChatService<T: TextGenerator> {
    generator: T,
}

impl <T: TextGenerator> ChatService<T> {
    pub fn new(generator: T) -> Self {
        Self { generator }
    }

    pub async fn generate_text(&self, target: Character, request: String) -> anyhow::Result<String> {
        self.generator.generate(target, request).await
    }
    
}
