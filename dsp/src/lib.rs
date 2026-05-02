mod gendy1;
mod notes;
pub use crate::notes::Notes;
use crate::{gendy1::Gendy1, notes::Note};

pub const MAX_NUM_CPS: usize = 12;
const MAX_VOICE_COUNT: usize = 8;
const A4: f32 = 440.;

pub struct Voices {
  oscillator: Vec<Gendy1>,
}

impl Voices {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      oscillator: vec![Gendy1::new(sample_rate); MAX_VOICE_COUNT],
    }
  }

  pub fn reset(&mut self) {
    for osc in self.oscillator.iter_mut() {
      osc.reset();
    }
  }

  pub fn process(
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
    notes: &mut Vec<Note>,
  ) -> f32 {
    let mut sum = 0.;

    for (note, oscillator) in notes.iter_mut().zip(self.oscillator.iter_mut()) {
      let freq = Self::midi_to_hz(note.get_note());

      let output = oscillator.process(
        gain,
        amp_dist,
        dur_dist,
        a_amp,
        a_dur,
        freq,
        freq * 2.,
        scale_amp,
        scale_dur,
        num_cps,
      );

      sum += output;
    }

    sum
  }

  fn midi_to_hz(note: u8) -> f32 {
    A4 * 2_f32.powf((note as f32 - 69.) / 12.)
  }
}
