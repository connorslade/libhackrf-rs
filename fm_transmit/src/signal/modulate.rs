use std::{f32::consts::TAU, fs::File, io::BufReader};

use hound::{SampleFormat, WavReader};
use num_complex::Complex;

type Wav = WavReader<BufReader<File>>;

pub struct Modulator {
    samples: Box<dyn Iterator<Item = f32>>,
    audio_sample_rate: u32,
    audio_samples: u32,
    sample_rate: u64,
    bandwidth: f32,

    i: u64,
    phase: f32,
    sample: f32,
    next_sample: f32,
}

impl Modulator {
    pub fn new(sample_rate: u32, bandwidth: f32, wav: Wav) -> Self {
        let audio_sample_rate = wav.spec().sample_rate;
        let audio_samples = wav.duration();
        let channels = wav.spec().channels;

        assert!(sample_rate >= audio_sample_rate);

        let samples: Box<dyn Iterator<Item = f32>> = match wav.spec().sample_format {
            SampleFormat::Float => Box::new(
                wav.into_samples::<f32>()
                    .map(|x| x.unwrap())
                    .step_by(channels as usize),
            ),
            SampleFormat::Int => {
                let max = (1u32 << (wav.spec().bits_per_sample - 1)) as f32;
                Box::new(
                    wav.into_samples::<i32>()
                        .map(move |x| x.unwrap() as f32 / max)
                        .step_by(channels as usize),
                )
            }
        };

        Self {
            samples,
            audio_sample_rate,
            audio_samples,
            sample_rate: sample_rate as _,
            bandwidth,

            i: 0,
            phase: 0.0,
            sample: 0.0,
            next_sample: 0.0,
        }
    }

    pub fn progress(&self) -> f32 {
        let t = (self.i / self.sample_rate) as f32 * self.audio_sample_rate as f32
            / self.audio_samples as f32;
        t.min(1.0)
    }

    pub fn sample(&mut self) -> Complex<f32> {
        let rate = self.sample_rate / self.audio_sample_rate as u64;

        let sample = self.i % rate;
        if sample == 0 {
            self.sample = self.next_sample;
            self.next_sample = self.samples.next().unwrap_or_default();
        }

        self.i += 1;

        let t = sample as f32 / rate as f32;
        let deviation = lerp(self.sample, self.next_sample, t) * self.bandwidth;
        self.phase += TAU * deviation / self.sample_rate as f32;

        (Complex::i() * self.phase).exp()
    }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}
