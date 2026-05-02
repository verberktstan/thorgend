pub trait FloatExt {
  fn dbtoa(self) -> Self;
  fn mix(self, right: f32, factor: f32) -> Self;
  fn mstosamps(self, sample_rate: Self) -> Self;
}

impl FloatExt for f32 {
  /// Converts decibels to a linear amplitude value
  fn dbtoa(self) -> Self {
    (10_f32).powf(self * 0.05)
  }

  fn mix(self, right: f32, factor: f32) -> Self {
    self + (right - self) * factor
  }

  /// Convert milliseconds to samples based on the samplerate.
  fn mstosamps(self, sample_rate: Self) -> Self {
    self * 0.001 * sample_rate
  }
}
