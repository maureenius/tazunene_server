use super::character::Character;

pub trait VoiceSynthesizer {
    fn synthesize(&self, text: &str) -> anyhow::Result<Vec<u8>>;
}

pub trait TextGenerator {
    async fn generate(&self, target: Character, request: String) -> anyhow::Result<String>;
}
