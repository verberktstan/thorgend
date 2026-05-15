mod gendy1;
mod linear_adsr;
mod notes;
mod lfo;
mod pitch_drift_compensation;
pub use lfo::Lfo;
pub use shared::float_ext::FloatExt;
mod shared {
  pub mod float_ext;
}
pub use crate::notes::Notes;
use crate::{
  gendy1::Gendy1,
  linear_adsr::ADSR,
  notes::{ADSRStage, Note},
};

const MAX_VOICE_COUNT: usize = 8;
const ADSR_RETRIGGER_TIME_IN_MS: f32 = 2.;

// Hardcoded distributions passed to Gendy1::process. If you change DUR_DIST or A_DUR
// you must re-run the dur_bar_measurement test and update the compensation coefficients
// in pitch_drift_compensation.rs.
pub const AMP_DIST: i32 = 2; // LOGISTIC
pub const A_AMP: f32 = 1.0;
pub const DUR_DIST: i32 = 3; // HYPERBCOS
pub const A_DUR: f32 = 1.0;

pub struct Voices {
  oscillator: Vec<Gendy1>,
  adsr: Vec<ADSR>,
}

impl Voices {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      oscillator: vec![Gendy1::new(sample_rate); MAX_VOICE_COUNT],
      adsr: vec![ADSR::new(sample_rate, ADSR_RETRIGGER_TIME_IN_MS); MAX_VOICE_COUNT],
    }
  }

  pub fn reset(&mut self) {
    for osc in self.oscillator.iter_mut() {
      osc.reset();
    }
  }

  pub fn process(
    &mut self,
    amp_dist: i32,
    dur_dist: i32,
    a_amp: f32,
    a_dur: f32,
    scale_amp: f32,
    scale_dur: f32,
    num_cps: usize,
    max_freq_factor: f32,
    attack: f32,
    decay: f32,
    sustain: f32,
    release: f32,
    notes: &mut Vec<Note>,
  ) -> f32 {
    let mut sum = 0.;

    for ((note, oscillator), adsr) in notes
      .iter_mut()
      .zip(self.oscillator.iter_mut())
      .zip(self.adsr.iter_mut())
    {
      if *note.get_adsr_stage() == ADSRStage::Idle {
        continue;
      }
      let envelope = adsr.process(note, attack, decay, sustain, release);
      let freq = adsr.get_freq();
      let min_f = pitch_drift_compensation::compensated_min_freq(freq, scale_dur, max_freq_factor, dur_dist, a_dur);
      let output = oscillator.process(
        amp_dist,
        dur_dist,
        a_amp,
        a_dur,
        min_f,
        min_f * max_freq_factor,
        scale_amp,
        scale_dur,
        num_cps,
      ) * envelope;

      sum += output;
    }

    sum
  }
}
