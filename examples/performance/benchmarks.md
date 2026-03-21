# Rosy vs COSY INFINITY Performance Benchmarks

**Rosy v0.8.2** | **COSY INFINITY 9.1** | NIU Metis Cluster | March 2026

## Summary

| Suite | Mode | Rosy | COSY | Speedup |
|---|---|---|---|---|
| Non-MPI (68 tests) | `--release` | 56.0s | 112.6s | **2.0x** |
| Non-MPI (68 tests) | `--optimized` | 50.5s | 115.1s | **2.3x** |
| MPI, 20 nodes (9 tests) | `--release` | 3.52s | 7.62s | **2.2x** |
| MPI, 20 nodes (9 tests) | `--optimized` | 3.86s | 6.39s | **1.7x** |

Rosy compiles ROSY source to native Rust binaries; COSY interprets FOX scripts.
Speedup = COSY time / Rosy time. Values > 1.0 mean Rosy is faster.

---

## Test Environment

- **Cluster**: NIU Metis (Rocky Linux 8, PBS Pro)
- **CPU**: Intel Xeon (compute nodes)
- **Rosy**: v0.8.2, compiled with `rustc` nightly (for `--optimized`: LTO + single codegen-unit + SIMD DA)
- **COSY**: COSY INFINITY 9.1, compiled with `gfortran 14.2.0 -Ofast -mcmodel=large` + OpenMPI 5.0.7
- **MPI**: OpenMPI 5.0.7 (GCC 14.2.0), 20 nodes, 1 rank per node

---

## Non-MPI Benchmarks

17 benchmarks across 4 scaling tiers (T1=small, T2=medium, T3=large, T4=stress).
Each benchmark tests a distinct language feature or computational pattern.

### `--release` mode

| # | Benchmark | T1 | T2 | T3 | T4 |
|---|---|---|---|---|---|
| 01 | Arithmetic Loop | 0.2x | **7.6x** | **17.9x** | **14.9x** |
| 02 | DA Multiply | 1.2x | **2.1x** | **2.3x** | **2.5x** |
| 03 | DA Trig | **2.1x** | 0.8x | 0.6x | 0.5x |
| 04 | Matrix Inversion | **8.9x** | **2.6x** | **3.1x** | **3.4x** |
| 05 | Matrix Determinant | 0.3x | **3.0x** | 0.8x | **5.3x** |
| 06 | Optimization (Simplex) | **2.5x** | **3.3x** | **5.1x** | **4.1x** |
| 07 | Optimization (LMDIF) | 0.6x | 0.7x | 0.7x | 0.8x |
| 08 | Math Functions | **3.3x** | **5.9x** | **5.2x** | **1.7x** |
| 09 | Recursive Fibonacci | N/A | N/A | N/A | N/A |
| 10 | Vector Operations | 1.1x | **3.2x** | **3.8x** | **4.0x** |
| 11 | Nested Loops | 0.6x | **2.1x** | **7.8x** | **8.8x** |
| 12 | DA Derivatives | 0.6x | 0.8x | 0.7x | 0.7x |
| 13 | String Operations | 0.5x | **1.6x** | **1.4x** | **1.4x** |
| 14 | DA Transfer Map | **1.8x** | **1.7x** | **1.3x** | 1.2x |
| 15 | DA High-Order Multiply | 0.2x | 0.7x | **1.6x** | **3.4x** |
| 16 | DA Bending Magnet | **1.9x** | 0.8x | 0.6x | 0.6x |
| 17 | DA Aberration | **1.7x** | 0.4x | **1.3x** | 1.1x |
| | **TOTAL** | | | | **2.0x** |

### `--optimized` mode

| # | Benchmark | T1 | T2 | T3 | T4 |
|---|---|---|---|---|---|
| 01 | Arithmetic Loop | **3.2x** | **26.9x** | **13.2x** | **14.4x** |
| 02 | DA Multiply | **3.2x** | **12.3x** | **3.8x** | **2.9x** |
| 03 | DA Trig | **29.8x** | **2.1x** | 0.8x | 0.6x |
| 04 | Matrix Inversion | **7.8x** | **28.2x** | **11.1x** | **6.4x** |
| 05 | Matrix Determinant | **3.6x** | **7.8x** | **14.8x** | **8.2x** |
| 06 | Optimization (Simplex) | **3.1x** | **21.2x** | **9.5x** | **5.2x** |
| 07 | Optimization (LMDIF) | **6.5x** | **1.6x** | 1.1x | 1.1x |
| 08 | Math Functions | **34.4x** | **11.8x** | **4.8x** | **1.7x** |
| 09 | Recursive Fibonacci | N/A | N/A | N/A | N/A |
| 10 | Vector Operations | **3.8x** | **5.2x** | **2.0x** | **5.3x** |
| 11 | Nested Loops | **4.2x** | **7.1x** | **9.0x** | **8.3x** |
| 12 | DA Derivatives | **2.7x** | **3.3x** | 1.0x | 0.8x |
| 13 | String Operations | **36.8x** | **11.1x** | **2.2x** | **1.5x** |
| 14 | DA Transfer Map | **4.6x** | **18.7x** | **1.3x** | **1.8x** |
| 15 | DA High-Order Multiply | **8.0x** | **8.8x** | **8.5x** | **7.7x** |
| 16 | DA Bending Magnet | **33.8x** | **5.6x** | 1.2x | 0.7x |
| 17 | DA Aberration | **1.7x** | **1.5x** | **1.4x** | **1.3x** |
| | **TOTAL** | | | | **2.3x** |

