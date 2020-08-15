mod instruments;
mod sampling;
mod song;

use num_rational::Ratio;

use self::{
    instruments::*,
    song::{Musician, Song, Note, NoteName, NoteType, Timing},
};

type TimeSignature = Ratio<u8>;
type Beat = Ratio<u16>;

const FIRST_BEAT: Beat = Beat::new_raw(0, 1);

fn main() -> Result<(), String> {
    let timing = Timing::new(120.0, Timing::FOUR_FOUR);
    let mut song = Song::new(timing);

    let mut sin_musician = Musician::new(SinWave::new());
    sin_musician.add_note(Note {
        note_type: NoteType::Single(NoteName::A(4)),
        start_beat: FIRST_BEAT,
        beat_length: Beat::new(1, 1),
    })?;
    sin_musician.add_note(Note {
        note_type: NoteType::Single(NoteName::G(4)),
        start_beat: Beat::new(1, 1),
        beat_length: Beat::new(1, 1),
    })?;
    sin_musician.add_note(Note {
        note_type: NoteType::Single(NoteName::F(4)),
        start_beat: Beat::new(2, 1),
        beat_length: Beat::new(1, 1),
    })?;
    sin_musician.add_note(Note {
        note_type: NoteType::Single(NoteName::G(4)),
        start_beat: Beat::new(3, 1),
        beat_length: Beat::new(1, 1),
    })?;
    sin_musician.add_note(Note {
        note_type: NoteType::Single(NoteName::A(4)),
        start_beat: Beat::new(4, 1),
        beat_length: Beat::new(1, 1),
    })?;
    sin_musician.add_note(Note {
        note_type: NoteType::Single(NoteName::A(4)),
        start_beat: Beat::new(5, 1),
        beat_length: Beat::new(1, 1),
    })?;
    sin_musician.add_note(Note {
        note_type: NoteType::Single(NoteName::A(4)),
        start_beat: Beat::new(6, 1),
        beat_length: Beat::new(2, 1),
    })?;
    sin_musician.add_note(Note {
        note_type: NoteType::Single(NoteName::G(4)),
        start_beat: Beat::new(8, 1),
        beat_length: Beat::new(1, 1),
    })?;
    sin_musician.add_note(Note {
        note_type: NoteType::Single(NoteName::G(4)),
        start_beat: Beat::new(9, 1),
        beat_length: Beat::new(1, 1),
    })?;
    sin_musician.add_note(Note {
        note_type: NoteType::Single(NoteName::G(4)),
        start_beat: Beat::new(10, 1),
        beat_length: Beat::new(2, 1),
    })?;
    sin_musician.add_note(Note {
        note_type: NoteType::Single(NoteName::A(4)),
        start_beat: Beat::new(12, 1),
        beat_length: Beat::new(1, 1),
    })?;
    sin_musician.add_note(Note {
        note_type: NoteType::Single(NoteName::C(5)),
        start_beat: Beat::new(13, 1),
        beat_length: Beat::new(1, 1),
    })?;
    sin_musician.add_note(Note {
        note_type: NoteType::Single(NoteName::C(5)),
        start_beat: Beat::new(14, 1),
        beat_length: Beat::new(2, 1),
    })?;
    sin_musician.add_note(Note {
        note_type: NoteType::Single(NoteName::A(4)),
        start_beat: Beat::new(16, 1),
        beat_length: Beat::new(1, 1),
    })?;
    sin_musician.add_note(Note {
        note_type: NoteType::Single(NoteName::G(4)),
        start_beat: Beat::new(17, 1),
        beat_length: Beat::new(1, 1),
    })?;
    sin_musician.add_note(Note {
        note_type: NoteType::Single(NoteName::F(4)),
        start_beat: Beat::new(18, 1),
        beat_length: Beat::new(1, 1),
    })?;
    sin_musician.add_note(Note {
        note_type: NoteType::Single(NoteName::G(4)),
        start_beat: Beat::new(19, 1),
        beat_length: Beat::new(1, 1),
    })?;
    sin_musician.add_note(Note {
        note_type: NoteType::Single(NoteName::A(4)),
        start_beat: Beat::new(20, 1),
        beat_length: Beat::new(1, 1),
    })?;
    sin_musician.add_note(Note {
        note_type: NoteType::Single(NoteName::A(4)),
        start_beat: Beat::new(21, 1),
        beat_length: Beat::new(1, 1),
    })?;
    sin_musician.add_note(Note {
        note_type: NoteType::Single(NoteName::A(4)),
        start_beat: Beat::new(22, 1),
        beat_length: Beat::new(1, 1),
    })?;
    sin_musician.add_note(Note {
        note_type: NoteType::Single(NoteName::A(4)),
        start_beat: Beat::new(23, 1),
        beat_length: Beat::new(1, 1),
    })?;
    sin_musician.add_note(Note {
        note_type: NoteType::Single(NoteName::G(4)),
        start_beat: Beat::new(24, 1),
        beat_length: Beat::new(1, 1),
    })?;
    sin_musician.add_note(Note {
        note_type: NoteType::Single(NoteName::G(4)),
        start_beat: Beat::new(25, 1),
        beat_length: Beat::new(1, 1),
    })?;
    sin_musician.add_note(Note {
        note_type: NoteType::Single(NoteName::A(4)),
        start_beat: Beat::new(26, 1),
        beat_length: Beat::new(1, 1),
    })?;
    sin_musician.add_note(Note {
        note_type: NoteType::Single(NoteName::G(4)),
        start_beat: Beat::new(27, 1),
        beat_length: Beat::new(1, 1),
    })?;
    sin_musician.add_note(Note {
        note_type: NoteType::Single(NoteName::F(4)),
        start_beat: Beat::new(28, 1),
        beat_length: Beat::new(4, 1),
    })?;
    song.add_musician(sin_musician);

    let mut triangle_musician = Musician::new(TriangleWave::new());
    triangle_musician.add_note(Note {
        note_type: NoteType::Single(NoteName::F(3)),
        start_beat: Beat::new(0, 1),
        beat_length: Beat::new(4, 1),
    })?;
    song.add_musician(triangle_musician);
    let mut triangle_musician = Musician::new(TriangleWave::new());
    triangle_musician.add_note(Note {
        note_type: NoteType::Single(NoteName::A(3)),
        start_beat: Beat::new(0, 1),
        beat_length: Beat::new(4, 1),
    })?;
    song.add_musician(triangle_musician);
    let mut triangle_musician = Musician::new(TriangleWave::new());
    triangle_musician.add_note(Note {
        note_type: NoteType::Single(NoteName::C(3)),
        start_beat: Beat::new(0, 1),
        beat_length: Beat::new(4, 1),
    })?;
    song.add_musician(triangle_musician);

    let mut triangle_musician = Musician::new(TriangleWave::new());
    triangle_musician.add_note(Note {
        note_type: NoteType::Chord3(NoteName::F(3), NoteName::A(3), NoteName::C(4)),
        start_beat: Beat::new(0, 1),
        beat_length: Beat::new(4, 1),
    })?;
    triangle_musician.add_note(Note {
        note_type: NoteType::Chord3(NoteName::F(3), NoteName::A(3), NoteName::C(4)),
        start_beat: Beat::new(4, 1),
        beat_length: Beat::new(4, 1),
    })?;
    triangle_musician.add_note(Note {
        note_type: NoteType::Chord3(NoteName::C(3), NoteName::E(3), NoteName::G(4)),
        start_beat: Beat::new(8, 1),
        beat_length: Beat::new(4, 1),
    })?;
    triangle_musician.add_note(Note {
        note_type: NoteType::Chord3(NoteName::F(3), NoteName::A(3), NoteName::C(4)),
        start_beat: Beat::new(12, 1),
        beat_length: Beat::new(4, 1),
    })?;
    triangle_musician.add_note(Note {
        note_type: NoteType::Chord3(NoteName::F(3), NoteName::A(3), NoteName::C(4)),
        start_beat: Beat::new(16, 1),
        beat_length: Beat::new(4, 1),
    })?;
    triangle_musician.add_note(Note {
        note_type: NoteType::Chord3(NoteName::F(3), NoteName::A(3), NoteName::C(4)),
        start_beat: Beat::new(20, 1),
        beat_length: Beat::new(4, 1),
    })?;
    triangle_musician.add_note(Note {
        note_type: NoteType::Chord3(NoteName::C(3), NoteName::E(3), NoteName::G(4)),
        start_beat: Beat::new(24, 1),
        beat_length: Beat::new(4, 1),
    })?;
    triangle_musician.add_note(Note {
        note_type: NoteType::Chord3(NoteName::F(3), NoteName::A(3), NoteName::C(4)),
        start_beat: Beat::new(28, 1),
        beat_length: Beat::new(4, 1),
    })?;
    song.add_musician(triangle_musician);

    song.export_to_wav("test.wav")?;
    Ok(())
}

pub fn beat_in_seconds(beat: &Beat, bpm: f32) -> f32 {
    let beat_as_float = *beat.numer() as f32 / *beat.denom() as f32;
    // We need to know how of these beats can fit into a single second
    beat_as_float / (bpm / 60.0)
}
