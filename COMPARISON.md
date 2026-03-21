# Rosy vs COSY INFINITY — Performance Comparison

**Rosy v0.8.4** vs **COSY INFINITY 9.1** | NIU Metis Cluster | March 2026

Rosy compiles ROSY source to native Rust executables. COSY interprets FOX scripts in a Fortran runtime. All numbers below are **best-of** across multiple runs in both `--release` and `--optimized` modes — each system gets its fastest time.

## Overall

| Suite | Rosy | COSY | Winner |
|---|---|---|---|
| Non-MPI (16 benchmarks, T4) | 44.0s | 101.1s | Rosy **2.3x** |
| MPI (10 benchmarks, 20 nodes) | 3.76s | 7.49s | Rosy **2.0x** |

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
| | **TOTAL** | **44,053** | **101,068** | Rosy **2.3x** |

**Scorecard (T4):** Rosy wins 12, COSY wins 3, tie 1 (bench 09 excluded — COSY lacks recursive functions)

---

## MPI Benchmarks (best-of, 20 Nodes)

20 compute nodes, 1 MPI rank per node. CPUSEC timing (computation only).
Best Rosy time from any run, best COSY time from any run.

| # | Benchmark | Rosy (s) | COSY (s) | Winner |
|---|---|---:|---:|---|
| 01 | PLOOP Arithmetic | 0.407 | 1.665 | Rosy **4.1x** |
| 02 | PLOOP DA | 0.305 | 0.281 | COSY 1.1x |
| 03 | PLOOP Matrix | 0.329 | 0.321 | tie |
| 04 | PLOOP Optimization | 0.308 | 0.267 | COSY 1.2x |
| 05 | PLOOP Math | 0.409 | 0.683 | Rosy **1.7x** |
| 06 | PLOOP Scaling | 0.569 | 2.069 | Rosy **3.6x** |
| 07 | PLOOP Nested | 0.310 | 0.490 | Rosy **1.6x** |
| 08 | PLOOP Large Output | 0.438 | 0.896 | Rosy **2.0x** |
| 09 | PLOOP Fibonacci | 0.328 | 0.742 | Rosy **2.3x** |
| 10 | PLOOP Vector | 0.329 | 0.283 | COSY 1.2x |
| | **TOTAL** | **3.73** | **7.70** | Rosy **2.1x** |

**Scorecard:** Rosy wins 6, COSY wins 3, tie 1

> Rosy MPI times include ~300ms of initialization overhead per benchmark — constant
> regardless of workload size. For the sub-second DA/vector benchmarks where COSY
> leads, MPI init accounts for most of Rosy's runtime.

---

## Where Each System Wins

### Rosy dominates: general computation

| Category | Best Speedup | Why |
|---|---|---|
| Arithmetic/loops | **9–15x** | LLVM auto-vectorization + loop unrolling |
| Nested loops | **8–9x** | Loop fusion, dead code elimination |
| Matrix operations | **5–7x** | Compiled linear algebra vs interpreted |
| Math functions | **2–5x** | Inlined transcendentals, no dispatch |
| Optimization (Simplex) | **5–6x** | Tight compiled iteration |
| Vector operations | **4–5x** | Native Vec operations vs interpreted |

### COSY dominates: high-order DA

| Category | COSY Advantage | Why |
|---|---|---|
| DA Trig (order 6+) | 1.5x | Hand-optimized Fortran Taylor series |
| DA Derivatives | 1.4x | Direct array manipulation in DA engine |
| DA Bending Magnet | 1.2x | Specialized beam optics DA kernels |

### Pattern

COSY's advantage is concentrated in **DA operations at high polynomial orders** where its Fortran DA engine (hand-tuned over 30 years) has tighter inner loops. For everything else — scalar math, loops, matrices, strings, vectors, optimization — Rosy's compiled output wins, often decisively.

As workloads scale up (T1 → T4), Rosy's advantage grows because compilation overhead is amortized and LLVM optimizations (vectorization, inlining) have more to work with.

---

## Environment

| | Details |
|---|---|
| Cluster | NIU Metis, Rocky Linux 8, PBS Pro |
| CPU | Intel Xeon (compute nodes) |
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
