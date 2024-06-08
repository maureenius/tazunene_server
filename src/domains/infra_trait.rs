#[cfg(test)]
use mockall::automock;

use super::character::{Character, CharacterName};

pub trait VoiceSynthesizer {
    fn synthesize(&self, text: &str) -> anyhow::Result<Vec<u8>>;
}

#[cfg_attr(test, automock)]
pub trait TextGenerator {
    async fn generate(&self, target: Character, request: String) -> anyhow::Result<String>;
}

#[cfg_attr(test, automock)]
pub trait CharacterRepository {
    async fn find_by_id(&self, id: u64) -> anyhow::Result<Character>;
    
    async fn find_by_name(&self, name: &CharacterName) -> anyhow::Result<Character>;
    
    async fn create(&self, character: &Character) -> anyhow::Result<Character>;
    async fn update(&self, before: &Character, after: &Character) -> anyhow::Result<Character>;
}
