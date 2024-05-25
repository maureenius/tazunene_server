use vvcore::{AccelerationMode, CPointerWrap, ResultCode, VoicevoxCore};

pub struct VoicevoxClient {
    core: VoicevoxCore,
    actor_id: u32,
}
impl VoicevoxClient {
    pub fn new(jtalk_path: &str) -> Self {
        Self { core: create_vv(jtalk_path), actor_id: 14 }
    }

    pub fn speak(&self, text: &str) -> Result<CPointerWrap<u8>, ResultCode> {
        self.core.tts_simple(text, self.actor_id)
    }
}

fn create_vv(path: &str) -> VoicevoxCore {
    let dir = std::ffi::CString::new(path).unwrap();
    VoicevoxCore::new_from_options(AccelerationMode::Auto, 0, true, dir.as_c_str()).unwrap()
}

