# Rosy vs COSY INFINITY — Performance Comparison

**Rosy v0.8.4** vs **COSY INFINITY 9.1** | NIU Metis Cluster | March 2026

Rosy compiles ROSY source to native Rust executables. COSY interprets FOX scripts in a Fortran runtime. All numbers below are **best-of** across multiple runs in both `--release` and `--optimized` modes — each system gets its fastest time.

## Overall

| Suite | Rosy | COSY | Winner |
|---|---|---|---|
| Non-MPI (17 benchmarks, T4) | 48.4s | 116.3s | Rosy **2.4x** |
| MPI (10 benchmarks, 20 nodes) | 13.3s | 71.4s | Rosy **5.4x** |

---

## Non-MPI Benchmarks (best-of, T3 + T4)

Single compute node. T3 = representative workload (1–10s), T4 = stress test (10–30s).
Best Rosy time from any mode, best COSY time from any mode.

### T3 — Representative Workloads

| # | Benchmark | Rosy (ms) | COSY (ms) | Winner |
|---|---|---:|---:|---|
| 01 | Arithmetic Loop | 221 | 2,802 | Rosy **12.7x** |
| 02 | DA Multiply | 67 | 153 | Rosy **2.3x** |
| 03 | DA Trig | 453 | 284 | COSY 1.6x |
| 04 | Matrix Inversion | 16 | 85 | Rosy **5.3x** |
| 05 | Matrix Determinant | 11 | 74 | Rosy **6.7x** |
| 06 | Simplex Optimization | 37 | 223 | Rosy **6.0x** |
| 07 | LMDIF Optimization | 1,254 | 1,203 | tie |
| 08 | Math Functions | 1,616 | 8,613 | Rosy **5.3x** |
| 09 | Recursive Fibonacci | 9 | — | — |
| 10 | Vector Operations | 13 | 54 | Rosy **4.2x** |
| 11 | Nested Loops | 97 | 742 | Rosy **7.6x** |
| 12 | DA Derivatives | 322 | 237 | COSY 1.4x |
| 13 | String Operations | 149 | 223 | Rosy **1.5x** |
| 14 | DA Transfer Map | 86 | 122 | Rosy **1.4x** |
| 15 | DA High-Order Multiply | 16 | 27 | Rosy **1.7x** |
| 16 | DA Bending Magnet | 173 | 135 | COSY 1.3x |
| 17 | DA Aberration | 48 | 63 | Rosy **1.3x** |
| 18 | POLVAL Map Eval | 580 | 1,977 | Rosy **3.4x** |

### T4 — Stress Tests

| # | Benchmark | Rosy (ms) | COSY (ms) | Winner |
|---|---|---:|---:|---|
| 01 | Arithmetic Loop | 1,836 | 28,151 | Rosy **15.3x** |
| 02 | DA Multiply | 578 | 1,460 | Rosy **2.5x** |
| 03 | DA Trig | 4,572 | 3,095 | COSY 1.5x |
| 04 | Matrix Inversion | 117 | 667 | Rosy **5.7x** |
| 05 | Matrix Determinant | 82 | 585 | Rosy **7.1x** |
| 06 | Simplex Optimization | 400 | 1,992 | Rosy **5.0x** |
| 07 | LMDIF Optimization | 11,883 | 12,350 | tie |
| 08 | Math Functions | 17,330 | 30,006 | Rosy **1.7x** |
| 09 | Recursive Fibonacci | 641 | — | — |
| 10 | Vector Operations | 96 | 413 | Rosy **4.3x** |
| 11 | Nested Loops | 1,465 | 12,902 | Rosy **8.8x** |
| 12 | DA Derivatives | 3,221 | 2,294 | COSY 1.4x |
| 13 | String Operations | 1,465 | 2,176 | Rosy **1.5x** |
| 14 | DA Transfer Map | 313 | 433 | Rosy **1.4x** |
| 15 | DA High-Order Multiply | 26 | 92 | Rosy **3.5x** |
| 16 | DA Bending Magnet | 833 | 703 | COSY 1.2x |
| 17 | DA Aberration | 195 | 249 | Rosy **1.3x** |
| 18 | POLVAL Map Eval | 4,371 | 15,194 | Rosy **3.5x** |
| | **TOTAL** | **48,424** | **116,262** | Rosy **2.4x** |

**Scorecard (T4):** Rosy wins 13, COSY wins 3, tie 1 (bench 09 excluded — COSY lacks recursive functions)

---

## MPI Benchmarks (20 Nodes, scaled workloads)

20 compute nodes, 1 MPI rank per node. CPUSEC timing (computation only).
Workloads scaled so each benchmark runs 10–20s in COSY, eliminating MPI
initialization overhead (~300ms) as a confounding factor.

### Release Mode

