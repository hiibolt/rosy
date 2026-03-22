================================================================
  Rosy (release) vs Rosy (optimized) vs DACELIB (-O3)
  Best-of-50 runs
================================================================

  Rosy:    /home/hiibolt/Development/rosy/target/release/rosy (rosy 0.8.4)
  DACELIB: DACE 2.1.0, Clang 21.1.8 -O3
  Date:    Sun Mar 22 02:42:25 PM CDT 2026
  Host:    nuclearbombconsole
  CPU:     12th Gen Intel(R) Core(TM) i7-1260P

Building Rosy binaries...
  Done.

================================================================
  T3 Results (best of 50)
================================================================
Benchmark              Rosy Rel   Rosy Opt    DACELIB    Rel/DACE    Opt/DACE
-------------------- ---------- ---------- ---------- ----------- -----------
DA Multiply                42.2       39.2       88.1       2.09x       2.25x   
DA Trig                   342.5      331.8      116.3       0.34x       0.35x   
DA Derivatives            243.9      235.1      169.2       0.69x       0.72x   
DA Transfer Map            66.2       59.4      113.3       1.71x       1.91x   
DA High-Order Mul          12.5       12.2       11.2       0.90x       0.92x   
DA Bending Magnet         129.2      130.5       55.7       0.43x       0.43x   
DA Aberration              33.1       29.9       40.6       1.23x       1.36x   
-------------------- ---------- ---------- ---------- ----------- -----------
TOTAL                     869.6      838.1      594.4       0.68x       0.71x

  Release:   Rosy wins 3, DACELIB wins 4, ties 0
  Optimized: Rosy wins 3, DACELIB wins 4, ties 0

================================================================
  T4 Results (best of 50)
================================================================
Benchmark              Rosy Rel   Rosy Opt    DACELIB    Rel/DACE    Opt/DACE
-------------------- ---------- ---------- ---------- ----------- -----------
DA Multiply               405.7      358.4      851.4       2.10x       2.38x   
DA Trig                  3697.1     3307.7     1181.8       0.32x       0.36x   
DA Derivatives           2449.4     2371.6     1666.7       0.68x       0.70x   
DA Transfer Map           255.7      232.3      449.8       1.76x       1.94x   
DA High-Order Mul            20       17.7       46.3       2.31x       2.62x   
DA Bending Magnet         653.7      676.4      270.1       0.41x       0.40x   
DA Aberration             156.9        141      192.6       1.23x       1.37x   
-------------------- ---------- ---------- ---------- ----------- -----------
TOTAL                    7638.5     7105.1     4658.7       0.61x       0.66x

  Release:   Rosy wins 4, DACELIB wins 3, ties 0
  Optimized: Rosy wins 4, DACELIB wins 3, ties 0

================================================================
  Ratio > 1.0x = Rosy faster   |  Ratio < 1.0x = DACELIB faster
================================================================