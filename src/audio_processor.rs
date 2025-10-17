use crate::{config::StreamConfig, error::WebrtcApmError};

pub enum WebrtcApmNoiseSuppressionLevel {
    Low,
    Moderate,
    High,
    VeryHigh,
}

impl From<WebrtcApmNoiseSuppressionLevel> for webrtc_apm_sys::WebRtcApmNoiseSuppressionLevel {
    fn from(value: WebrtcApmNoiseSuppressionLevel) -> Self {
        match value {
            WebrtcApmNoiseSuppressionLevel::High => {
                webrtc_apm_sys::WebRtcApmNoiseSuppressionLevel_WEBRTC_APM_NS_HIGH
            }
            WebrtcApmNoiseSuppressionLevel::Low => {
                webrtc_apm_sys::WebRtcApmNoiseSuppressionLevel_WEBRTC_APM_NS_LOW
            }
            WebrtcApmNoiseSuppressionLevel::Moderate => {
                webrtc_apm_sys::WebRtcApmNoiseSuppressionLevel_WEBRTC_APM_NS_MODERATE
            }
            WebrtcApmNoiseSuppressionLevel::VeryHigh => {
                webrtc_apm_sys::WebRtcApmNoiseSuppressionLevel_WEBRTC_APM_NS_VERY_HIGH
            }
        }
    }
}
pub struct WebrtcApmAudioProcessingConfig {
    config: webrtc_apm_sys::WebRtcAudioProcessingConfig,
}

impl Default for WebrtcApmAudioProcessingConfig {
    fn default() -> Self {
        let config = webrtc_apm_sys::WebRtcAudioProcessingConfig {
            enable_noise_suppression: true,
            ns_level: WebrtcApmNoiseSuppressionLevel::Moderate.into(),
            enable_echo_cancellation: true,
            echo_mobile_mode: false,
            enable_high_pass_filter: true,
            enable_analog_gain_control: false,
            agc_target_level_dbfs: -3,
            agc_compression_gain_db: 9,
            agc_enable_limiter: true,
        };
        Self { config: config }
    }
}

pub struct WebrtcApmAudioProcessingConfigBuilder {
    cfg: webrtc_apm_sys::WebRtcAudioProcessingConfig,
}

impl WebrtcApmAudioProcessingConfigBuilder {
    pub fn new() -> Self {
        Self {
            cfg: WebrtcApmAudioProcessingConfig::default().config,
        }
    }

    pub fn noise_suppression_level(mut self, level: WebrtcApmNoiseSuppressionLevel) -> Self {
        self.cfg.ns_level = level.into();
        self
    }

    pub fn echo_cancellation(mut self, enable: bool) -> Self {
        self.cfg.enable_echo_cancellation = enable;
        self
    }

    pub fn build(self) -> WebrtcApmAudioProcessingConfig {
        WebrtcApmAudioProcessingConfig { config: self.cfg }
    }
}

pub struct AudioProcessor {
    handle: webrtc_apm_sys::WebRtcAudioProcessingHandle,
}

impl AudioProcessor {
    pub fn new() -> Option<Self> {
        let handle = unsafe { webrtc_apm_sys::webrtc_apm_create() };
        if handle.is_null() {
            return None;
        }
        Some(Self { handle })
    }

    /// Initialize the audio processor
    pub fn initialize(&mut self) -> Result<(), WebrtcApmError> {
        let err = unsafe { webrtc_apm_sys::webrtc_apm_initialize(self.handle) };
        let err = WebrtcApmError::from(err);
        if err.is_ok() { Ok(()) } else { Err(err) }
    }

    /// Apply configuration
    pub fn apply_config(
        &mut self,
        config: &WebrtcApmAudioProcessingConfig,
    ) -> Result<(), WebrtcApmError> {
        let err = unsafe {
            webrtc_apm_sys::webrtc_apm_apply_config(self.handle, &config.config as *const _)
        };
        let err = WebrtcApmError::from(err);
        if err.is_ok() { Ok(()) } else { Err(err) }
    }

    /// Process audio stream (int16)
    pub fn process_stream_int16(
        &mut self,
        input_config: &StreamConfig,
        output_config: &StreamConfig,
        src: &[i16],
        dest: &mut [i16],
    ) -> Result<(), WebrtcApmError> {
        let err = unsafe {
            webrtc_apm_sys::webrtc_apm_process_stream_int16(
                self.handle,
                input_config.handle,
                output_config.handle,
                src.as_ptr(),
                dest.as_mut_ptr(),
            )
        };
        let err = WebrtcApmError::from(err);
        if err.is_ok() { Ok(()) } else { Err(err) }
    }

    /// Process reverse stream (int16) for echo cancellation
    pub fn process_reverse_stream_int16(
        &mut self,
        input_config: &StreamConfig,
        output_config: &StreamConfig,
        src: &[i16],
        dest: &mut [i16],
    ) -> Result<(), WebrtcApmError> {
        let err = unsafe {
            webrtc_apm_sys::webrtc_apm_process_reverse_stream_int16(
                self.handle,
                input_config.handle,
                output_config.handle,
                src.as_ptr(),
                dest.as_mut_ptr(),
            )
        };
        let err = WebrtcApmError::from(err);
        if err.is_ok() { Ok(()) } else { Err(err) }
    }

    /// Set stream delay for echo cancellation (in milliseconds)
    pub fn set_stream_delay_ms(&mut self, delay_ms: i32) -> Result<(), WebrtcApmError> {
        let err = unsafe { webrtc_apm_sys::webrtc_apm_set_streamdelay_ms(self.handle, delay_ms) };
        let err = WebrtcApmError::from(err);
        if err.is_ok() { Ok(()) } else { Err(err) }
    }
}

impl Drop for AudioProcessor {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe { webrtc_apm_sys::webrtc_apm_destroy(self.handle) };
        }
    }
}

unsafe impl Send for AudioProcessor {}
