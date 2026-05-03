use nih_plug::prelude::*;
use thorgend_dsp::MAX_NUM_CPS;
mod custom_formatters;
use crate::thorgend_params::custom_formatters::{s2v_f32_ms_then_s, v2s_f32_ms_then_s};
use nih_plug::formatters::{s2v_f32_gain_to_db, v2s_f32_gain_to_db, v2s_f32_rounded};

#[derive(Params)]
pub struct ThorgendParams {
  /// Probability distribution for amplitude random walk (0=linear, 1=Cauchy, 2=logistic, 3=hyperbcos, 4=arcsine, 5=expon, 6=sinus)
  #[id = "ampdist"]
  pub amp_dist: IntParam,

  /// Probability distribution for duration random walk
  #[id = "durdist"]
  pub dur_dist: IntParam,

  /// Shape parameter for amplitude distribution [0.0001, 1.0]
  #[id = "aparam"]
  pub a_amp: FloatParam,

  /// Shape parameter for duration distribution [0.0001, 1.0]
  #[id = "dparam"]
  pub a_dur: FloatParam,

  /// Scale factor for amplitude step size
  #[id = "ampscale"]
  pub scale_amp: FloatParam,

  /// Scale factor for duration step size
  #[id = "durscale"]
  pub scale_dur: FloatParam,

  /// Number of active control points (breakpoints per cycle)
  #[id = "numcps"]
  pub num_cps: IntParam,

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
      amp_dist: IntParam::new("Amp Distribution", 1, IntRange::Linear { min: 0, max: 6 }),

      dur_dist: IntParam::new("Dur Distribution", 1, IntRange::Linear { min: 0, max: 6 }),

      a_amp: FloatParam::new(
        "Amp Param",
        1.0,
        FloatRange::Linear {
          min: 0.0001,
          max: 1.0,
        },
      ),

      a_dur: FloatParam::new(
        "Dur Param",
        1.0,
        FloatRange::Linear {
          min: 0.0001,
          max: 1.0,
        },
      ),

      scale_amp: FloatParam::new("Amp Scale", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 }),

      scale_dur: FloatParam::new("Dur Scale", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 }),

      num_cps: IntParam::new(
        "Num Control Points",
        MAX_NUM_CPS as i32,
        IntRange::Linear {
          min: 1,
          max: MAX_NUM_CPS as i32,
        },
      ),

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
