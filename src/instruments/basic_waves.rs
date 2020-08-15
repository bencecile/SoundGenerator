use std::f32::consts::PI;

use super::WaveFunction;

pub struct SinWave {
    phases: [f32; 5],
}
impl SinWave {
    pub fn new() -> SinWave {
        SinWave {
            phases: [0.0; 5],
        }
    }
}
impl WaveFunction for SinWave {
    fn reset(&mut self) {
        self.phases = [0.0; 5];
    }

    fn sample_at(&mut self, note_channel: usize, delta_seconds: f32, freq: f32) -> f32 {
        let phase = &mut self.phases[note_channel];
        *phase += delta_seconds * freq;
        if *phase > 1.0 {
            *phase -= 1.0;
        }
        (*phase * 2.0 * PI).sin()
    }
}

pub struct SquareWave {
    phases: [f32; 5],
}
impl SquareWave {
    pub fn new() -> SquareWave {
        SquareWave {
            phases: [0.0; 5],
        }
    }
}
impl WaveFunction for SquareWave {
    fn reset(&mut self) {
        self.phases = [0.0; 5];
    }

    fn sample_at(&mut self, note_channel: usize, delta_seconds: f32, freq: f32) -> f32 {
        let phase = &mut self.phases[note_channel];
        *phase += delta_seconds * freq;
        if *phase > 1.0 {
            *phase -= 1.0;
        }
        if *phase < 0.5 {
            1.0
        } else {
            -1.0
        }
    }
}

pub struct TriangleWave {
    phases: [f32; 5],
}
impl TriangleWave {
    pub fn new() -> TriangleWave {
        TriangleWave {
            phases: [0.0; 5],
        }
    }
}
impl WaveFunction for TriangleWave {
    fn reset(&mut self) {
        self.phases = [0.0; 5];
    }
    fn sample_at(&mut self, note_channel: usize, delta_seconds: f32, freq: f32) -> f32 {
        let phase = &mut self.phases[note_channel];
        *phase += delta_seconds * freq;
        if *phase > 1.0 {
            *phase -= 1.0;
        }
        // We want a linear wave that can go from peak to trough in half the time, then back up
        if *phase < 0.5 {
            1.0 - *phase * 4.0
        } else {
            -1.0 + (*phase - 0.5) * 4.0
        }
    }

}
