use crate::Beat;

pub type Sample = i16;

pub struct SamplingProperties {
    pub start_beat: Beat,
    /// If a beat starts on or after the cutoff, don't sample it
    pub cutoff_beat: Beat,
    pub sample_rate: f32,
    pub bpm: f32,
    // TODO We may want to keep track of the total song time up to the start_beat
    //  This would help to create better waves on timing changes
}
impl SamplingProperties {
    fn num_samples(&self) -> usize {
        let beat_length = self.cutoff_beat - self.start_beat;
        let seconds = crate::beat_in_seconds(&beat_length, self.bpm);
        let num_samples = self.sample_rate as f32 * seconds;
        num_samples as usize
    }
}

pub struct Mixer {
    properties: SamplingProperties,
    samples: Vec<Sample>,
}
impl Mixer {
    pub fn new(properties: SamplingProperties) -> Mixer {
        let samples = vec![0; properties.num_samples()];
        Mixer {
            properties,
            samples,
        }
    }

    /// Re-use the allocated samples vector from an old mixer.
    /// This will only use the new properties passed in.
    pub fn from_old_mixer(mut old_mixer: Mixer, properties: SamplingProperties) -> Mixer {
        old_mixer.samples.clear();
        old_mixer.samples.resize(properties.num_samples(), 0);
        Mixer {
            properties,
            samples: old_mixer.samples,
        }
    }

    pub fn properties(&self) -> &SamplingProperties { &self.properties }
    pub fn iter_samples(&self) -> impl Iterator<Item = &Sample> { self.samples.iter() }

    pub fn samples_for_beats(&mut self, start_beat: Beat, beat_length: Beat,
        sound_level: f32) -> MixerSamples {
        let seconds_at_start = crate::beat_in_seconds(&start_beat, self.properties.bpm);
        let cutoff_beat =  start_beat + beat_length;
        let start_index = {
            let num_samples = self.properties.sample_rate as f32 * seconds_at_start;
            num_samples as usize
        };
        let end_index = {
            let seconds = crate::beat_in_seconds(&cutoff_beat, self.properties.bpm);
            let num_samples = self.properties.sample_rate as f32 * seconds;
            num_samples as usize
        };
        MixerSamples {
            samples: &mut self.samples[start_index .. end_index],
            sound_level,
            sample_rate: self.properties.sample_rate,
        }
    }
}

pub struct MixerSamples<'a> {
    samples: &'a mut [Sample],
    sound_level: f32,
    pub sample_rate: f32,
}
impl <'a> MixerSamples<'a> {
    pub fn total_samples(&self) -> usize { self.samples.len() }

    pub fn mix_sample(&mut self, index: usize, sample: f32) {
        let sample = Sample::max_value() as f32 * sample * self.sound_level;
        self.samples[index] += sample as Sample;
    }
}
