#[derive(Clone, Copy)]
pub struct SampleAndHold {
    sample_rate: f32,
    sh_phase: f32,
    sh_held: f32,
}

impl SampleAndHold {
    pub fn new(sample_rate: f32) -> Self {
        Self { sample_rate, sh_phase: 0.0, sh_held: 0.0 }
    }

    /// Advances the S&H clock. When it ticks, snapshots `input` through `tanh(drive * input)`.
    /// `sh_frequency` — clock rate in Hz (2–200)
    /// `drive`        — linear amplitude multiplier applied before tanh clipping (1.0 = 0 dB)
    pub fn process(&mut self, input: f32, sh_frequency: f32, drive: f32) -> f32 {
        self.sh_phase += sh_frequency / self.sample_rate;
        if self.sh_phase >= 1.0 {
            self.sh_phase -= 1.0;
            self.sh_held = (input * drive).tanh();
        }
        self.sh_held
    }
}
