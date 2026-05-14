# Variance correction for dur-based pitch compensation

## Why a residual drift remains

`dur_compensated_min_freq` (`dsp/src/lib.rs`) pins the **arithmetic mean** of segment
frequencies to `note_hz`:

```
μ_f = min_freq_adj + (max_freq_adj - min_freq_adj) × E[dur]  =  note_hz
```

But the perceived fundamental is the **harmonic mean** of segment frequencies, not the
arithmetic mean. Because `1/x` is convex, Jensen's inequality gives:

```
E[1/f_i]  >  1 / E[f_i]
```

so the harmonic mean is always *below* the arithmetic mean. The oscillator therefore runs
slightly flat relative to `note_hz`, and the size of the error grows with `Var[dur]` — which
itself changes with `scale_dur` (and therefore with `noisyness`). That is the residual pitch
drift.

---

## Mathematical derivation

Let:

```
f_i  =  min_freq_adj + g × dur_i          (g = max_freq_adj - min_freq_adj = 7 × min_freq_adj)
μ_f  =  E[f_i]   =  min_freq_adj + g × dur_bar(s)
σ²_f =  Var[f_i] =  g² × var_dur(s)
```

Second-order Taylor expansion of `E[1/f_i]` around `μ_f`:

```
E[1/f_i]  ≈  1/μ_f  +  σ²_f / μ_f³
           =  (1/μ_f) × (1 + σ²_f/μ_f²)
```

Harmonic mean:

```
f_harm  ≈  1 / E[1/f_i]  ≈  μ_f / (1 + σ²_f/μ_f²)
```

For `f_harm = note_hz` we need the corrected arithmetic mean target:

```
μ_f_target  =  note_hz × (1 + σ²_f / μ_f²)
```

Substituting `g = 7 × min_freq_adj` and `μ_f ≈ note_hz`:

```
μ_f_target  =  note_hz × (1 + C(s) × var_dur(s))
```

where the scale factor is:

```
C(s)  =  (7 / (1 + 7 × dur_bar(s)))²
```

`C(s)` is already known from the first-order compensation; it needs no new measurement.

The corrected `min_freq_adj` becomes:

```
min_freq_adj  =  note_hz × k(s) / (1 + 7 × dur_bar(s))

k(s)  =  1 + C(s) × var_dur(s)
       =  1 + (7 / (1 + 7 × dur_bar(s)))² × var_dur(s)
```

When `var_dur(s) = 0` this reduces to the current formula. The correction is purely
additive in `k`, so it is safe to implement incrementally.

---

## What to measure

`var_dur(s)` is not yet known. Extend the `dur_bar_measurement` test in
`dsp/src/gendy1.rs` to accumulate `sum_sq` alongside `sum`:

```rust
let mut sum    = 0.0_f64;
let mut sum_sq = 0.0_f64;

for j in 0..(WARMUP + SAMPLES) {
    dur = mirror_dur(dur + scale_dur * gendyn_dist(DIST, A, fastrand::f32()));
    if j >= WARMUP {
        sum    += dur as f64;
        sum_sq += (dur * dur) as f64;
    }
}

let dur_bar = sum    / SAMPLES as f64;
let var_dur = sum_sq / SAMPLES as f64 - dur_bar * dur_bar;
```

Print both columns for `s ∈ [0.0, 1.0]` in steps of 0.05 (same grid as the original run).

---

## Expected shape of var_dur(s)

Before measuring, physics gives us two boundary constraints:

- **s = 0**: no perturbation, `dur` stays fixed → `var_dur = 0`  
- **s → 0⁺**: `dur` pins against the upper boundary → `var_dur → 0`  
- **s = 1**: large steps spread the distribution → `var_dur` is at its maximum

The variance is therefore expected to peak somewhere in the mid-to-high `s` range and be
near zero at both extremes. A low-degree polynomial (degree 2 or 3) should fit well.

---

## Implementation

Once `var_dur(s)` is measured and fit:

1. **Add `var_dur(s)`** as an inline function next to `dur_compensated_min_freq` in
   `dsp/src/lib.rs`, with the fit coefficients and a reference to the measurement test.

2. **Update `dur_compensated_min_freq`** — replace the single-line `note_hz / divisor`
   return with:

   ```rust
   let dur_bar  = 1.0 - 0.385 * scale_dur;   // existing
   let divisor  = 1.0 + 7.0 * dur_bar;       // existing
   let c        = (7.0 / divisor).powi(2);   // new
   let k        = 1.0 + c * var_dur(scale_dur); // new
   note_hz * k / divisor
   ```

3. **No other files change.** The correction is entirely local to `dur_compensated_min_freq`.

---

## Estimated magnitude

`C(s)` at the operating extremes:

| s    | dur_bar | C(s) = (7/(1+7×d))² |
|------|---------|----------------------|
| 0.05 | 0.981   | (7/7.87)² ≈ 0.79     |
| 0.50 | 0.806   | (7/6.64)² ≈ 1.11     |
| 1.00 | 0.615   | (7/5.31)² ≈ 1.74     |

If `var_dur` peaks around 0.04–0.08 (a plausible range for a reflected biased walk), the
correction `k - 1 = C × var_dur` reaches at most **0.04–0.14**, i.e. 4–14% in frequency
(roughly 0.7–2.3 semitones at the high end). That is audible and worth correcting.

At `s = 0.05` (near-zero noise), `var_dur ≈ 0`, so `k ≈ 1` and the correction vanishes
naturally — no special-casing needed at the boundary.
