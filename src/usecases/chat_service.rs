use std::sync::Arc;

use crate::domains::infra_trait::{CharacterRepository, TextGenerator};

pub struct ChatService<T: TextGenerator, CR: CharacterRepository> {
    generator: Arc<T>,
    repository: Arc<CR>,
}

impl <T: TextGenerator, CR: CharacterRepository> ChatService<T, CR> {
    pub fn new(generator: Arc<T>, repository: Arc<CR>) -> Self {
        Self { generator, repository }
    }

    pub async fn generate_text(&self, request: String) -> anyhow::Result<String> {
        let target = self.repository.find_by_id(1)?;

        self.generator.generate(target, request).await
    }
}
