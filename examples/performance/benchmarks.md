# Rosy vs COSY INFINITY Performance Benchmarks

**Rosy v0.8.4** | **COSY INFINITY 9.1** | NIU Metis Cluster | March 2026

## Summary

| Suite | Mode | Rosy | COSY | Speedup |
|---|---|---|---|---|
| Non-MPI (68 tests) | `--release` | 55.7s | 114.4s | **2.1x** |
| Non-MPI (68 tests) | `--optimized` | 52.4s | 123.3s | **2.4x** |
| MPI, 20 nodes (10 tests) | `--release` | 3.57s | 8.88s | **2.5x** |
| MPI, 20 nodes (10 tests) | `--optimized` | 3.38s | 5.10s | **1.5x** |

Rosy compiles ROSY source to native Rust binaries; COSY interprets FOX scripts.
Speedup = COSY time / Rosy time. Values > 1.0 mean Rosy is faster.

---

## Test Environment

- **Cluster**: NIU Metis (Rocky Linux 8, PBS Pro)
- **CPU**: Intel Xeon (compute nodes)
- **Rosy**: v0.8.4, compiled with `rustc` nightly (for `--optimized`: LTO + single codegen-unit + SIMD DA)
- **COSY**: COSY INFINITY 9.1, compiled with `gfortran 14.2.0 -Ofast -mcmodel=large` + OpenMPI 5.0.7
- **MPI**: OpenMPI 5.0.7 (GCC 14.2.0), 20 nodes, 1 rank per node

---

## Non-MPI Benchmarks

17 benchmarks across 4 scaling tiers (T1=small, T2=medium, T3=large, T4=stress).
Each benchmark tests a distinct language feature or computational pattern.

### `--release` mode

| # | Benchmark | T1 | T2 | T3 | T4 |
|---|---|---|---|---|---|
| 01 | Arithmetic Loop | — | **7.0x** | **12.7x** | **15.3x** |
| 02 | DA Multiply | — | **2.1x** | **2.3x** | **2.5x** |
| 03 | DA Trig | — | 0.7x | 0.6x | 0.6x |
| 04 | Matrix Inversion | **2.2x** | **2.7x** | **3.3x** | **2.7x** |
| 05 | Matrix Determinant | **2.2x** | **3.3x** | **6.0x** | **6.5x** |
| 06 | Optimization (Simplex) | **2.6x** | **2.8x** | **5.0x** | **4.1x** |
| 07 | Optimization (LMDIF) | 0.9x | 0.7x | 0.7x | 0.8x |
| 08 | Math Functions | **2.3x** | **5.5x** | **5.3x** | **1.7x** |
| 09 | Recursive Fibonacci | N/A | N/A | N/A | N/A |
| 10 | Vector Operations | **2.4x** | **2.6x** | **3.7x** | **3.7x** |
| 11 | Nested Loops | **2.1x** | **5.1x** | **7.7x** | **8.8x** |
| 12 | DA Derivatives | **2.0x** | 0.8x | 0.7x | 0.7x |
| 13 | String Operations | **2.1x** | **1.5x** | **1.4x** | **1.3x** |
| 14 | DA Transfer Map | **1.7x** | **1.5x** | **1.2x** | 1.2x |
| 15 | DA High-Order Multiply | 0.6x | 0.6x | **1.7x** | **3.5x** |
| 16 | DA Bending Magnet | **1.8x** | 0.7x | 0.8x | 1.0x |
| 17 | DA Aberration | **1.6x** | **1.3x** | **1.2x** | 1.1x |
| | **TOTAL** | | | | **2.1x** |

### `--optimized` mode (T3-T4 focus)

| # | Benchmark | T3 | T4 |
|---|---|---|---|
| 01 | Arithmetic Loop | **11.5x** | **13.9x** |
| 02 | DA Multiply | 0.7x | **1.9x** |
| 03 | DA Trig | 1.1x | 0.9x |
| 04 | Matrix Inversion | **11.5x** | **6.6x** |
| 05 | Matrix Determinant | **14.6x** | **8.3x** |
| 06 | Optimization (Simplex) | **9.4x** | **5.1x** |
| 07 | Optimization (LMDIF) | 1.0x | 1.1x |
| 08 | Math Functions | **5.4x** | **1.7x** |
| 09 | Recursive Fibonacci | N/A | N/A |
| 10 | Vector Operations | **11.4x** | **5.5x** |
| 11 | Nested Loops | **9.6x** | **8.8x** |
| 12 | DA Derivatives | 1.1x | 0.7x |
| 13 | String Operations | **2.2x** | **1.6x** |
| 14 | DA Transfer Map | **2.5x** | **1.7x** |
| 15 | DA High-Order Multiply | **8.4x** | **3.7x** |
| 16 | DA Bending Magnet | 0.8x | 0.8x |
| 17 | DA Aberration | **3.4x** | **1.8x** |
| | **TOTAL** | | **2.4x** |

> **Note**: Optimized T1/T2 results are omitted — COSY process startup (~100ms)
> dominates tiny workloads. T3-T4 timings are representative of real-world use.

### Where COSY wins

COSY's Fortran-native DA engine outperforms Rosy on DA-heavy workloads at high orders:

