// Compensates for the pitch shift caused by the HYPERBCOS dur distribution (a=1.0) being
// positively biased. Monte Carlo simulation shows dur_bar(s) = 1.0 - 0.385*s for s > 0,
// and 0.5 at s=0. Adjusting min_freq so E[segment_freq] = note_hz keeps pitch stable as
// scale_dur (and therefore noisyness) changes.
//
// Returns note_hz unchanged for any other distribution/shape combination — compensation
// coefficients are only known for HYPERBCOS a=1.0.
pub fn compensated_min_freq(note_hz: f32, scale_dur: f32, dur_dist: i32, a_dur: f32) -> f32 {
    if dur_dist != 3 || a_dur != 1.0 {
        return note_hz;
    }
    // divisor = 1 + 7 * dur_bar(scale_dur)
    let divisor = if scale_dur < 1e-4 {
        4.5 // 1 + 7 * 0.5
    } else {
        8.0 - 2.695 * scale_dur // 1 + 7 * (1.0 - 0.385 * scale_dur)
    };
    note_hz / divisor
}
