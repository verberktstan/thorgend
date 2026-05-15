pub struct Lfo {
  sample_rate: f32,
  phase: f32,
  sh_phase: f32,
  sh_held: f32,
}

impl Lfo {
  pub fn new(sample_rate: f32) -> Self {
    Self { sample_rate, phase: 0.0, sh_phase: 0.0, sh_held: 0.0 }
  }

  /// `frequency`   — LFO oscillation rate in Hz
  /// `sh_frequency` — sample-and-hold clock rate in Hz (2–200); sin() is only evaluated at this rate
  /// `drive`       — linear amplitude multiplier (1.0 = 0 dB) applied before tanh clipping
  /// `freq_mod`    — additive frequency modulation input in Hz (pass 0.0 when unused)
  pub fn process(&mut self, frequency: f32, sh_frequency: f32, drive: f32, freq_mod: f32) -> f32 {
    let effective_freq = (frequency + freq_mod).max(0.0);

    self.phase += effective_freq / self.sample_rate;
    if self.phase >= 1.0 {
      self.phase -= 1.0;
    }

    self.sh_phase += sh_frequency / self.sample_rate;
    if self.sh_phase >= 1.0 {
      self.sh_phase -= 1.0;
      let sine = (self.phase * std::f32::consts::TAU).sin();
      self.sh_held = (sine * drive).tanh();
    }

    self.sh_held
  }
}
