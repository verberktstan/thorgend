mod gendy1;
mod linear_adsr;
mod notes;
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
const A4: f32 = 440.;
const ADSR_ATTACK: f32 = 10.;
const ADSR_DECAY: f32 = 200.;
const ADSR_SUSTAIN: f32 = 0.5;
const ADSR_RELEASE: f32 = 1000.;
const ADSR_RETRIGGER_TIME: f32 = 0.;

pub struct Voices {
  oscillator: Vec<Gendy1>,
  adsr: Vec<ADSR>,
}

impl Voices {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      oscillator: vec![Gendy1::new(sample_rate); MAX_VOICE_COUNT],
      adsr: vec![ADSR::new(sample_rate, ADSR_RETRIGGER_TIME); MAX_VOICE_COUNT],
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
    min_freq: f32,
    max_freq: f32,
    scale_amp: f32,
    scale_dur: f32,
    num_cps: usize,
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

      let freq = Self::midi_to_hz(note.get_note());
      let envelope = adsr.process(note, ADSR_ATTACK, ADSR_DECAY, ADSR_SUSTAIN, ADSR_RELEASE);
      let output = oscillator.process(
        amp_dist,
        dur_dist,
        a_amp,
        a_dur,
        freq,
        freq * 2.,
        scale_amp,
        scale_dur,
        num_cps,
      ) * envelope;

      sum += output;
    }

    sum
  }

  fn midi_to_hz(note: u8) -> f32 {
    A4 * 2_f32.powf((note as f32 - 69.) / 12.)
  }
}
