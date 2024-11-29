use std::iter;

use itertools::Itertools;
use num_complex::Complex;
use num_traits::Zero;

use crate::{
    consts::{AUDIO_CUTOFF_FREQ, IQ_CUTOFF_FREQ, SAMPLE_RATE, WAVE_SAMPLE_RATE},
    filters::{down_sample::DownSampleExt, low_pass::LowPassExt, offset::OffsetExt},
};

pub struct Demodulator {
    iq: Vec<Complex<f32>>,
    last_sample: Complex<f32>,
}

impl Demodulator {
    pub fn new() -> Self {
        Self {
            iq: Vec::new(),
            last_sample: Complex::zero(),
        }
    }

    pub fn replace(&mut self, data: Vec<Complex<f32>>) {
        self.iq = data;
    }

    pub fn audio(&mut self, offset: f32, gain: f32) -> Vec<f32> {
        self.last_sample = self.iq.last().copied().unwrap_or_default();
        let mut audio = iter::once(self.last_sample)
            .chain(self.iq.iter().copied())
            .offset(offset, SAMPLE_RATE)
            .low_pass(SAMPLE_RATE, IQ_CUTOFF_FREQ)
            .tuple_windows()
            .map(|(a, b)| (b * a.conj()).arg() * gain)
            .low_pass(SAMPLE_RATE, AUDIO_CUTOFF_FREQ)
            .down_sample(SAMPLE_RATE, WAVE_SAMPLE_RATE)
            .collect::<Vec<_>>();

        let dc = audio.iter().copied().sum::<f32>() / audio.len() as f32;
        audio.iter_mut().for_each(|x| *x -= dc);

        audio
    }
}
