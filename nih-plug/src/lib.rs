mod thorgend_params;
use nih_plug::prelude::*;
use std::sync::Arc;
use thorgend_dsp::{Lfo, Notes, Voices, AMP_DIST, A_AMP, DUR_DIST, A_DUR};
use thorgend_params::ThorgendParams;

pub struct Thorgend {
  params: Arc<ThorgendParams>,
  sample_rate: f32,
  voices: Voices,
  notes: Notes,
  lfo2: Lfo,
}

impl Default for Thorgend {
  fn default() -> Self {
    Self {
      params: Arc::new(ThorgendParams::default()),
      sample_rate: 44100.0,
      voices: Voices::new(44100.0),
      notes: Notes::new(),
      lfo2: Lfo::new(44100.0),
    }
  }
}

impl Thorgend {
  fn process_midi_events(&mut self, context: &mut impl ProcessContext<Self>) {
    // while is needed because events come in batches
    while let Some(event) = context.next_event() {
      match event {
        NoteEvent::NoteOn { note, velocity, .. } => {
          self.notes.note_on(note, velocity);
        }
        NoteEvent::NoteOff { note, .. } => {
          self.notes.note_off(note);
        }
        NoteEvent::MidiCC { cc, value, .. } => match cc {
          64 => self.notes.sustain(value > 0.),
          120 => self.notes.remove_notes(),
          123 => self.notes.release_notes(),
          _ => (),
        },
        NoteEvent::MidiPitchBend { value, .. } => {
          todo!("Implement pitch bend");
        }
        _ => (),
      }
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

  const MIDI_INPUT: MidiConfig = MidiConfig::Basic;
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
    self.voices = Voices::new(buffer_config.sample_rate);
    self.lfo2 = Lfo::new(buffer_config.sample_rate);

    true
  }

  fn reset(&mut self) {
    self.voices.reset();
  }

  fn process(
    &mut self,
    buffer: &mut Buffer,
    _aux: &mut AuxiliaryBuffers,
    context: &mut impl ProcessContext<Self>,
  ) -> ProcessStatus {
    let num_cps = self.params.num_cps.value() as usize;
    let noisespeed = self.params.noisespeed.value();
    let noiseindex = util::db_to_gain(self.params.noiseindex.value() * 60.0 - 60.0);
    let noisyness = self.params.noisyness.value();
    let attack = self.params.attack.value();
    let decay = self.params.decay.value();
    let release = self.params.release.value();
    let lfo_sh_freq = self.params.lfo_sh_freq.value();
    let lfo_drive = util::db_to_gain(self.params.lfo_drive.value());
    self
      .notes
      .set_voice_count(self.params.voices.value() as usize);

    self.process_midi_events(context);

    for channel_samples in buffer.iter_samples() {
      let sustain = self.params.sustain.smoothed.next();
      let output_gain = self.params.output_gain.smoothed.next();

      let lfo2_out = self.lfo2.process(noisespeed, lfo_sh_freq, lfo_drive, 0.0) * noiseindex + noisyness;
      let effective_scale_amp = (0.5_f32 + lfo2_out).clamp(0.0, 1.0);
      let effective_scale_dur = (0.5_f32 + lfo2_out).clamp(0.0, 1.0);
      let voices_out = self.voices.process(
        AMP_DIST,
        DUR_DIST,
        A_AMP,
        A_DUR,
        effective_scale_amp,
        effective_scale_dur,
        num_cps,
        attack,
        decay,
        sustain,
        release,
        self.notes.get_notes(),
      ) * output_gain;

      for sample in channel_samples {
        *sample = voices_out;
      }
    }

    ProcessStatus::Normal
  }
}

impl ClapPlugin for Thorgend {
  const CLAP_ID: &'static str = "Thorgend";
  const CLAP_DESCRIPTION: Option<&'static str> = Some("GENDYN stochastic synthesis");
  const CLAP_MANUAL_URL: Option<&'static str> = None;
  const CLAP_SUPPORT_URL: Option<&'static str> = None;
  const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::Instrument];
}

impl Vst3Plugin for Thorgend {
  const VST3_CLASS_ID: [u8; 16] = *b"Thorgend........";
  const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Instrument];
}

nih_export_clap!(Thorgend);
nih_export_vst3!(Thorgend);
