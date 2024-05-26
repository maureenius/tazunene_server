pub trait VoiceSynthesizer {
    fn synthesize(&self, text: &str) -> anyhow::Result<Vec<u8>>;
}
