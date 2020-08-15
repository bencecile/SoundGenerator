use std::{
    cmp::Ordering,
    path::Path
};

use crate::{
    Beat, TimeSignature,
    sampling::{Mixer, SamplingProperties, MixerSamples},
};
use hound::{SampleFormat, WavSpec, WavWriter};

pub struct Song {
    musicians: Vec<Musician>,
    /// Use the beat number to specify when a new timing will start
    timings: Vec<(Beat, Timing)>,
    // TODO We will want to have characteristics of the note (strong attack, weak decay, etc.)
}
impl Song {
    pub fn new(starting_timing: Timing) -> Song {
        let mut timings = Vec::new();
        timings.push( (crate::FIRST_BEAT, starting_timing) );
        Song {
            musicians: Vec::new(),
            timings,
        }
    }

    pub fn add_musician(&mut self, musician: Musician) { self.musicians.push(musician); }
    // pub fn get_musician(&mut self, index: usize) -> &mut Musician { &mut self.musicians[index] }

    pub fn export_to_wav(&mut self, file_path: impl AsRef<Path>) -> Result<(), String> {
        let sample_rate = 44100;
        let spec = WavSpec {
            channels: 1,
            sample_rate,
            bits_per_sample: 16,
            sample_format: SampleFormat::Int,
        };
        let mut wav_writer = WavWriter::create(file_path, spec)
            .map_err(|e| e.to_string())?;

        let mut mixer = None;
        for timing_index in 0..self.timings.len() {
            let cutoff_beat = if timing_index + 1 < self.timings.len() {
                // We can easily know the range if there's another timing after this one
                self.timings[timing_index + 1].0
            } else {
                if let Some(end_beat) = self.find_end_beat_of_last_note() {
                    end_beat
                } else {
                    break;
                }
            };
            // TODO We will want to split up the beat ranges so they won't get too large
            let (start_beat, timing) = &self.timings[timing_index];
            let properties = SamplingProperties {
                start_beat: *start_beat,
                cutoff_beat,
                sample_rate: sample_rate as f32,
                bpm: timing.bpm,
            };
            mixer = if let Some(old_mixer) = mixer.take() {
                Some(Mixer::from_old_mixer(old_mixer, properties))
            } else {
                Some(Mixer::new(properties))
            };
            for musician in &mut self.musicians {
                musician.reset();
                musician.sample_notes(mixer.as_mut().unwrap());
            }

            for sample in mixer.as_ref().unwrap().iter_samples() {
                wav_writer.write_sample(*sample)
                    .map_err(|e| e.to_string())?;
            }
        }

        wav_writer.finalize()
            .map_err(|e| e.to_string())
    }
}
impl Song {
    fn find_end_beat_of_last_note(&self) -> Option<Beat> {
        let mut end_beat = None;
        for musician in &self.musicians {
            if let Some(note) = musician.notes.last() {
                let note_end_beat = note.start_beat + note.beat_length;
                match end_beat {
                    Some(last_beat_value) => if note_end_beat > last_beat_value {
                        end_beat = Some(note_end_beat);
                    },
                    None => end_beat = Some(note_end_beat),
                }
            }
        }

        end_beat
    }
}

pub struct Musician {
    instrument: Box<dyn Instrument>,
    notes: Vec<Note>,
    // TODO We will probably want to be able to name a musician for the UI
}
impl Musician {
    pub fn new(instrument: impl Instrument + 'static) -> Musician {
        Musician {
            instrument: Box::new(instrument),
            notes: Vec::new(),
        }
    }

    /// 2 notes cannot overlap each other
    pub fn add_note(&mut self, note: Note) -> Result<(), String> {
        let insert_index = match self.notes.binary_search(&note) {
            Ok(index) => return Err(self.notes[index].note_collision_msg()),
            Err(index) => index,
        };
        // We just need to check the notes before and after to make sure there's no collisions
        if insert_index > 0 {
            let note_before = &self.notes[insert_index - 1];
            if (note_before.start_beat + note_before.beat_length) > note.start_beat {
                return Err(note_before.note_collision_msg());
            }
        }
        // If we wanted to insert at the end, len() would be the new position
        if insert_index < self.notes.len() {
            // Since it does a right-shift, the insert_index note will be the one after
            //  (after insert if there is no collision)
            let note_after = &self.notes[insert_index];
            if (note.start_beat + note.beat_length) > note_after.start_beat {
                return Err(note_after.note_collision_msg());
            }
        }

        self.notes.insert(insert_index, note);
        Ok(())
    }

    pub fn reset(&mut self) { self.instrument.reset(); }

