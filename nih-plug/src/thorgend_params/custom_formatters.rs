use nih_plug::prelude::AtomicF32;
use std::sync::{atomic::Ordering, Arc};

pub fn v2s_f32_ms_then_s() -> Arc<dyn Fn(f32) -> String + Send + Sync> {
  Arc::new(move |value| {
    if value >= 10000. {
      format!("{:.1} s", value / 1000.0)
    } else if value >= 1000. {
      format!("{:.2} s", value / 1000.0)
    } else if value >= 100. {
      format!("{value:.0} ms")
    } else if value >= 10. {
      format!("{value:.1} ms")
    } else {
      format!("{value:.2} ms")
    }
  })
}

pub fn s2v_f32_ms_then_s() -> Arc<dyn Fn(&str) -> Option<f32> + Send + Sync> {
  Arc::new(move |string| {
    let time_segment = string.trim().to_ascii_lowercase();

    if let Some(val) = time_segment.strip_suffix("ms") {
      val.trim().parse::<f32>().ok()
    } else if let Some(val) = time_segment.strip_suffix('s') {
      val.trim().parse::<f32>().ok().map(|x| x * 1000.0)
    } else {
      time_segment.parse::<f32>().ok()
    }
  })
}
