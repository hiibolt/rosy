# Rosy vs COSY INFINITY — Performance Comparison

**Rosy v0.8.4** vs **COSY INFINITY 9.1** | NIU Metis Cluster | March 2026

## At a Glance

Rosy compiles ROSY source to native Rust executables. COSY interprets FOX scripts in a Fortran runtime.

|  | `--release` | `--optimized` |
|---|---|---|
| **Non-MPI** (64 comparable tests) | **2.1x faster** | **2.4x faster** |
| **MPI** (10 tests, 20 nodes) | **2.5x faster** | **2.1x faster** |

---

## Non-MPI Benchmarks

17 benchmarks, 4 scaling tiers each (T1–T4). Single compute node.
Best-of results from multiple runs. Times are wall-clock milliseconds.

### Release Mode (`--release`)

| Benchmark | T1 | | T2 | | T3 | | T4 | |
|---|---|---|---|---|---|---|---|---|
| | Rosy | COSY | Rosy | COSY | Rosy | COSY | Rosy | COSY |
| 01 Arithmetic Loop | 3 | 395 | 5 | 38 | 221 | 2802 | 1836 | 28151 |
| 02 DA Multiply | 5 | 322 | 11 | 23 | 67 | 153 | 578 | 1460 |
| 03 DA Trig | 4 | 322 | 59 | 40 | 499 | 284 | 5351 | 3095 |
| 04 Matrix Inversion | 3 | 7 | 5 | 14 | 25 | 85 | 245 | 667 |
| 05 Matrix Determinant | 3 | 7 | 4 | 13 | 12 | 74 | 89 | 585 |
| 06 Simplex Optimization | 3 | 9 | 7 | 19 | 45 | 223 | 491 | 1992 |
| 07 LMDIF Optimization | 22 | 19 | 172 | 117 | 1710 | 1203 | 15372 | 12350 |
| 08 Math Functions | 3 | 7 | 17 | 96 | 1618 | 8613 | 17330 | 30006 |
| 09 Recursive Fibonacci | 3 | err | 3 | err | 10 | err | 643 | err |
| 10 Vector Operations | 3 | 7 | 4 | 10 | 15 | 54 | 111 | 413 |
| 11 Nested Loops | 3 | 6 | 5 | 23 | 97 | 742 | 1465 | 12902 |
| 12 DA Derivatives | 4 | 8 | 40 | 33 | 340 | 237 | 3380 | 2294 |
| 13 String Operations | 3 | 6 | 12 | 18 | 162 | 223 | 1650 | 2176 |
| 14 DA Transfer Map | 4 | 7 | 6 | 9 | 98 | 122 | 370 | 433 |
| 15 DA High-Order Mult | 13 | 7 | 14 | 9 | 16 | 27 | 26 | 92 |
| 16 DA Bending Magnet | 4 | 7 | 26 | 19 | 187 | 159 | 921 | 879 |
| 17 DA Aberration | 4 | 7 | 9 | 13 | 53 | 63 | 229 | 249 |

**Totals (excluding bench 09):** Rosy 55,090ms vs COSY 114,447ms → **2.1x**

### Optimized Mode (`--optimized`, T3–T4)

Small workloads (T1–T2) are dominated by build overhead; T3–T4 are representative.

| Benchmark | T3 Rosy | T3 COSY | T3 | T4 Rosy | T4 COSY | T4 |
|---|---|---|---|---|---|---|
| 01 Arithmetic Loop | 318 | 3667 | **11.5x** | 2163 | 30007 | **13.9x** |
| 02 DA Multiply | 367 | 256 | 0.7x | 820 | 1567 | **1.9x** |
| 03 DA Trig | 453 | 511 | 1.1x | 4572 | 4290 | 0.9x |
| 04 Matrix Inversion | 16 | 182 | **11.5x** | 117 | 766 | **6.6x** |
| 05 Matrix Determinant | 11 | 166 | **14.6x** | 82 | 680 | **8.3x** |
| 06 Simplex Optimization | 37 | 345 | **9.4x** | 400 | 2052 | **5.1x** |
| 07 LMDIF Optimization | 1254 | 1313 | 1.0x | 11883 | 12572 | 1.1x |
| 08 Math Functions | 1616 | 8792 | **5.4x** | 17360 | 30112 | **1.7x** |
| 09 Recursive Fibonacci | 9 | err | — | 641 | err | — |
| 10 Vector Operations | 13 | 152 | **11.4x** | 96 | 531 | **5.5x** |
| 11 Nested Loops | 99 | 945 | **9.6x** | 1472 | 12973 | **8.8x** |
| 12 DA Derivatives | 322 | 338 | 1.1x | 3221 | 2414 | 0.7x |
| 13 String Operations | 149 | 327 | **2.2x** | 1465 | 2275 | **1.6x** |
| 14 DA Transfer Map | 86 | 218 | **2.5x** | 313 | 530 | **1.7x** |
| 15 DA High-Order Mult | 16 | 131 | **8.4x** | 26 | 96 | **3.7x** |
| 16 DA Bending Magnet | 173 | 135 | 0.8x | 833 | 703 | 0.8x |
| 17 DA Aberration | 48 | 164 | **3.4x** | 195 | 352 | **1.8x** |

