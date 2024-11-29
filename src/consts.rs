use hound::{SampleFormat, WavSpec};

pub const SAMPLE_RATE: u32 = 2_000_000;

pub const IQ_CUTOFF_FREQ: f32 = 200_000.0;
pub const AUDIO_CUTOFF_FREQ: f32 = 22_000.0;

pub const WAVE_SAMPLE_RATE: u32 = 44_100;
pub const WAVE_SPEC: WavSpec = WavSpec {
    channels: 1,
    sample_rate: WAVE_SAMPLE_RATE,
    bits_per_sample: 32,
    sample_format: SampleFormat::Float,
};
