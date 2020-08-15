mod instruments;
mod sampling;
mod song;

use num_rational::Ratio;

use self::{
    instruments::*,
    song::{Musician, Song, Note, NoteName, Timing},
};

type TimeSignature = Ratio<u8>;
type Beat = Ratio<u16>;

const FIRST_BEAT: Beat = Beat::new_raw(0, 1);

fn main() -> Result<(), String> {
    let timing = Timing::new(100.0, Timing::FOUR_FOUR);
    let mut song = Song::new(timing);

    let mut sin_musician = Musician::new(SinWave::new());
    sin_musician.add_note(Note::new(
        NoteName::C(4), FIRST_BEAT, Beat::new(1, 1)
    ))?;
    sin_musician.add_note(Note::new(
        NoteName::C(4),
        Beat::new(3, 1),
        Beat::new(1, 1)
    ))?;
    song.add_musician(sin_musician);

    let mut square_musician = Musician::new(SquareWave::new());
    square_musician.add_note(Note::new(
        NoteName::E(4),
        Beat::new(1, 1),
        Beat::new(1, 1)
    ))?;
    square_musician.add_note(Note::new(
        NoteName::E(4),
        Beat::new(3, 1),
        Beat::new(1, 1)
    ))?;
    song.add_musician(square_musician);

    let mut triangle_musician = Musician::new(TriangleWave::new());
    triangle_musician.add_note(Note::new(
        NoteName::G(4),
        Beat::new(2, 1),
        Beat::new(1, 1)
    ))?;
    triangle_musician.add_note(Note::new(
        NoteName::G(4),
        Beat::new(3, 1),
        Beat::new(1, 1)
    ))?;
    song.add_musician(triangle_musician);

    song.export_to_wav("test.wav")?;
    Ok(())
}

pub fn beat_in_seconds(beat: &Beat, bpm: f32) -> f32 {
    let beat_as_float = *beat.numer() as f32 / *beat.denom() as f32;
    // We need to know how of these beats can fit into a single second
    beat_as_float * (bpm / 60.0)
}
