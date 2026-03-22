// DACELIB Benchmark: DA aberration coefficient extraction (T4)
// Matches: examples/performance/non_mpi/17_da_aberration/bench_t4.rosy
// Order 5, 6 variables => C(11,6) = 462 monomials per DA vector
// 50,000 iterations

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
    DA X1, PX1, Y1, PY1;
    DA X2, PX2, Y2, PY2;
    DA DX, DPX, DY, DPY;
    DA DDX, DDPX;
    DA R;

    std::cout << "DA Aberration Benchmark (DACELIB): order 5, 6 vars, 50000 iterations" << std::endl;

    auto t1 = std::chrono::high_resolution_clock::now();

    for (int i = 0; i < 50000; i++) {
        // Build transfer map for first element
        X1  = X + PX + 0.1 * X * X - 0.05 * Y * Y;
        PX1 = PX - 0.3 * X + 0.02 * X * X * X;
        Y1  = Y + PY + 0.1 * Y * Y - 0.05 * X * X;
        PY1 = PY - 0.3 * Y + 0.02 * Y * Y * Y;

        // Compose with second element
        X2  = X1 + PX1 + 0.1 * X1 * X1;
        PX2 = PX1 - 0.3 * X1;
        Y2  = Y1 + PY1 + 0.1 * Y1 * Y1;
        PY2 = PY1 - 0.3 * Y1;

        // Extract first-order aberration coefficients
        DX  = X2.deriv(1);
        DPX = PX2.deriv(2);
        DY  = Y2.deriv(3);
        DPY = PY2.deriv(4);

        // Extract second-order aberration coefficients
        DDX  = DX.deriv(1);
        DDPX = DPX.deriv(2);

        R = DDX + DDPX + DY + DPY;
    }

    auto t2 = std::chrono::high_resolution_clock::now();
    double elapsed = std::chrono::duration<double>(t2 - t1).count();

    std::cout << "Result constant part: " << cons(R) << std::endl;
    std::cout << "CPUSEC: " << elapsed << std::endl;

    return 0;
}
