use std::f32::consts::PI;

use crate::sampling::Sample;
use super::WaveFunction;

pub struct SinWave {
    phase: f32,
}
impl SinWave {
    pub fn new() -> SinWave {
        SinWave {
            phase: 0.0,
        }
    }
}
impl WaveFunction for SinWave {
    fn reset(&mut self) {
        self.phase = 0.0;
    }

    fn sample_at(&mut self, delta_seconds: f32, freq: f32) -> f32 {
        self.phase += delta_seconds * freq;
        if self.phase > 1.0 {
            self.phase -= 1.0;
        }
        (self.phase * 2.0 * PI).sin()
    }
}

pub struct SquareWave {
    phase: f32,
}
impl SquareWave {
    pub fn new() -> SquareWave {
        SquareWave {
            phase: 0.0,
        }
    }
}
impl WaveFunction for SquareWave {
    fn reset(&mut self) {
        self.phase = 0.0;
    }

    fn sample_at(&mut self, delta_seconds: f32, freq: f32) -> f32 {
        self.phase += delta_seconds * freq;
        if self.phase > 1.0 {
            self.phase -= 1.0;
        }
        if self.phase < 0.5 {
            1.0
        } else {
            -1.0
        }
    }
}

pub struct TriangleWave {
    phase: f32,
}
impl TriangleWave {
    pub fn new() -> TriangleWave {
        TriangleWave {
            phase: 0.0,
        }
    }
}
impl WaveFunction for TriangleWave {
    fn reset(&mut self) {
        self.phase = 0.0;
    }
    fn sample_at(&mut self, delta_seconds: f32, freq: f32) -> f32 {
        self.phase += delta_seconds * freq;
        if self.phase > 1.0 {
            self.phase -= 1.0;
        }
        // We want a linear wave that can go from peak to trough in half the time, then back up
        if self.phase < 0.5 {
            1.0 - self.phase * 4.0
        } else {
            -1.0 + (self.phase - 0.5) * 4.0
        }
    }

}