    pub fn sample_notes(&mut self, mixer: &mut Mixer) {
        let note_index = match self.find_starting_note(mixer.properties())  {
            Some(index) => index,
            None => return,
        };
        let cutoff_beat = mixer.properties().cutoff_beat;
        for note in &self.notes[note_index ..] {
            if note.start_beat >= cutoff_beat {
                break;
            }
            let mixer_samples = mixer.samples_for_beats(
                note.start_beat, note.beat_length
            );
            self.instrument.sample_note(note, mixer_samples);
        }
    }
}
impl Musician {
    /// Try to find the first index of the note that starts on or after the beat
    fn find_starting_note(&self, properties: &SamplingProperties) -> Option<usize> {
        match self.notes.binary_search_by_key(&properties.start_beat,
            |note| note.start_beat) {
            Ok(index) => Some(index),
            Err(index) => {
                // Since there wasn't an exact beat, we have to look for one that's inside the range
                for note_index in index..self.notes.len() {
                    // Since the notes are sorted, we already know that this note will start
                    //  after what we're looking for
                    let note = &self.notes[note_index];
                    if note.start_beat < properties.cutoff_beat {
                        return Some(note_index);
                    }
                    // There's no point in continuing to search if we're already outside the range
                    if note.start_beat >= properties.cutoff_beat {
                        break;
                    }
                }
                None
            }
        }
    }
}

pub trait Instrument {
    fn sample_note<'a>(&mut self, note: &Note, mixer_samples: MixerSamples<'a>);

    fn reset(&mut self);

    /// This would return false for example, if this were a percussion instrument (with only 1 pitch)
    fn can_use_note_names(&self) -> bool;
    // TODO Have methods for the UI to call to get info about the kind of instrument
}

pub struct Timing {
    pub bpm: f32,
    /// 4/4 is your normal bar timing (4 beats).
    /// This should only modify the UI (it can't change the rhythm).
    pub time_signature: TimeSignature,
}
impl Timing {
    pub const FOUR_FOUR: TimeSignature = TimeSignature::new_raw(4, 4);

    pub fn new(bpm: f32, time_signature: TimeSignature) -> Timing {
        Timing { bpm, time_signature }
    }
}

/// Each note will go through 3 phases, in order: attack, sustain, decay.
#[derive(Clone, Debug)]
pub struct Note {
    pub name: NoteName,
    pub start_beat: Beat,
    /// Specify how long this note is held in beats (ie. 1 is a quarter note)
    pub beat_length: Beat,
}
impl Note {
    pub fn new(name: NoteName, start_beat: Beat, beat_length: Beat) -> Note {
        Note { name, start_beat, beat_length }
    }
}
impl Note {
    fn note_collision_msg(&self) -> String { format!("Note collision with {:?}", self) }
}
impl Eq for Note {}
impl PartialEq for Note {
    fn eq(&self, other: &Self) -> bool { self.start_beat.eq(&other.start_beat) }
}
impl Ord for Note {
    fn cmp(&self, other: &Self) -> Ordering { self.start_beat.cmp(&other.start_beat) }
}
impl PartialOrd for Note {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

// TODO We will need to have different note types. Single, chord, rest

/// The parameter is the octave on which this note is placed.
#[derive(Copy, Clone, Debug)]
pub enum NoteName {
    C(i8),
    DFlat(i8),
    D(i8),
    EFlat(i8),
    E(i8),
    F(i8),
    GFlat(i8),
    G(i8),
    AFlat(i8),
    A(i8),
    BFlat(i8),
    B(i8),
}
impl NoteName {
    /// The sound frequency required to play this note
    pub fn freq(self) -> f32 {
        // From https://en.wikipedia.org/wiki/Twelfth_root_of_two
        440.0 * 2_f32.powf(self.semitones_from_middle_a() as f32 / 12.0)
    }
}
impl NoteName {
    fn semitones_from_middle_a(self) -> i16 {
        // Middle A is A4
        let (semitones_from_a, octave) = match self {
            Self::C(octave) => (-9_i16, octave),
            Self::DFlat(octave) => (-8_i16, octave),
            Self::D(octave) => (-7_i16, octave),
            Self::EFlat(octave) => (-6_i16, octave),
            Self::E(octave) => (-5_i16, octave),
            Self::F(octave) => (-4_i16, octave),
            Self::GFlat(octave) => (-3_i16, octave),
            Self::G(octave) => (-2_i16, octave),
            Self::AFlat(octave) => (-1_i16, octave),
            Self::A(octave) => (0_i16, octave),
            Self::BFlat(octave) => (1_i16, octave),
            Self::B(octave) => (2_i16, octave),
        };
        // Find the A in the given octave first (and how many semitones that A is from middle A)
        (octave as i16 - 4) * 12 + semitones_from_a
    }
}
