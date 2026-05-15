pub struct Lfo {
  sample_rate: f32,
  phase: f32,
}

impl Lfo {
  pub fn new(sample_rate: f32) -> Self {
    Self { sample_rate, phase: 0.0 }
  }

  /// Steps the phase accumulator. Call once per sample before any `SampleAndHold` reads.
  /// `frequency` — LFO oscillation rate in Hz
  /// `freq_mod`  — additive frequency modulation in Hz (pass 0.0 when unused)
  pub fn advance(&mut self, frequency: f32, freq_mod: f32) {
    let effective_freq = (frequency + freq_mod).max(0.0);
    self.phase += effective_freq / self.sample_rate;
    if self.phase >= 1.0 {
      self.phase -= 1.0;
    }
  }

  /// Returns sin() at the current phase. Called by `SampleAndHold` only on its clock tick.
  pub fn sample(&self) -> f32 {
    (self.phase * std::f32::consts::TAU).sin()
  }
}