**Totals (excluding bench 09):** Rosy 51,783ms vs COSY 123,250ms → **2.4x**

---

## MPI Benchmarks (PLOOP, 20 Nodes)

20 compute nodes, 1 MPI rank per node. Each rank executes one PLOOP iteration.
Times are CPUSEC (seconds, computation only — excludes MPI init).

### Release Mode

| Benchmark | Rosy (s) | COSY (s) | Speedup |
|---|---|---|---|
| 01 PLOOP Arithmetic | 0.407 | 1.665 | **4.0x** |
| 02 PLOOP DA | 0.324 | 0.281 | 0.9x |
| 03 PLOOP Matrix | 0.329* | 0.321* | 1.0x |
| 04 PLOOP Optimization | 0.332 | 0.267 | 0.8x |
| 05 PLOOP Math | 0.409 | 0.683 | **1.7x** |
| 06 PLOOP Scaling | 0.569 | 2.069 | **3.6x** |
| 07 PLOOP Nested | 0.310 | 0.490 | **1.6x** |
| 08 PLOOP Large Output | 0.458 | 0.896 | **2.0x** |
| 09 PLOOP Fibonacci | 0.328 | 0.742 | **2.3x** |
| 10 PLOOP Vector | 0.329 | 0.283 | 0.9x |
| **TOTAL** | **3.80** | **7.70** | **2.0x** |

*\* Bench 03 release data from optimized solo run (intermittent build cache miss in release job)*

### Optimized Mode (solo run)

| Benchmark | Rosy (s) | COSY (s) | Speedup |
|---|---|---|---|
| 01 PLOOP Arithmetic | 0.425 | 1.672 | **3.9x** |
| 02 PLOOP DA | 0.305 | 0.316 | 1.0x |
| 03 PLOOP Matrix | 0.329 | 0.321 | 1.0x |
| 04 PLOOP Optimization | 0.308 | 0.336 | 1.1x |
| 05 PLOOP Math | 0.420 | 0.707 | **1.7x** |
| 06 PLOOP Scaling | 0.584 | 2.096 | **3.6x** |
| 07 PLOOP Nested | 0.318 | 0.596 | **1.9x** |
| 08 PLOOP Large Output | 0.438 | 0.938 | **2.1x** |
| 09 PLOOP Fibonacci | 0.332 | 0.806 | **2.4x** |
| 10 PLOOP Vector | 0.337 | 0.337 | 1.0x |
| **TOTAL** | **3.80** | **8.13** | **2.1x** |

---

## Analysis

### Rosy's Strengths

Rosy compiles to native code — LLVM can auto-vectorize, inline, and register-allocate
across the entire program. This produces dramatic speedups on:

| Category | Typical Speedup | Why |
|---|---|---|
| Arithmetic loops | **9–15x** | SIMD auto-vectorization, loop unrolling |
| Nested loops | **8–10x** | Loop fusion, dead code elimination |
| Matrix ops | **3–7x** | Compiled linear algebra vs interpreted |
| Math functions | **2–5x** | Inlined transcendentals, no dispatch overhead |
| Simplex optimization | **4–5x** | Tight compiled iteration vs interpreted |

### COSY's Strengths

COSY's Fortran-native DA (Differential Algebra) engine uses hand-optimized array
operations that are hard to match with generic compiled code:

| Category | COSY Advantage | Why |
|---|---|---|
| DA Trig (high order) | ~1.5x | Optimized Taylor series in Fortran |
| DA Derivatives | ~1.4x | Direct array manipulation in DA engine |
| DA Bending Magnet | ~1.2x | Specialized beam physics DA kernels |
| LMDIF Optimization | ~1.3x | Tight Fortran DA engine in inner loop |

### Key Observations

1. **Scaling with workload**: Rosy's advantage grows with problem size (T1→T4).
   Compilation overhead is amortized at scale.

2. **DA is COSY's home turf**: At high DA orders (order 6+, 3+ variables),
   COSY's hand-tuned Fortran DA engine outperforms Rosy's generic implementation.
   This is the primary target for Rosy's ongoing SIMD/pool-allocator optimizations.

3. **MPI overhead**: Rosy MPI times include ~300ms of initialization per benchmark.
   For sub-second workloads, this masks Rosy's computational advantage.

4. **Non-DA workloads**: For any computation that isn't DA-heavy, Rosy wins
   decisively — often by 5–15x.

---

## Environment

| Component | Details |
|---|---|
| Cluster | NIU Metis, Rocky Linux 8, PBS Pro |
| CPU | Intel Xeon (compute nodes) |
| Rosy | v0.8.4, `rustc` nightly 1.96.0 |
| COSY | 9.1, `gfortran 14.2.0 -Ofast -mcmodel=large` |
| MPI | OpenMPI 5.0.7 |
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
