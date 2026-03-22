// DACELIB Benchmark: DA sector bending magnet with edge focusing (T3)
// Matches: examples/performance/non_mpi/16_da_bending_magnet/bench_t3.rosy
// Order 5, 4 variables => C(9,4) = 126 monomials per DA vector
// 10,000 iterations

#include <iostream>
#include <chrono>
#include <dace/dace.h>

using namespace DACE;

int main() {
    DA::init(5, 4);

    DA X  = DA(1) + 0.001;
    DA PX = DA(2) + 0.001;
    DA Y  = DA(3) + 0.001;
    DA PY = DA(4) + 0.001;
    DA XR, PXR, YR, PYR, S, R;

    double THETA = 0.1;

    std::cout << "DA Bending Magnet Benchmark (DACELIB): order 5, 4 vars, 10000 iterations" << std::endl;

    auto t1 = std::chrono::high_resolution_clock::now();

    for (int i = 0; i < 10000; i++) {
        // Sector bend rotation
        XR  = X * cos(THETA * (1.0 + PX)) + sin(THETA * (1.0 + PX));
        PXR = PX * cos(THETA * (1.0 + PX)) - X * sin(THETA * (1.0 + PX));

        // Edge focusing
        YR  = Y + PY * 0.5;
        PYR = PY - 0.3 * Y;

        // Path length
        S = sqrt(XR * XR + YR * YR + 1.0);
        R = XR + PXR + YR + PYR + S;
    }

    auto t2 = std::chrono::high_resolution_clock::now();
    double elapsed = std::chrono::duration<double>(t2 - t1).count();

    std::cout << "Result constant part: " << cons(R) << std::endl;
    std::cout << "CPUSEC: " << elapsed << std::endl;

    return 0;
}
