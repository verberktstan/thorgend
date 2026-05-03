use std::array;

const A4: f32 = 440.;

#[derive(PartialEq, Clone)]
pub enum ADSRStage {
  Attack,
  Decay,
  Sustain,
  Release,
  Retrigger,
  Idle,
}

#[derive(Clone)]
pub struct Note {
  note: u8,
  gain: f32,
  adsr_stage: ADSRStage,
  midi_note_to_hz: [f32; 128],
}

impl Note {
  pub fn default() -> Self {
    Self {
      note: 0,
      gain: 0.,
      adsr_stage: ADSRStage::Idle,
      midi_note_to_hz: array::from_fn(|note| A4 * 2_f32.powf((note as f32 - 69.) / 12.)),
    }
  }

  pub fn note_on(&mut self, note: u8, velocity: f32) {
    self.note = note;
    self.gain = velocity.sqrt();
    self.adsr_stage = ADSRStage::Attack;
  }

  pub fn note_off(&mut self) {
    self.adsr_stage = ADSRStage::Release;
  }

  pub fn steal_note(&mut self, note: u8, velocity: f32) {
    self.note = note;
    self.gain = velocity;
    self.adsr_stage = match self.adsr_stage {
      ADSRStage::Idle => ADSRStage::Attack,
      _ => ADSRStage::Retrigger,
    };
  }

  pub fn reset_note(&mut self) {
    self.note = 0;
    self.gain = 0.;
    self.adsr_stage = ADSRStage::Idle;
  }

  pub fn set_adsr_stage(&mut self, adsr_stage: ADSRStage) {
    self.adsr_stage = adsr_stage;
  }

  pub fn get_note(&self) -> u8 {
    self.note
  }

  pub fn get_freq(&self) -> f32 {
    self.midi_note_to_hz[self.get_note() as usize]
  }

  pub fn get_gain(&self) -> f32 {
    self.gain
  }

  pub fn get_adsr_stage(&self) -> &ADSRStage {
    &self.adsr_stage
  }
}
