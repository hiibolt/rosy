# Rosy vs DACELIB Performance Comparison

> **Rosy** v0.16.0 (Rust, `--release` / `--optimized`) vs **DACELIB** 2.1.0 (C, Clang 21.1.8 `-O3`)
>
> Host: `nuclearbombconsole` — 12th Gen Intel Core i7-1260P
>
> Best-of-10 runs · March 27, 2026

**Ratio > 1.0× = Rosy faster · Ratio < 1.0× = DACELIB faster**

---

## T3 Results

| Benchmark         | Rosy Rel (ms) | Rosy Opt (ms) | DACELIB (ms) | Rel/DACE | Opt/DACE |
|-------------------|---------------|---------------|--------------|----------|----------|
| DA Multiply       |          48.0 |          39.2 |         90.8 |   1.89×  | **2.32×** |
| DA Trig           |         292.2 |         279.1 |        119.0 |   0.41×  |   0.43×  |
| DA Derivatives    |          99.9 |          88.6 |        170.2 |   1.70×  | **1.92×** |
| DA Transfer Map   |          66.3 |          61.7 |        117.7 |   1.78×  | **1.91×** |
| DA High-Order Mul |          14.1 |          13.1 |         13.4 |   0.95×  |   1.02×  |
| DA Bending Magnet |          95.0 |          88.0 |         62.8 |   0.66×  |   0.71×  |
| DA Aberration     |          28.3 |          25.4 |         44.1 |   1.56×  | **1.74×** |
| **TOTAL**         |     **643.8** |     **595.1** |    **618.0** | **0.96×** | **1.04×** |

- **Release:** Rosy wins 4, DACELIB wins 2, ties 1
- **Optimized:** Rosy wins 4, DACELIB wins 2, ties 1

---

## T4 Results

| Benchmark         | Rosy Rel (ms) | Rosy Opt (ms) | DACELIB (ms) | Rel/DACE | Opt/DACE |
|-------------------|---------------|---------------|--------------|----------|----------|
| DA Multiply       |         416.6 |         361.0 |        935.1 |   2.24×  | **2.59×** |
| DA Trig           |        2959.7 |        2864.0 |       1231.8 |   0.42×  |   0.43×  |
| DA Derivatives    |        1051.9 |         949.5 |       1854.7 |   1.76×  | **1.95×** |
| DA Transfer Map   |         258.8 |         249.9 |        467.7 |   1.81×  | **1.87×** |
| DA High-Order Mul |          21.6 |          20.6 |         53.5 |   2.48×  | **2.60×** |
| DA Bending Magnet |         497.7 |         467.0 |        283.8 |   0.57×  |   0.61×  |
| DA Aberration     |         117.2 |         103.0 |        202.0 |   1.72×  | **1.96×** |
| **TOTAL**         |    **5323.5** |    **5015.0** |   **5028.6** | **0.94×** | **1.00×** |

- **Release:** Rosy wins 5, DACELIB wins 2, ties 0
- **Optimized:** Rosy wins 5, DACELIB wins 2, ties 0

---

## Key Optimizations (v0.16.0)

1. **Progressive Horner truncation (#18)** — At each Horner step, multiply is truncated
   to the order that can actually contribute to the final result. Biggest impact on
   DA Trig (+23%) and DA Bending Magnet (+65%).

2. **Precomputed derivative/integral index tables (#19, #21)** — Derivative and integral
   target indices are computed once at `init_taylor()` time, eliminating all HashMap
   lookups and Monomial construction during differentiation. DA Derivatives improved
   from 0.72× to **1.92×** (now nearly 2× faster than DACE).

3. **Inline order-check truncated multiply (#20)** — `multiply_truncated()` skips pairs
   where `order_a + order_b > trunc_order` with a single comparison per pair, avoiding
   any auxiliary allocation.

---

## Previous Results (v0.8.4 baseline, best-of-50, March 22 2026)

| Benchmark         | Rosy Opt (ms) | DACELIB (ms) | Opt/DACE |
|-------------------|---------------|--------------|----------|
| DA Multiply       |          39.2 |         88.1 |   2.25×  |
| DA Trig           |         331.8 |        116.3 |   0.35×  |
| DA Derivatives    |         235.1 |        169.2 |   0.72×  |
| DA Transfer Map   |          59.4 |        113.3 |   1.91×  |
| DA High-Order Mul |          12.2 |         11.2 |   0.92×  |
| DA Bending Magnet |         130.5 |         55.7 |   0.43×  |
| DA Aberration     |          29.9 |         40.6 |   1.36×  |
| **TOTAL**         |     **838.1** |    **594.4** | **0.71×** |

- Rosy wins 3, DACELIB wins 4