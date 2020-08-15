mod basic_waves;
pub use basic_waves::*;

use crate::{
    sampling::{MixerSamples, Sample},
    song::{Note, Instrument, NoteType},
};

pub trait WaveFunction {
    fn reset(&mut self);

    fn sample_at(&mut self, note_channel: usize, delta_seconds: f32, freq: f32) -> f32;
}

impl <T: WaveFunction> Instrument for T {
    fn reset(&mut self) {  }

    fn sample_note<'a>(&mut self, note: &Note, mut mixer_samples: MixerSamples<'a>) {
        let delta_seconds = 1.0 / mixer_samples.sample_rate;
        match note.note_type {
            NoteType::Single(note_name) => {
                let freq = note_name.freq();
                let total_samples = mixer_samples.total_samples();
                for sample_index in 0..total_samples {
                    let mut decay = 1.0 - (sample_index as f32 / total_samples as f32);
                    if decay < 0.5 {
                        decay = 0.5;
                    }
                    let raw_sample = self.sample_at(0, delta_seconds, freq);
                    mixer_samples.mix_sample(sample_index, raw_sample * decay);
                }
            },
            NoteType::Chord3(note_name1, note_name2, note_name3) => {
                let freq1 = note_name1.freq();
                let freq2 = note_name2.freq();
                let freq3 = note_name3.freq();
                let total_samples = mixer_samples.total_samples();
                for sample_index in 0..total_samples {
                    let mut decay = 1.0 - (sample_index as f32 / total_samples as f32);
                    if decay < 0.5 {
                        decay = 0.5;
                    }
                    let raw_sample1 = self.sample_at(0, delta_seconds, freq1) / 3.0;
                    let raw_sample2 = self.sample_at(1, delta_seconds, freq2) / 3.0;
                    let raw_sample3 = self.sample_at(2, delta_seconds, freq3) / 3.0;

                    let sample = (raw_sample1 + raw_sample2 + raw_sample3) * decay;
                    mixer_samples.mix_sample(sample_index, sample);
                }
            },
            _ => todo!(),
        }
    }

    fn can_use_note_names(&self) -> bool { true }
}
