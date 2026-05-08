pub trait FloatExt {
  fn dbtoa(self) -> Self;
  fn map_range(self, from_lo: f32, from_hi: f32, to_lo: f32, to_hi: f32) -> Self;
  fn mix(self, right: f32, factor: f32) -> Self;
  fn mstosamps(self, sample_rate: Self) -> Self;
}

impl FloatExt for f32 {
  /// Converts decibels to a linear amplitude value
  fn dbtoa(self) -> Self {
    (10_f32).powf(self * 0.05)
  }

  fn map_range(self, from_lo: f32, from_hi: f32, to_lo: f32, to_hi: f32) -> Self {
    to_lo + (self - from_lo) / (from_hi - from_lo) * (to_hi - to_lo)
  }

  fn mix(self, right: f32, factor: f32) -> Self {
    self + (right - self) * factor
  }

  /// Convert milliseconds to samples based on the samplerate.
  fn mstosamps(self, sample_rate: Self) -> Self {
    self * 0.001 * sample_rate
  }
}
