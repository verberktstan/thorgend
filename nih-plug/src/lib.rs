use nih_plug::prelude::*;
use std::sync::Arc;

const MEMORY_SIZE: usize = 12;

struct Rng(u32);

impl Rng {
    fn new() -> Self {
        Self(2463534242)
    }

    fn next_u32(&mut self) -> u32 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.0 = x;
        x
    }

    fn frand(&mut self) -> f32 {
        (self.next_u32() >> 8) as f32 * (1.0 / 16777216.0)
    }
}

// Probability distributions ported from Gendyn_distribution() in GendynUGens.cpp
fn gendyn_dist(which: i32, a: f32, f: f32) -> f32 {
    let a = a.clamp(0.0001, 1.0);
    let result = match which {
        1 => {
            // CAUCHY
            let c = (10.0 * a).atan();
            (c * (2.0 * f - 1.0)).tan() / a * 0.1
        }
        2 => {
            // LOGISTIC
            let cv = 0.5 + 0.499 * a;
            let c = ((1.0 - cv) / cv).ln();
            if c.abs() < 1e-10 {
                return 0.0;
            }
            let f2 = (f - 0.5) * 0.998 * a + 0.5;
            ((1.0 - f2) / f2).ln() / c
        }
        3 => {
            // HYPERBCOS
            let c = (1.5692255_f32 * a).tan();
            if c.abs() < 1e-10 {
                return 0.0;
            }
            let t = (1.5692255_f32 * a * f).tan() / c;
            let arg = t * 0.999 + 0.001;
            if arg <= 0.0 {
                return -1.0;
            }
            2.0 * arg.ln() * (-0.1447648_f32) - 1.0
        }
        4 => {
            // ARCSINE
            let c = (std::f32::consts::FRAC_PI_2 * a).sin();
            if c.abs() < 1e-10 {
                return 0.0;
            }
            (std::f32::consts::PI * (f - 0.5) * a).sin() / c
        }
        5 => {
            // EXPON
            let c = (1.0 - 0.999 * a).ln();
            let t = (1.0 - f * 0.999 * a).ln() / c;
            2.0 * t - 1.0
        }
        6 => {
            // SINUS: use `a` as a constant (maps [0,1] -> [-1,1])
            2.0 * a - 1.0
        }
        _ => {
            // LINEAR (0) and default
            2.0 * f - 1.0
        }
    };
    if result.is_finite() { result } else { 0.0 }
}

// Mirror amp value back into [-1, 1] via folding
fn mirror_amp(mut v: f32) -> f32 {
    if v > 1.0 || v < -1.0 {
        if v < 0.0 {
            v += 4.0;
        }
        v = v.rem_euclid(4.0);
        if v > 1.0 && v < 3.0 {
            v = 2.0 - v;
        } else if v > 1.0 {
            v -= 4.0;
        }
    }
    v
}

// Mirror dur value back into [0, 1] via folding
fn mirror_dur(mut v: f32) -> f32 {
    if v > 1.0 || v < 0.0 {
        if v < 0.0 {
            v += 2.0;
        }
        v = v.rem_euclid(2.0);
        v = 2.0 - v;
    }
    v
}

pub struct Gendy1 {
    params: Arc<Gendy1Params>,
    sample_rate: f32,
    phase: f64,
    amp: f32,
    next_amp: f32,
    speed: f64,
    dur: f32,
    index: usize,
    memory_amp: [f32; MEMORY_SIZE],
    memory_dur: [f32; MEMORY_SIZE],
    rng: Rng,
}

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

impl Default for Gendy1 {
    fn default() -> Self {
        let mut rng = Rng::new();
        let mut memory_amp = [0.0f32; MEMORY_SIZE];
        let mut memory_dur = [0.0f32; MEMORY_SIZE];
        for i in 0..MEMORY_SIZE {
            memory_amp[i] = 2.0 * rng.frand() - 1.0;
            memory_dur[i] = rng.frand();
        }
        Self {
            params: Arc::new(Gendy1Params::default()),
            sample_rate: 44100.0,
            phase: 1.0,
            amp: 0.0,
            next_amp: 0.0,
            speed: 0.0,
            dur: 0.5,
            index: 0,
            memory_amp,
            memory_dur,
            rng,
        }
    }
}

impl Default for Gendy1Params {
    fn default() -> Self {
        Self {
            amp_dist: IntParam::new("Amp Distribution", 1, IntRange::Linear { min: 0, max: 6 }),
            dur_dist: IntParam::new("Dur Distribution", 1, IntRange::Linear { min: 0, max: 6 }),
            a_amp: FloatParam::new(
                "Amp Param",
                1.0,
                FloatRange::Linear { min: 0.0001, max: 1.0 },
            ),
            a_dur: FloatParam::new(
                "Dur Param",
                1.0,
                FloatRange::Linear { min: 0.0001, max: 1.0 },
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
            scale_amp: FloatParam::new(
                "Amp Scale",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            scale_dur: FloatParam::new(
                "Dur Scale",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            num_cps: IntParam::new(
                "Num Control Points",
                MEMORY_SIZE as i32,
                IntRange::Linear { min: 1, max: MEMORY_SIZE as i32 },
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

impl Plugin for Gendy1 {
    const NAME: &'static str = "Gendy1";
    const VENDOR: &'static str = "DM";
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
        self.rng = Rng::new();
        for i in 0..MEMORY_SIZE {
            self.memory_amp[i] = 2.0 * self.rng.frand() - 1.0;
            self.memory_dur[i] = self.rng.frand();
        }
        self.phase = 1.0;
        self.amp = 0.0;
        self.next_amp = 0.0;
        self.speed = 0.0;
        self.dur = 0.5;
        self.index = 0;
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
            let num = {
                let n = self.params.num_cps.value() as usize;
                if n >= 1 && n <= MEMORY_SIZE { n } else { MEMORY_SIZE }
            };

            if self.phase >= 1.0 {
                self.phase -= 1.0;
                self.index = (self.index + 1) % num;
                self.amp = self.next_amp;

                let new_next = self.memory_amp[self.index]
                    + scale_amp * gendyn_dist(amp_dist, a_amp, self.rng.frand());
                self.next_amp = mirror_amp(new_next);
                self.memory_amp[self.index] = self.next_amp;

                let new_dur = self.memory_dur[self.index]
                    + scale_dur * gendyn_dist(dur_dist, a_dur, self.rng.frand());
                self.dur = mirror_dur(new_dur);
                self.memory_dur[self.index] = self.dur;

                self.speed = ((min_freq + (max_freq - min_freq) * self.dur) as f64
                    / self.sample_rate as f64)
                    * num as f64;
            }

            let z = ((1.0 - self.phase) * self.amp as f64
                + self.phase * self.next_amp as f64) as f32
                * gain;
            self.phase += self.speed;

            for sample in channel_samples {
                *sample = z;
            }
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for Gendy1 {
    const CLAP_ID: &'static str = "dm-Gendy1";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("GENDYN stochastic synthesis");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::Instrument, ClapFeature::Stereo];
}

impl Vst3Plugin for Gendy1 {
    const VST3_CLASS_ID: [u8; 16] = *b"dm-Gendy1.......";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Fx];
}

nih_export_clap!(Gendy1);
nih_export_vst3!(Gendy1);
