use std::{f32::consts::TAU, fs::File, io::BufReader};

use hound::WavReader;
use num_complex::Complex;

type Wav = WavReader<BufReader<File>>;

pub struct Modulator {
    wav: Wav,

    sample_rate: u64,
    bandwidth: f32,

    i: u64,
    sample: f32,
    phase: f32,
}

impl Modulator {
    pub fn new(sample_rate: u32, bandwidth: f32, wav: Wav) -> Self {
        assert!(sample_rate >= wav.spec().sample_rate);

        Self {
            wav,
            sample_rate: sample_rate as _,
            bandwidth,

            i: 0,
            sample: 0.0,
            phase: 0.0,
        }
    }

    pub fn progress(&self) -> f32 {
        let t = (self.i / self.sample_rate) as f32 * self.wav.spec().sample_rate as f32
            / self.wav.duration() as f32;
        t.min(1.0)
    }

    pub fn sample(&mut self) -> Complex<f32> {
        let spec = self.wav.spec();
        let rate = self.sample_rate / spec.sample_rate as u64;

        let sample = self.i % rate;
        if sample == 0 {
            let channels = spec.channels;
            let mut iter = self.wav.samples().step_by(channels as usize);
            self.sample = iter.next().unwrap().unwrap();
        }

        self.i += 1;

        let deviation = self.sample * self.bandwidth;
        self.phase += TAU * deviation / self.sample_rate as f32;

        (Complex::i() * self.phase).exp()
    }
}
