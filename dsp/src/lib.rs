mod gendy1;
mod linear_adsr;
mod notes;
mod lfo;
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
const LOWEST_MIDI_HZ: f32 = 8.18;
const HIGHEST_MIDI_HZ: f32 = 13289.75;

// Hardcoded distributions passed to Gendy1::process. Owned here so that
// dur_compensated_min_freq (below) and the call site stay in sync — if you
// change DUR_DIST or A_DUR you must re-run the dur_bar_measurement test and
// update the compensation coefficients.
pub const AMP_DIST: i32 = 2; // LOGISTIC
pub const A_AMP: f32 = 1.0;
pub const DUR_DIST: i32 = 3; // HYPERBCOS — tightly coupled to dur_compensated_min_freq
pub const A_DUR: f32 = 1.0;  // tightly coupled to dur_compensated_min_freq

// Compensates for the pitch shift caused by the HYPERBCOS dur distribution (a=1.0) being
// positively biased. Monte Carlo simulation shows dur_bar(s) = 1.0 - 0.385*s for s > 0,
// and 0.5 at s=0. Adjusting min_freq so E[segment_freq] = note_hz keeps pitch stable as
// scale_dur (and therefore noisyness) changes.
// IMPORTANT: coefficients are specific to DUR_DIST=3 (HYPERBCOS) with A_DUR=1.0.
fn dur_compensated_min_freq(note_hz: f32, scale_dur: f32) -> f32 {
    // divisor = 1 + 7 * dur_bar(scale_dur)
    let divisor = if scale_dur < 1e-4 {
        4.5 // 1 + 7 * 0.5
    } else {
        8.0 - 2.695 * scale_dur // 1 + 7 * (1.0 - 0.385 * scale_dur)
    };
    note_hz / divisor
}

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
      let min_f = dur_compensated_min_freq(freq, scale_dur).clamp(LOWEST_MIDI_HZ, HIGHEST_MIDI_HZ);
      let output = oscillator.process(
        amp_dist,
        dur_dist,
        a_amp,
        a_dur,
        min_f,
        (min_f * 8.).clamp(LOWEST_MIDI_HZ, HIGHEST_MIDI_HZ),
        scale_amp,
        scale_dur,
        num_cps,
      ) * envelope;

      sum += output;
    }

    sum
  }
}
