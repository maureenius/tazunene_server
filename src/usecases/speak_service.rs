use crate::domains::infra_trait::VoiceSynthesizer;
use anyhow::Result;

pub struct SpeakService<T: VoiceSynthesizer> {
    synthesizer: T,
}

impl<T: VoiceSynthesizer> SpeakService<T> {
    pub fn new(synthesizer: T) -> Self {
        Self { synthesizer }
    }

    pub fn synthesize_speech(&self, text: &str) -> Result<Vec<u8>> {
        self.synthesizer.synthesize(text)
    }
}
