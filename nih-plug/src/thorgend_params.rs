use nih_plug::prelude::*;
mod custom_formatters;
use crate::thorgend_params::custom_formatters::{s2v_f32_ms_then_s, v2s_f32_ms_then_s};
use nih_plug::formatters::{s2v_f32_gain_to_db, v2s_f32_gain_to_db, v2s_f32_rounded};

#[derive(Params)]
pub struct ThorgendParams {
  #[id = "voices"]
  pub voices: IntParam,

  #[id = "num_cps"]
  pub num_cps: IntParam,

  #[id = "attack"]
  pub attack: FloatParam,

  #[id = "decay"]
  pub decay: FloatParam,

  #[id = "sustain"]
  pub sustain: FloatParam,

  #[id = "release"]
  pub release: FloatParam,

  #[id = "output_gain"]
  pub output_gain: FloatParam,

  #[id = "noisyness"]
  pub noisyness: FloatParam,

  #[id = "noiseindex"]
  pub noiseindex: FloatParam,


  #[id = "lfo_drive"]
  pub dichotomization: FloatParam,

  #[id = "variousity_rate"]
  pub variousity_rate: FloatParam,
}

pub fn freq_skew_factor() -> f32 {
  FloatRange::skew_factor(-2.0)
}

impl Default for ThorgendParams {
  fn default() -> Self {
    Self {
      num_cps: IntParam::new("Richness", 7, IntRange::Linear { min: 2, max: 18 }),


      noiseindex: FloatParam::new("Noise Complexity", 0.0, FloatRange::Linear { min: 0.0, max: 1.0 }),

      noisyness: FloatParam::new("Noisyness", 0.0, FloatRange::Linear { min: -1.0, max: 1.0 }),

      voices: IntParam::new("Voices", 1, IntRange::Linear { min: 1, max: 16 }),

      attack: FloatParam::new(
        "Attack",
        1.,
        FloatRange::Skewed {
          min: 0.1,
          max: 5000.,
          factor: 0.2,
        },
      )
      .with_value_to_string(v2s_f32_ms_then_s())
      .with_string_to_value(s2v_f32_ms_then_s()),

      decay: FloatParam::new(
        "Decay",
        5.,
        FloatRange::Skewed {
          min: 1.,
          max: 15000.,
          factor: 0.2,
        },
      )
      .with_value_to_string(v2s_f32_ms_then_s())
      .with_string_to_value(s2v_f32_ms_then_s()),

      sustain: FloatParam::new(
        "Sustain",
        util::db_to_gain(-6.0),
        FloatRange::Skewed {
          min: util::db_to_gain(-60.0),
          max: util::db_to_gain(0.0),
          factor: FloatRange::gain_skew_factor(-60.0, 0.0),
        },
      )
      .with_smoother(SmoothingStyle::Logarithmic(50.0))
      .with_unit(" dB")
      .with_value_to_string(v2s_f32_gain_to_db(2))
      .with_string_to_value(s2v_f32_gain_to_db()),

      release: FloatParam::new(
        "Release",
        5.,
        FloatRange::Skewed {
          min: 1.,
          max: 15000.,
          factor: 0.3,
        },
      )
      .with_value_to_string(v2s_f32_ms_then_s())
      .with_string_to_value(s2v_f32_ms_then_s()),

      output_gain: FloatParam::new(
        "Output Gain",
        util::db_to_gain(-6.0),
        FloatRange::Skewed {
          min: util::db_to_gain(-60.0),
          max: util::db_to_gain(0.0),
          factor: FloatRange::gain_skew_factor(-60.0, 0.0),
        },
      )
      .with_smoother(SmoothingStyle::Logarithmic(50.0))
      .with_unit(" dB")
      .with_value_to_string(v2s_f32_gain_to_db(2))
      .with_string_to_value(s2v_f32_gain_to_db()),


      variousity_rate: FloatParam::new(
        "Variousity rate",
        0.0,
        FloatRange::Linear { min: 0.0, max: 1.0 },
      ),

      dichotomization: FloatParam::new(
        "Dichotomization",
        0.0,
        FloatRange::Linear { min: 0.0, max: 24.0 },
      )
      .with_unit(" dB")
      .with_value_to_string(formatters::v2s_f32_rounded(1)),
    }
  }
}
