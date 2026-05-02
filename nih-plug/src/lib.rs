use nih_plug::prelude::*;
use std::sync::Arc;
use thorgend_dsp::{Gendy1, MAX_NUM_CPS};

#[derive(Params)]
pub struct Gendy1Params {
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

  /// Minimum output frequency in Hz
  #[id = "minfreq"]
  pub min_freq: FloatParam,

  /// Maximum output frequency in Hz
  #[id = "maxfreq"]
  pub max_freq: FloatParam,

  /// Scale factor for amplitude step size
  #[id = "ampscale"]
  pub scale_amp: FloatParam,

  /// Scale factor for duration step size
  #[id = "durscale"]
  pub scale_dur: FloatParam,

  /// Number of active control points (breakpoints per cycle)
  #[id = "numcps"]
  pub num_cps: IntParam,

  #[id = "gain"]
  pub gain: FloatParam,
}

impl Default for Gendy1Params {
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
      min_freq: FloatParam::new(
        "Min Freq",
        440.0,
        FloatRange::Skewed {
          min: 20.0,
          max: 20000.0,
          factor: FloatRange::skew_factor(-2.0),
        },
      )
      .with_unit(" Hz")
      .with_value_to_string(formatters::v2s_f32_hz_then_khz(2))
      .with_string_to_value(formatters::s2v_f32_hz_then_khz()),
      max_freq: FloatParam::new(
        "Max Freq",
        660.0,
        FloatRange::Skewed {
          min: 20.0,
          max: 20000.0,
          factor: FloatRange::skew_factor(-2.0),
        },
      )
      .with_unit(" Hz")
      .with_value_to_string(formatters::v2s_f32_hz_then_khz(2))
      .with_string_to_value(formatters::s2v_f32_hz_then_khz()),
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
      gain: FloatParam::new(
        "Gain",
        util::db_to_gain(-6.0),
        FloatRange::Skewed {
          min: util::db_to_gain(-60.0),
          max: util::db_to_gain(0.0),
          factor: FloatRange::gain_skew_factor(-60.0, 0.0),
        },
      )
      .with_smoother(SmoothingStyle::Logarithmic(50.0))
      .with_unit(" dB")
      .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
      .with_string_to_value(formatters::s2v_f32_gain_to_db()),
    }
  }
}

pub struct Thorgend {
  params: Arc<Gendy1Params>,
  sample_rate: f32,
  gendy1: Gendy1,
}

impl Default for Thorgend {
  fn default() -> Self {
    Self {
      params: Arc::new(Gendy1Params::default()),
      sample_rate: 44100.0,
      gendy1: Gendy1::new(44100.0),
    }
  }
}

impl Plugin for Thorgend {
  const NAME: &'static str = "Thorgend";
  const VENDOR: &'static str = "Definitieve Standaard";
  const URL: &'static str = "";
  const EMAIL: &'static str = "";
  const VERSION: &'static str = env!("CARGO_PKG_VERSION");

  const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
    main_input_channels: None,
    main_output_channels: NonZeroU32::new(2),
    ..AudioIOLayout::const_default()
  }];

  const MIDI_INPUT: MidiConfig = MidiConfig::None;
  const SAMPLE_ACCURATE_AUTOMATION: bool = true;

  type BackgroundTask = ();
  type SysExMessage = ();

  fn params(&self) -> Arc<dyn Params> {
    self.params.clone()
  }

  fn initialize(
    &mut self,
    _audio_io_layout: &AudioIOLayout,
    buffer_config: &BufferConfig,
    _context: &mut impl InitContext<Self>,
  ) -> bool {
    self.sample_rate = buffer_config.sample_rate;
    true
  }

  fn reset(&mut self) {
    self.gendy1.reset();
  }

  fn process(
    &mut self,
    buffer: &mut Buffer,
    _aux: &mut AuxiliaryBuffers,
    _context: &mut impl ProcessContext<Self>,
  ) -> ProcessStatus {
    for channel_samples in buffer.iter_samples() {
      let gain = self.params.gain.smoothed.next();
      let amp_dist = self.params.amp_dist.value();
      let dur_dist = self.params.dur_dist.value();
      let a_amp = self.params.a_amp.value();
      let a_dur = self.params.a_dur.value();
      let min_freq = self.params.min_freq.value();
      let max_freq = self.params.max_freq.value();
      let scale_amp = self.params.scale_amp.value();
      let scale_dur = self.params.scale_dur.value();
      let num_cps = self.params.num_cps.value() as usize;

      let gendy1_out = self.gendy1.process(
        gain, amp_dist, dur_dist, a_amp, a_dur, min_freq, max_freq, scale_amp, scale_dur, num_cps,
      );

      for sample in channel_samples {
        *sample = gendy1_out;
      }
    }

    ProcessStatus::Normal
  }
}

impl ClapPlugin for Thorgend {
  const CLAP_ID: &'static str = "dm-Thorgend";
  const CLAP_DESCRIPTION: Option<&'static str> = Some("GENDYN stochastic synthesis");
  const CLAP_MANUAL_URL: Option<&'static str> = None;
  const CLAP_SUPPORT_URL: Option<&'static str> = None;
  const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::Instrument, ClapFeature::Stereo];
}

impl Vst3Plugin for Thorgend {
  const VST3_CLASS_ID: [u8; 16] = *b"dm-Thorgend.....";
  const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Fx];
}

nih_export_clap!(Thorgend);
nih_export_vst3!(Thorgend);
