// DACELIB Benchmark: DA polynomial multiplication chains (T3)
// Matches: examples/performance/non_mpi/02_da_multiply/bench_t3.rosy
// Order 8, 4 variables => C(12,4) = 495 monomials per DA vector
// 50,000 iterations

#include <iostream>
#include <chrono>
#include <dace/dace.h>

using namespace DACE;

int main() {
    DA::init(8, 4);

    DA X = DA(1);
    DA Y = DA(2);
    DA Z = DA(3);
    DA W = DA(4);
    DA R;

    std::cout << "DA Multiply Benchmark (DACELIB): order 8, 4 vars, 50000 iterations" << std::endl;

    auto t1 = std::chrono::high_resolution_clock::now();

    for (int i = 0; i < 50000; i++) {
        R = (X + Y) * (Z + W);
        R = R * (X - Z);
        R = R * R;
        R = R + X * Y * Z * W;
    }

    auto t2 = std::chrono::high_resolution_clock::now();
    double elapsed = std::chrono::duration<double>(t2 - t1).count();

    std::cout << "Result constant part: " << cons(R) << std::endl;
    std::cout << "CPUSEC: " << elapsed << std::endl;

    return 0;
}
