mod gendy1;
mod linear_adsr;
mod notes;
mod lfo;
pub use lfo::Lfo;
mod shared {
  pub mod float_ext;
}
pub use crate::notes::Notes;
use crate::{
  gendy1::Gendy1,
  linear_adsr::ADSR,
  notes::{ADSRStage, Note},
};

pub const MAX_NUM_CPS: usize = 12;
const MAX_VOICE_COUNT: usize = 8;
const ADSR_RETRIGGER_TIME_IN_MS: f32 = 2.;

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
      let output = oscillator.process(
        amp_dist,
        dur_dist,
        a_amp,
        a_dur,
        freq,
        freq * 2., // TODO: add something like a bandwidth param
        scale_amp,
        scale_dur,
        num_cps,
      ) * envelope;

      sum += output;
    }

    sum
  }
}
