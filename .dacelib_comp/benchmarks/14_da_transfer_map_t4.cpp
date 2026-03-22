// DACELIB Benchmark: DA transfer map for a FODO cell (T4)
// Matches: examples/performance/non_mpi/14_da_transfer_map/bench_t4.rosy
// Order 5, 6 variables => C(11,6) = 462 monomials per DA vector
// 200,000 iterations

#include <iostream>
#include <chrono>
#include <dace/dace.h>

using namespace DACE;

int main() {
    DA::init(5, 6);

    DA X  = DA(1);
    DA PX = DA(2);
    DA Y  = DA(3);
    DA PY = DA(4);
    DA Z  = DA(5);
    DA D  = DA(6);
    DA X1, PX1, Y1, PY1, Z1;
    DA X2, PX2, Y2, PY2, Z2;
    DA XF, YF, ZF, R;

    std::cout << "DA Transfer Map Benchmark (DACELIB): order 5, 6 vars, 200000 iterations" << std::endl;

    auto t1 = std::chrono::high_resolution_clock::now();

    for (int i = 0; i < 200000; i++) {
        // Drift 1: L=1.0
        X1 = X + 1.0 * PX;
        Y1 = Y + 1.0 * PY;
        Z1 = Z + 1.0 * (1.0 + D);

        // Quadrupole focusing kick: K=0.5
        PX1 = PX - 0.5 * X1;
        PY1 = PY + 0.5 * Y1;

        // Drift 2: L=2.0
        X2 = X1 + 2.0 * PX1;
        Y2 = Y1 + 2.0 * PY1;
        Z2 = Z1 + 2.0 * (1.0 + D);

        // Quadrupole defocusing kick: K=-0.5
        PX2 = PX1 + 0.5 * X2;
        PY2 = PY1 - 0.5 * Y2;

        // Drift 3: L=1.0
        XF = X2 + 1.0 * PX2;
        YF = Y2 + 1.0 * PY2;
        ZF = Z2 + 1.0 * (1.0 + D);

        // Combine results to prevent optimization
        R = XF * XF + YF * YF + PX2 * PX2 + PY2 * PY2;
    }

    auto t2 = std::chrono::high_resolution_clock::now();
    double elapsed = std::chrono::duration<double>(t2 - t1).count();

    std::cout << "Result constant part: " << cons(R) << std::endl;
    std::cout << "CPUSEC: " << elapsed << std::endl;

    return 0;
}