- **DA Trig** (T3-T4): COSY's transcendental Taylor series evaluation is ~1.5x faster
- **DA Derivatives** (T4): COSY ~1.4x faster
- **DA Bending Magnet** (T3-T4): COSY ~1.2x faster
- **LMDIF Optimization**: COSY ~1.3x faster (built-in DA engine tighter for iterative optimization)

### Where Rosy wins

Rosy's compiled output excels on general computation:

- **Arithmetic/Loops** (T3-T4): **9-15x** faster (LLVM auto-vectorization)
- **Matrix Inversion/Determinant**: **3-7x** faster
- **Math Functions**: **2-5x** faster
- **Nested Loops**: **8-9x** faster
- **Simplex Optimization**: **4-5x** faster
- **Vector Operations**: **4-6x** faster

---

## MPI Benchmarks (PLOOP)

10 benchmarks using PLOOP (parallel loop) across 20 compute nodes.
Each rank executes one PLOOP iteration independently. COSY uses MPI_ALLGATHER
for result collection; Rosy uses MPI (via the `mpi` crate).

### `--release` mode

| # | Benchmark | Rosy (s) | COSY (s) | Speedup |
|---|---|---|---|---|
| 01 | PLOOP Arithmetic | 0.412 | 1.665 | **4.0x** |
| 02 | PLOOP DA | 0.326 | 0.286 | 0.9x |
| 03 | PLOOP Matrix | build err | 0.138 | — |
| 04 | PLOOP Optimization | 0.349 | 0.297 | 0.8x |
| 05 | PLOOP Math | 0.426 | 0.768 | **1.8x** |
| 06 | PLOOP Scaling | 0.578 | 3.347 | **5.8x** |
| 07 | PLOOP Nested | 0.365 | 0.490 | **1.3x** |
| 08 | PLOOP Large Output | 0.458 | 0.896 | **2.0x** |
| 09 | PLOOP Fibonacci | 0.328 | 0.815 | **2.5x** |
| 10 | PLOOP Vector | 0.330 | 0.319 | 1.0x |
| | **TOTAL** | **3.57** | **8.88** | **2.5x** |

### `--optimized` mode

| # | Benchmark | Rosy (s) | COSY (s) | Speedup |
|---|---|---|---|---|
| 01 | PLOOP Arithmetic | build err | 1.693 | — |
| 02 | PLOOP DA | 0.368 | 0.357 | 1.0x |
| 03 | PLOOP Matrix | 0.397 | 0.138 | 0.3x |
| 04 | PLOOP Optimization | build err | 0.117 | — |
| 05 | PLOOP Math | 0.430 | 0.541 | **1.3x** |
| 06 | PLOOP Scaling | 0.621 | 2.155 | **3.5x** |
| 07 | PLOOP Nested | 0.398 | 0.364 | 0.9x |
| 08 | PLOOP Large Output | 0.470 | 0.788 | **1.7x** |
| 09 | PLOOP Fibonacci | 0.358 | 0.635 | **1.8x** |
| 10 | PLOOP Vector | 0.337 | 0.119 | 0.4x |
| | **TOTAL** | **3.38** | **5.10** | **1.5x** |

> **Note**: "build err" entries are intermittent cold-cache failures from concurrent PBS jobs
> sharing the cargo registry. The 2D array fix (v0.8.4) is confirmed working — bench 03
> builds and runs correctly (see optimized results and manual testing).

### MPI analysis

Rosy's Rust-compiled binaries show the largest advantage on compute-heavy PLOOP
iterations (arithmetic: 4x, scaling: 5.8x, math: 1.8x). COSY edges ahead on
short-lived DA/vector operations where its interpreter overhead is amortized by
the efficient Fortran DA engine.

Rosy MPI times include ~300ms of MPI initialization overhead per benchmark
(constant across all benchmarks), which proportionally impacts shorter runs more.

---

## Build Modes

| Flag | Rust Profile | LTO | Codegen Units | SIMD DA |
|---|---|---|---|---|
| (none) | `debug` | no | default | no |
| `--release` | `release` | no | 1 | no |
| `--optimized` | `release` | fat | 1 | yes (nightly) |

## Tier Definitions

| Tier | Purpose | Typical Runtime |
|---|---|---|
| T1 | Baseline/correctness | < 100ms |
| T2 | Light scaling | ~100ms–1s |
| T3 | Representative workload | 1–10s |
| T4 | Stress test (30s timeout) | 10s–30s |

## Timing

- **Non-MPI**: wall-clock time (includes process startup)
- **MPI**: `CPUSEC` intrinsic (computation time only, excludes MPI init)

## COSY MPI Binary

Built from pre-converted MPI source at `/opt/metis/opt-gaea/contrib/cosy/cosy-9.1-intel2020/`:

```
mpif77 -m64 -Ofast -mcmodel=large -fallow-argument-mismatch -std=legacy
```

## Reproducing

```bash
# On NIU Metis:
cd ~/rosy/examples/performance
qsub run_local.pbs            # Non-MPI, --release
qsub run_local_optimized.pbs  # Non-MPI, --optimized
qsub run_mpi.pbs              # MPI 20-node, --release
qsub run_mpi_optimized.pbs    # MPI 20-node, --optimized

# Or run locally (non-MPI only):
./run_local.sh --cosy /path/to/cosy
```
