pub struct Lfo {
  sample_rate: f32,
  phase: f32,
}

impl Lfo {
  pub fn new(sample_rate: f32) -> Self {
    Self { sample_rate, phase: 0.0 }
  }

  pub fn process(&mut self, frequency: f32) -> f32 {
    let out = (self.phase * std::f32::consts::TAU).sin(); // TODO: Replace this custom Sine oscillator with an approximation function
    self.phase += frequency / self.sample_rate;
    if self.phase >= 1.0 {
      self.phase -= 1.0;
    }
    out
  }
}
