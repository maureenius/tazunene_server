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
        let target = self.repository.find_by_id(1).await?;

        self.generator.generate(target, request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domains::{character::{Character, CharacterName, Personality}, infra_trait::{MockCharacterRepository, MockTextGenerator}};

    #[tokio::test]
    async fn test_chat_service() {
        // Setup
        let character_name = CharacterName::new("Test Name");
        let character_personality = Personality::new("Test Personality");
        let character = Character::new(&character_name, &character_personality);

        let request = String::from("Request");

        let mut mock_generator = MockTextGenerator::new();
        mock_generator.expect_generate().returning(move |target, request| {
            assert_eq!(target, character);
            assert_eq!(request, "Request");

            Ok(String::from("Generated text"))
        });
        let mock_generator_arc = Arc::new(mock_generator);
        
        let mut mock_repo = MockCharacterRepository::new();
        mock_repo.expect_find_by_id().returning(move |id| {
            assert_eq!(id, 1);

            Ok(Character::new(&character_name, &character_personality))
        });
        let mock_repo_arc = Arc::new(mock_repo);

        let chat_service = ChatService::new(mock_generator_arc, mock_repo_arc);

        // Exercise
        let result = chat_service.generate_text(request).await;

        // Verify
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Generated text");
    }
}
