use nih_plug::prelude::*;
mod custom_formatters;
use crate::thorgend_params::custom_formatters::{s2v_f32_ms_then_s, v2s_f32_ms_then_s};
use nih_plug::formatters::{s2v_f32_gain_to_db, v2s_f32_gain_to_db, v2s_f32_rounded};

#[derive(Params)]
pub struct ThorgendParams {
  /// Scale factor for amplitude step size
  #[id = "ampscale"]
  pub scale_amp: FloatParam,

  /// Scale factor for duration step size
  #[id = "durscale"]
  pub scale_dur: FloatParam,

  #[id = "lfo_rate"]
  pub lfo_rate: FloatParam,

  #[id = "lfo_mul"]
  pub lfo_mul: FloatParam,

  #[id = "lfo_add"]
  pub lfo_add: FloatParam,

  #[id = "noisespeed"]
  pub noisespeed: FloatParam,

  #[id = "noiseindex"]
  pub noiseindex: FloatParam,

  #[id = "noisyness"]
  pub noisyness: FloatParam,

  #[id = "voices"]
  pub voices: IntParam,

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
}

impl Default for ThorgendParams {
  fn default() -> Self {
    Self {
      scale_amp: FloatParam::new("Amp Scale", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 }),

      scale_dur: FloatParam::new("Dur Scale", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 }),

      lfo_rate: FloatParam::new(
        "LFO Rate",
        0.5,
        FloatRange::Skewed {
          min: 0.01,
          max: 20.0,
          factor: FloatRange::skew_factor(-2.0),
        },
      )
      .with_unit(" Hz")
      .with_value_to_string(formatters::v2s_f32_rounded(2)),

      lfo_mul: FloatParam::new(
        "LFO Mul",
        0.5,
        FloatRange::Skewed {
          min: 0.0,
          max: 1.0,
          factor: 0.5,
        },
      ),

      lfo_add: FloatParam::new("LFO Add", 0.0, FloatRange::Linear { min: -1.0, max: 1.0 }),

      noisespeed: FloatParam::new(
        "Noise Speed",
        0.5,
        FloatRange::Skewed {
          min: 0.01,
          max: 20.0,
          factor: FloatRange::skew_factor(-2.0),
        },
      )
      .with_unit(" Hz")
      .with_value_to_string(formatters::v2s_f32_rounded(2)),

      noiseindex: FloatParam::new("Noise Index", 0.0, FloatRange::Linear { min: 0.0, max: 1.0 }),

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
    }
  }
}
