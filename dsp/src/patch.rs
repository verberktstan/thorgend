pub enum Curve {
  Linear,
  Exponential { exponent: f32 },
}

pub struct Patch {
  input_min: f32,
  input_max: f32,
  curve: Curve,
}

impl Patch {
  pub fn new(input_min: f32, input_max: f32) -> Self {
    Self { input_min, input_max, curve: Curve::Linear }
  }

  pub fn bipolar() -> Self {
    Self::new(-1.0, 1.0)
  }

  pub fn unipolar() -> Self {
    Self::new(0.0, 1.0)
  }

  pub fn with_curve(mut self, curve: Curve) -> Self {
    self.curve = curve;
    self
  }

  fn to_unit(&self, input: f32) -> f32 {
    let t = ((input - self.input_min) / (self.input_max - self.input_min)).clamp(0.0, 1.0);
    match &self.curve {
      Curve::Linear => t,
      Curve::Exponential { exponent } => t.powf(*exponent),
    }
  }

  pub fn map_f32(&self, input: f32, min: f32, max: f32) -> f32 {
    min + self.to_unit(input) * (max - min)
  }

  pub fn map_usize(&self, input: f32, min: usize, max: usize) -> usize {
    (min as f32 + self.to_unit(input) * (max - min) as f32).round() as usize // TODO: Omit .round and just cast to usize for performance reasons?
  }
}
