use crate::domains::infra_trait::TextGenerator;

pub struct ChatService<T: TextGenerator> {
    generator: T,
}

impl <T: TextGenerator> ChatService<T> {
    pub fn new(generator: T) -> Self {
        Self { generator }
    }

    pub async fn generate_text(&self, request: String) -> anyhow::Result<String> {
        self.generator.generate(request).await
    }
    
}
