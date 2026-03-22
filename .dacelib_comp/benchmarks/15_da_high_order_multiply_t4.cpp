// DACELIB Benchmark: High-order DA polynomial multiplication (T4)
// Matches: examples/performance/non_mpi/15_da_high_order_multiply/bench_t4.rosy
// Order 7, 6 variables => C(13,6) = 1716 monomials per DA vector
// 5,000 iterations

#include <iostream>
#include <chrono>
#include <dace/dace.h>

using namespace DACE;

int main() {
    DA::init(7, 6);

    DA X  = DA(1);
    DA PX = DA(2);
    DA Y  = DA(3);
    DA PY = DA(4);
    DA Z  = DA(5);
    DA D  = DA(6);
    DA F, G, R;

    std::cout << "DA High-Order Multiply Benchmark (DACELIB): order 7, 6 vars, 5000 iterations" << std::endl;

    auto t1 = std::chrono::high_resolution_clock::now();

    for (int i = 0; i < 5000; i++) {
        // Build realistic polynomials with cross-terms
        F = X * PX + Y * PY + Z * D;
        G = X * Y + PX * PY + Z * Z;

        // Heavy multiply chain
        R = (F + G) * (F - G);
        R = R * R + F * G;
        R = R + X * Y * Z * PX * PY * D;
    }

    auto t2 = std::chrono::high_resolution_clock::now();
    double elapsed = std::chrono::duration<double>(t2 - t1).count();

    std::cout << "Result constant part: " << cons(R) << std::endl;
    std::cout << "CPUSEC: " << elapsed << std::endl;

    return 0;
}
