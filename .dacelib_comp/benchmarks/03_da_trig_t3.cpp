// DACELIB Benchmark: DA transcendental functions (T3)
// Matches: examples/performance/non_mpi/03_da_trig/bench_t3.rosy
// Order 5, 2 variables => C(7,2) = 21 monomials per DA vector
// 50,000 iterations

#include <iostream>
#include <chrono>
#include <dace/dace.h>

using namespace DACE;

int main() {
    DA::init(5, 2);

    DA X = DA(1) + 0.5;
    DA Y = DA(2) + 0.3;
    DA R;

    std::cout << "DA Trig Benchmark (DACELIB): order 5, 2 vars, 50000 iterations" << std::endl;

    auto t1 = std::chrono::high_resolution_clock::now();

    for (int i = 0; i < 50000; i++) {
        R = sin(X + Y) * cos(X - Y) + exp(X * 0.01);
        R = R + log(1.0 + X * X * 0.1);
        R = sqrt(R * R + 1.0);
    }

    auto t2 = std::chrono::high_resolution_clock::now();
    double elapsed = std::chrono::duration<double>(t2 - t1).count();

    std::cout << "Result constant part: " << cons(R) << std::endl;
    std::cout << "CPUSEC: " << elapsed << std::endl;

    return 0;
}
