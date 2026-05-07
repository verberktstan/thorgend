pub const MAX_NUM_CPS: usize = 12;

// Probability distributions ported from Gendyn_distribution() in GendynUGens.cpp
fn gendyn_dist(which: i32, a: f32, f: f32) -> f32 {
  let a = a.clamp(0.0001, 1.0);
  let result = match which {
    1 => {
      // CAUCHY
      let c = (10.0 * a).atan();
      (c * (2.0 * f - 1.0)).tan() / a * 0.1
    }
    2 => {
      // LOGISTIC
      let cv = 0.5 + 0.499 * a;
      let c = ((1.0 - cv) / cv).ln();
      if c.abs() < 1e-10 {
        return 0.0;
      }
      let f2 = (f - 0.5) * 0.998 * a + 0.5;
      ((1.0 - f2) / f2).ln() / c
    }
    3 => {
      // HYPERBCOS
      let c = (1.5692255_f32 * a).tan();
      if c.abs() < 1e-10 {
        return 0.0;
      }
      let t = (1.5692255_f32 * a * f).tan() / c;
      let arg = t * 0.999 + 0.001;
      if arg <= 0.0 {
        return -1.0;
      }
      2.0 * arg.ln() * (-0.1447648_f32) - 1.0
    }
    4 => {
      // ARCSINE
      let c = (std::f32::consts::FRAC_PI_2 * a).sin();
      if c.abs() < 1e-10 {
        return 0.0;
      }
      (std::f32::consts::PI * (f - 0.5) * a).sin() / c
    }
    5 => {
      // EXPON
      let c = (1.0 - 0.999 * a).ln();
      let t = (1.0 - f * 0.999 * a).ln() / c;
      2.0 * t - 1.0
    }
    6 => {
      // SINUS: use `a` as a constant (maps [0,1] -> [-1,1])
      2.0 * a - 1.0
    }
    _ => {
      // LINEAR (0) and default
      2.0 * f - 1.0
    }
  };
  if result.is_finite() {
    result
  } else {
    0.0
  }
}

// Mirror amp value back into [-1, 1] via folding
fn mirror_amp(mut v: f32) -> f32 {
  if v > 1.0 || v < -1.0 {
    if v < 0.0 {
      v += 4.0;
    }
    v = v.rem_euclid(4.0);
    if v > 1.0 && v < 3.0 {
      v = 2.0 - v;
    } else if v > 1.0 {
      v -= 4.0;
    }
  }
  v
}

// Mirror dur value back into [0, 1] via folding
fn mirror_dur(mut v: f32) -> f32 {
  if v > 1.0 || v < 0.0 {
    if v < 0.0 {
      v += 2.0;
    }
    v = v.rem_euclid(2.0);
    v = 2.0 - v;
  }
  v
}

trait Gendy {
  fn new(sample_rate: f32) -> Self;
  fn reset(&mut self);
  fn process(
    &mut self,
    gain: f32,
    amp_dist: i32,
    dur_dist: i32,
    a_amp: f32,
    a_dur: f32,
    min_freq: f32,
    max_freq: f32,
    scale_amp: f32,
    scale_dur: f32,
    num_cps: usize,
  ) -> f32;
}

#[derive(Clone, Copy)]
pub struct Gendy1 {
  sample_rate: f32,
  phase: f64,
  amp: f32,
  next_amp: f32,
  speed: f64,
  dur: f32,
  index: usize,
  memory_amp: [f32; MAX_NUM_CPS],
  memory_dur: [f32; MAX_NUM_CPS],
}

impl Gendy1 {
  pub fn new(sample_rate: f32) -> Self {
    let mut memory_amp = [0.0f32; MAX_NUM_CPS];
    let mut memory_dur = [0.0f32; MAX_NUM_CPS];
    for i in 0..MAX_NUM_CPS {
      memory_amp[i] = 2.0 * fastrand::f32() - 1.0;
      memory_dur[i] = fastrand::f32();
    }

    Self {
      sample_rate,
      phase: 1.0,
      amp: 0.0,
      next_amp: 0.0,
      speed: 0.0,
      dur: 0.5,
      index: 0,
      memory_amp,
      memory_dur,
    }
  }

  pub fn reset(&mut self) {
    for i in 0..MAX_NUM_CPS {
      self.memory_amp[i] = 2.0 * fastrand::f32() - 1.0;
      self.memory_dur[i] = fastrand::f32();
    }
    self.phase = 1.0;
    self.amp = 0.0;
    self.next_amp = 0.0;
    self.speed = 0.0;
    self.dur = 0.5;
    self.index = 0;
  }

  pub fn process(
    &mut self,
    amp_dist: i32,
    dur_dist: i32,
    a_amp: f32,
    a_dur: f32,
    min_freq: f32,
    max_freq: f32,
    scale_amp: f32,
    scale_dur: f32,
    num_cps: usize, // expected range: 1..=MAX_NUM_CPS; clamped to guard against out-of-range CV inputs
  ) -> f32 {
    let num = num_cps.clamp(1, MAX_NUM_CPS);

    if self.phase >= 1.0 {
      self.phase -= 1.0;
      self.index = (self.index + 1) % num;
      self.amp = self.next_amp;

      let new_next =
        self.memory_amp[self.index] + scale_amp * gendyn_dist(amp_dist, a_amp, fastrand::f32());
      self.next_amp = mirror_amp(new_next);
      self.memory_amp[self.index] = self.next_amp;

      let new_dur =
        self.memory_dur[self.index] + scale_dur * gendyn_dist(dur_dist, a_dur, fastrand::f32());
      self.dur = mirror_dur(new_dur);
      self.memory_dur[self.index] = self.dur;

      self.speed = ((min_freq + (max_freq - min_freq) * self.dur) as f64 / self.sample_rate as f64)
        * num as f64;
    }

    let z = ((1.0 - self.phase) * self.amp as f64 + self.phase * self.next_amp as f64) as f32;
    self.phase += self.speed;

    z
  }
}