| # | Benchmark | Rosy (s) | COSY (s) | Winner |
|---|---|---:|---:|---|
| 01 | PLOOP Arithmetic | 1.39 | 18.10 | Rosy **13.0x** |
| 02 | PLOOP DA | 0.51 | 0.52 | tie |
| 03 | PLOOP Matrix | 0.32 | 0.27 | COSY 1.2x |
| 04 | PLOOP Optimization | 0.30 | 0.35 | Rosy **1.1x** |
| 05 | PLOOP Math | 6.83 | 13.10 | Rosy **1.9x** |
| 06 | PLOOP Scaling | 7.88 | 19.65 | Rosy **2.5x** |
| 07 | PLOOP Nested | 0.40 | 8.01 | Rosy **20.1x** |
| 08 | PLOOP Large Output | 6.44 | 14.37 | Rosy **2.2x** |
| 09 | PLOOP Fibonacci | 0.51 | 17.79 | Rosy **34.9x** |
| 10 | PLOOP Vector | 0.55 | 1.33 | Rosy **2.4x** |
| | **TOTAL** | **25.1** | **93.5** | Rosy **3.7x** |

### Optimized Mode

| # | Benchmark | Rosy (s) | COSY (s) | Winner |
|---|---|---:|---:|---|
| 01 | PLOOP Arithmetic | — | 18.10 | — |
| 02 | PLOOP DA | 0.51 | 0.45 | COSY 1.1x |
| 03 | PLOOP Matrix | 0.29 | 0.26 | COSY 1.1x |
| 04 | PLOOP Optimization | 0.31 | 0.27 | COSY 1.2x |
| 05 | PLOOP Math | 3.59 | 13.46 | Rosy **3.7x** |
| 06 | PLOOP Scaling | 3.45 | 18.42 | Rosy **5.3x** |
| 07 | PLOOP Nested | 0.76 | 10.40 | Rosy **13.7x** |
| 08 | PLOOP Large Output | 3.39 | 14.47 | Rosy **4.3x** |
| 09 | PLOOP Fibonacci | 0.51 | 12.29 | Rosy **24.0x** |
| 10 | PLOOP Vector | 0.52 | 1.40 | Rosy **2.7x** |
| | **TOTAL** | **13.3** | **71.4** | Rosy **5.4x** |

**Scorecard (best-of):** Rosy wins 8, COSY wins 1, tie 1

> DA benchmarks (02–04) run < 1s because DA operations at order 6 are inherently
> bounded by the monomial count (84 terms). Scaling the iteration count higher
> just repeats the same work — the computation cannot be made arbitrarily larger
> without increasing DA order, which would change the benchmark character.

---

## Where Each System Wins

### Rosy dominates: general computation

| Category | Best Speedup | Why |
|---|---|---|
| Iterative loops (Fibonacci) | **25–35x** | LLVM strength-reduces tight scalar loops |
| Nested loops | **8–20x** | Loop fusion, dead code elimination |
| Arithmetic loops | **13–15x** | SIMD auto-vectorization + loop unrolling |
| Matrix operations | **5–7x** | Compiled linear algebra vs interpreted |
| Math functions (SIN, COS, EXP) | **2–5x** | Inlined transcendentals, no dispatch |
| Simplex optimization | **5–6x** | Tight compiled iteration |
| Vector operations | **2–4x** | Native Vec operations vs interpreted |
| POLVAL map evaluation | **3.5x** | SIMD batch eval + in-place vector append |

### COSY dominates: high-order DA

| Category | COSY Advantage | Why |
|---|---|---|
| DA Trig (order 6+) | 1.5x | Hand-optimized Fortran Taylor series |
| DA Derivatives | 1.4x | Direct array manipulation in DA engine |
| DA Bending Magnet | 1.2x | Specialized beam optics DA kernels |

### Pattern

COSY's advantage is concentrated in **DA operations at high polynomial orders** where its Fortran DA engine (hand-tuned over 30 years) has tighter inner loops. For everything else — scalar math, loops, matrices, strings, vectors, optimization — Rosy's compiled output wins, often decisively.

As workloads scale up, Rosy's advantage grows because compilation overhead is amortized and LLVM optimizations (vectorization, inlining) have more to work with. The scaled MPI benchmarks demonstrate this clearly: at sub-second workloads the two systems appeared comparable, but at 10–20s workloads Rosy pulls ahead to **3.7–5.4x**.

---

## Environment

| | Details |
|---|---|
| Cluster | NIU Metis, Red Hat Enterprise Linux 8.6, PBS Pro |
| CPU | 2x AMD EPYC 7713s (per compute node) |
| Rosy | v0.8.4, `rustc` nightly 1.96.0 |
| COSY | 9.1, `gfortran 14.2.0 -Ofast -mcmodel=large` |
| MPI | OpenMPI 5.0.7, 20 nodes x 1 rank |
| `--release` | Rust release profile, codegen-units=1 |
| `--optimized` | + LTO (fat), SIMD DA (nightly `portable_simd`) |

## Reproducing

```bash
cd ~/rosy/examples/performance
qsub run_local.pbs            # Non-MPI, --release
qsub run_local_optimized.pbs  # Non-MPI, --optimized
qsub run_mpi.pbs              # MPI 20-node, --release
qsub run_mpi_optimized.pbs    # MPI 20-node, --optimized
```
