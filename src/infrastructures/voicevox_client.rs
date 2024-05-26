use vvcore::{AccelerationMode, VoicevoxCore};

use crate::domains::infra_trait::VoiceSynthesizer;

pub struct VoicevoxClient {
    core: VoicevoxCore,
    actor_id: u32,
}
impl VoicevoxClient {
    pub fn new(jtalk_path: &str) -> Self {
        Self { core: create_vv(jtalk_path), actor_id: 14 }
    }
}
impl VoiceSynthesizer for VoicevoxClient {
    fn synthesize(&self, text: &str) -> anyhow::Result<Vec<u8>> {
        Ok(
            self.core.tts_simple(text, self.actor_id)
            .map_err(|e| anyhow::anyhow!("Voice synthesis failed: {:?}", e))?
            .as_slice()
            .to_vec()
        )
    }
}

fn create_vv(path: &str) -> VoicevoxCore {
    let dir = std::ffi::CString::new(path).unwrap();
    VoicevoxCore::new_from_options(AccelerationMode::Auto, 0, true, dir.as_c_str()).unwrap()
}

