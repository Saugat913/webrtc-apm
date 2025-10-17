# Webrtc-Apm(Audio Processing module)

webrtc-apm is a safe, idiomatic Rust wrapper around the WebRTC Audio Processing Module (APM)
, providing real-time features such as noise suppression, echo cancellation, automatic gain control, and high-pass filtering  bound through webrtc-apm-sys FFI layer.

## Quick Example
```rust
use webrtc_apm::{
    AudioProcessor, StreamConfig, WebrtcApmAudioProcessingConfig,
    WebrtcApmNoiseSuppressionLevel, WebrtcApmError,
};

fn main() -> Result<(), WebrtcApmError> {
    // Create and initialize the processor
    let mut processor = AudioProcessor::new()?;
    processor.initialize()?;

    // Configure processing behavior
    let mut config = WebrtcApmAudioProcessingConfig::default();
    config.config.ns_level =
        WebrtcApmNoiseSuppressionLevel::VeryHigh.into();
    processor.apply_config(&config)?;

    // Stream setup
    let input_cfg = StreamConfig::new(48000, 1).unwrap();
    let output_cfg = StreamConfig::new(48000, 1).unwrap();

    // Dummy buffers (replace with mic data)
    let input = vec![0i16; 480];
    let mut output = vec![0i16; 480];

    // Process stream
    processor.process_stream_int16(&input_cfg, &output_cfg, &input, &mut output)?;
    println!("✅ Processed {} samples successfully!", output.len());

    Ok(())
}
```