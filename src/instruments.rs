mod basic_waves;
pub use basic_waves::*;

use crate::{
    sampling::{MixerSamples, Sample},
    song::{Note, Instrument},
};

pub trait WaveFunction {
    fn reset(&mut self);

    fn sample_at(&mut self, delta_seconds: f32, freq: f32) -> f32;
}

impl <T: WaveFunction> Instrument for T {
    fn reset(&mut self) {  }

    fn sample_note<'a>(&mut self, note: &Note, mut mixer_samples: MixerSamples<'a>) {
        let freq = note.name.freq();
        let delta_seconds = 1.0 / mixer_samples.sample_rate;
        for sample_index in 0..mixer_samples.total_samples() {
            let raw_sample = self.sample_at(delta_seconds, freq);
            let sample = Sample::max_value() as f32 * raw_sample;
            mixer_samples.mix_sample(sample_index, sample as Sample);
        }
    }

    fn can_use_note_names(&self) -> bool { true }
}