> **Note**: Optimized T1/T2 COSY times include ~100ms process startup overhead that
> dominates tiny workloads. At T3/T4 scale (where startup is negligible), the comparison
> is more representative.

### Where COSY wins

COSY's Fortran-native DA engine outperforms Rosy on DA-heavy workloads at high orders:

- **DA Trig** (T3-T4): COSY's transcendental Taylor series evaluation is ~1.7x faster
- **DA Derivatives** (T3-T4): COSY ~1.3x faster
- **DA Bending Magnet** (T3-T4): COSY ~1.6x faster
- **LMDIF Optimization** (T2-T4): COSY slightly faster (~1.2x), likely due to its built-in DA engine being tighter for iterative optimization

### Where Rosy wins

Rosy's compiled output excels on general computation:

- **Arithmetic/Loops** (T3-T4): **9-18x** faster (LLVM auto-vectorization)
- **Matrix Inversion**: **3-9x** faster
- **Math Functions**: **2-6x** faster
- **Nested Loops**: **3-9x** faster
- **Simplex Optimization**: **3-5x** faster

---

## MPI Benchmarks (PLOOP)

10 benchmarks using PLOOP (parallel loop) across 20 compute nodes.
Each rank executes one PLOOP iteration independently. COSY uses MPI_ALLGATHER
for result collection; Rosy uses Rayon + MPI.

### `--release` mode

| # | Benchmark | Rosy (s) | COSY (s) | Speedup |
|---|---|---|---|---|
| 01 | PLOOP Arithmetic | 0.407 | 1.726 | **4.2x** |
| 02 | PLOOP DA | 0.324 | 0.281 | 0.9x |
| 03 | PLOOP Matrix | BUILD FAIL | - | - |
| 04 | PLOOP Optimization | 0.332 | 0.267 | 0.8x |
| 05 | PLOOP Math | 0.409 | 0.683 | **1.7x** |
| 06 | PLOOP Scaling | 0.569 | 2.069 | **3.6x** |
| 07 | PLOOP Nested | 0.310 | 0.515 | **1.7x** |
| 08 | PLOOP Large Output | 0.461 | 1.052 | **2.3x** |
| 09 | PLOOP Fibonacci | 0.375 | 0.742 | **2.0x** |
| 10 | PLOOP Vector | 0.329 | 0.283 | 0.9x |
| | **TOTAL** | **3.52** | **7.62** | **2.2x** |

### `--optimized` mode

| # | Benchmark | Rosy (s) | COSY (s) | Speedup |
|---|---|---|---|---|
| 01 | PLOOP Arithmetic | 0.430 | 1.693 | **3.9x** |
| 02 | PLOOP DA | 0.353 | 0.110 | 0.3x |
| 03 | PLOOP Matrix | BUILD FAIL | - | - |
| 04 | PLOOP Optimization | 0.337 | 0.131 | 0.4x |
| 05 | PLOOP Math | 0.452 | 0.541 | **1.2x** |
| 06 | PLOOP Scaling | 0.631 | 1.998 | **3.2x** |
| 07 | PLOOP Nested | 0.388 | 0.366 | 0.9x |
| 08 | PLOOP Large Output | 0.536 | 0.813 | **1.5x** |
| 09 | PLOOP Fibonacci | 0.364 | 0.605 | **1.7x** |
| 10 | PLOOP Vector | 0.368 | 0.132 | 0.4x |
| | **TOTAL** | **3.86** | **6.39** | **1.7x** |

> **Note**: Benchmark 03 (PLOOP Matrix) fails to build in Rosy due to a missing 2D array
> transpilation feature. COSY runs it successfully.

### MPI analysis

Rosy's Rust-compiled binaries show the largest advantage on compute-heavy PLOOP
iterations (arithmetic: 4x, scaling: 3.6x, math: 1.7x). COSY edges ahead on
short-lived DA/vector operations where its interpreter overhead is amortized by
the efficient Fortran DA engine.

The Rosy MPI times include ~300ms of MPI initialization overhead per benchmark
(constant across all benchmarks), which proportionally impacts shorter runs more.

---

## Methodology

### Build modes

| Flag | Rust profile | LTO | Codegen Units | SIMD DA |
|---|---|---|---|---|
| (none) | `debug` | no | default | no |
| `--release` | `release` | no | 1 | no |
| `--optimized` | `release` | fat | 1 | yes (nightly) |

### Tier definitions

- **T1**: Minimal workload (baseline, tests startup + feature correctness)
- **T2**: Light workload (seconds-scale, basic scaling)
- **T3**: Medium workload (10s of seconds, representative of real use)
- **T4**: Stress test (minutes-scale, sustained throughput; 30s timeout)

### Timing

- Non-MPI: wall-clock time (includes process startup)
- MPI: `CPUSEC` intrinsic (measures computation time only, excludes MPI init)

### COSY MPI binary

Built from pre-converted MPI source files at `/opt/metis/opt-gaea/contrib/cosy/cosy-9.1-intel2020/`
(maintained by cluster admins). Compiled with:

```
mpif77 -m64 -Ofast -mcmodel=large -fallow-argument-mismatch -std=legacy
```

### Reproducing

```bash
# On NIU Metis:
cd ~/rosy/examples/performance

# Submit individual jobs
qsub run_local.pbs            # Non-MPI, --release
qsub run_local_optimized.pbs  # Non-MPI, --optimized
qsub run_mpi.pbs              # MPI 20-node, --release
qsub run_mpi_optimized.pbs    # MPI 20-node, --optimized

# Or run locally (non-MPI only):
./run_local.sh --cosy /path/to/cosy
```
