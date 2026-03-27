# Rosy vs DACELIB Performance Comparison

> **Rosy** (Rust, `--optimized`) vs **DACELIB** 2.1.0 (C, Clang 21.1.8 `-O3`)
>
> Host: `nuclearbombconsole` — 12th Gen Intel Core i7-1260P
>
> **Ratio > 1.0× = Rosy faster · Ratio < 1.0× = DACELIB faster**

---

## Run 1 — Baseline (v0.8.4, best-of-50, March 25 2026)

First head-to-head comparison. Rosy's DA engine uses flat-array storage with
pool allocation and an N×N multiplication index table. No Horner-specific
optimizations yet.

### T3 (order 5, small monomial counts)

| Benchmark         | Rosy Opt (ms) | DACELIB (ms) | Opt/DACE |
|-------------------|---------------|--------------|----------|
| DA Multiply       |          39.2 |         88.1 | **2.25×** |
| DA Trig           |         331.8 |        116.3 |   0.35×  |
| DA Derivatives    |         235.1 |        169.2 |   0.72×  |
| DA Transfer Map   |          59.4 |        113.3 | **1.91×** |
| DA High-Order Mul |          12.2 |         11.2 |   0.92×  |
| DA Bending Magnet |         130.5 |         55.7 |   0.43×  |
| DA Aberration     |          29.9 |         40.6 | **1.36×** |
| **TOTAL**         |     **838.1** |    **594.4** | **0.71×** |

- Rosy wins 3, DACELIB wins 4
- **Rosy 29% slower overall.** Strong on pure multiply; weak on transcendentals
  and derivatives.

---

## Run 2 — Issues #18-#21 (v0.16.0, best-of-10, March 27 2026)

Three optimizations landed:

