pub struct StreamConfig {
    pub handle: webrtc_apm_sys::WebRtcStreamConfigHandle,
}

impl StreamConfig {
    pub fn new(sample_rate_hz: i32, num_channels: usize) -> Option<Self> {
        let handle = unsafe {
            webrtc_apm_sys::webrtc_apm_stream_config_create(sample_rate_hz, num_channels)
        };
        if handle.is_null() {
            return None;
        }
        Some(Self { handle })
    }
}

impl Drop for StreamConfig {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe { webrtc_apm_sys::webrtc_apm_stream_config_destroy(self.handle) };
        }
    }
}

unsafe impl Send for StreamConfig {}