1. **Progressive Horner truncation (#18)** — At each Horner step, multiply is
   truncated to the order that can actually contribute to the final result.
   Mathematically valid because step `i` from the end can only produce terms up
   to order `(n-1-i)`. Biggest impact on DA Trig and DA Bending Magnet.

2. **Precomputed derivative/integral index tables (#19, #21)** — `deriv_target`,
   `deriv_exponent`, and `integ_target` arrays are built once at `init_taylor()`
   time. Differentiation becomes a single linear scan with zero HashMap lookups,
   zero Monomial construction, zero allocation.

3. **Inline order-check truncated multiply (#20)** — `multiply_truncated()` skips
   pairs where `order_a + order_b > trunc_order` with a single `u8` comparison
   per pair. No auxiliary data structures.

### T3

| Benchmark         | Rosy Opt (ms) | DACELIB (ms) | Opt/DACE | Δ vs Run 1 |
|-------------------|---------------|--------------|----------|------------|
| DA Multiply       |          39.2 |         90.8 | **2.32×** |    +3%    |
| DA Trig           |         279.1 |        119.0 |   0.43×  |  **+23%** |
| DA Derivatives    |          88.6 |        170.2 | **1.92×** | **+167%** |
| DA Transfer Map   |          61.7 |        117.7 | **1.91×** |    same   |
| DA High-Order Mul |          13.1 |         13.4 |   1.02×  |   +11%    |
| DA Bending Magnet |          88.0 |         62.8 |   0.71×  |  **+65%** |
| DA Aberration     |          25.4 |         44.1 | **1.74×** |  **+28%** |
| **TOTAL**         |     **595.1** |    **618.0** | **1.04×** | **+46%** |

### T4

| Benchmark         | Rosy Opt (ms) | DACELIB (ms) | Opt/DACE | Δ vs Run 1 |
|-------------------|---------------|--------------|----------|------------|
| DA Multiply       |         361.0 |        935.1 | **2.59×** |    +9%    |
| DA Trig           |        2864.0 |       1231.8 |   0.43×  |  **+19%** |
| DA Derivatives    |         949.5 |       1854.7 | **1.95×** | **+179%** |
| DA Transfer Map   |         249.9 |        467.7 | **1.87×** |    -4%    |
| DA High-Order Mul |          20.6 |         53.5 | **2.60×** |    -1%    |
| DA Bending Magnet |         467.0 |        283.8 |   0.61×  |  **+53%** |
| DA Aberration     |         103.0 |        202.0 | **1.96×** |  **+43%** |
| **TOTAL**         |    **5015.0** |   **5028.6** | **1.00×** | **+52%** |

- Rosy wins 5 (T4), DACELIB wins 2
- **DA Derivatives flipped from DACE-faster (0.72×) to Rosy 2× faster (1.92×)**
- Overall: Rosy now at parity with DACE

---

## Run 3 — Allocation elimination (v0.16.0, best-of-10, March 27 2026)

Two more optimizations targeting per-multiply overhead:

4. **Thread-local bitset pool** — The `written` bitset (used to track which output
   monomials were touched during multiply) is now pooled like coefficient arrays.
   Eliminates one `Vec<u64>` allocation per multiply call.

5. **Lock-once Horner loops** — `horner_eval` and `horner_eval_fixed` now acquire
   the `TaylorRuntime` read-lock once and pass it to all inner
   `multiply_truncated_with_rt()` calls. Eliminates N-1 RwLock read-acquisitions
   per Horner evaluation (where N = number of Taylor coefficients).

### T3

| Benchmark         | Rosy Opt (ms) | DACELIB (ms) | Opt/DACE | Δ vs Run 2 |
|-------------------|---------------|--------------|----------|------------|
| DA Multiply       |          36.5 |         90.0 | **2.47×** |    +6%    |
| DA Trig           |         265.2 |        120.6 |   0.45×  |    +5%    |
| DA Derivatives    |          90.2 |        174.2 | **1.93×** |    +1%    |
| DA Transfer Map   |          61.2 |        116.2 | **1.90×** |    same   |
| DA High-Order Mul |          13.4 |         11.8 |   0.88×  |   -14%    |
| DA Bending Magnet |          86.9 |         57.6 |   0.66×  |    -7%    |
| DA Aberration     |          23.3 |         43.2 | **1.85×** |    +6%    |
| **TOTAL**         |     **576.7** |    **613.6** | **1.06×** |  **+2%** |

### T4

| Benchmark         | Rosy Opt (ms) | DACELIB (ms) | Opt/DACE | Δ vs Run 2 |
|-------------------|---------------|--------------|----------|------------|
| DA Multiply       |         346.6 |        921.7 | **2.66×** |    +3%    |
| DA Trig           |        2757.2 |       1221.1 |   0.44×  |    +2%    |
| DA Derivatives    |         888.2 |       1747.5 | **1.97×** |    +1%    |
| DA Transfer Map   |         233.6 |        452.6 | **1.94×** |    +4%    |
| DA High-Order Mul |          20.2 |         46.3 | **2.29×** |   -12%    |
| DA Bending Magnet |         378.0 |        250.9 |   0.66×  |    +8%    |
| DA Aberration     |          87.3 |        179.6 | **2.06×** |    +5%    |
| **TOTAL**         |    **4711.1** |   **4819.7** | **1.02×** |  **+2%** |

- Rosy wins 5 (T4), DACELIB wins 2
- **Rosy now 6% faster on aggregate (T3) and 2% faster (T4)**
- Remaining gaps: DA Trig (0.45×) and DA Bending Magnet (0.66×) — both
  transcendental-heavy Horner paths

---

## Remaining Gap Analysis

The two benchmarks where DACE still wins are both Horner-evaluation-heavy:

| Factor | DACE approach | Rosy approach | Impact |
|--------|--------------|---------------|--------|
| **Multiply inner loop** | Sort B by order into buckets; bounded inner loop skips entire buckets | Flat iteration over all B nonzeros; per-pair `u8` comparison to skip | DACE avoids touching pairs entirely; Rosy still branches per pair |
| **Addressing** | `ia1[e1+e1'] + ia2[e2+e2']` — 2 additions + 2 lookups | `mult_table[i*N+j]` — 1 lookup but iterates all pairs | DACE's approach naturally prunes; Rosy's table is faster per-pair but does more pairs |
| **Result packing** | Dense `cc[]` → sparse pack in one pass; clears `cc[i]=0` while packing | Dense coeffs + bitset scan → build nonzero list | Similar cost |

Closing these gaps requires DACE-style order bucketing in the Horner multiply
path, or a fundamentally different sparse multiply strategy.

---

## Failed Attempt: Order Bucketing (reverted, March 27 2026)

Implemented DACE-style order bucketing for Horner multiply:

- **`BucketedNonzero` struct** — counting-sort of RHS nonzero indices by order,
  with `up_to_order(max_o)` returning a bounded slice instead of per-pair branching
- **`order_boundary` table** — precomputed cumulative monomial counts by order in
  `TaylorRuntime`, used by `FixedMultiplier` to limit output loop without branching
- **`multiply_bucketed_rt()`** — new multiply path using bucketed RHS slices

**Result:** DA Bending Magnet improved slightly (0.66× → 0.83× T3), but other
benchmarks regressed or were flat. Net effect was ~zero or slightly negative.
The overhead of bucket-sorting the RHS nonzeros on every Horner multiply (even
though it's O(K)) outweighs the savings from eliminating per-pair branches on
small DA vectors (21-126 monomials). Order bucketing wins for DACE because DACE
also uses it for dense-workspace addressing (`ia1`/`ia2`), which Rosy doesn't
have.

**Lesson:** At our monomial counts, branch misprediction cost is already low
(predictable patterns in tight loops). The bottleneck is elsewhere.