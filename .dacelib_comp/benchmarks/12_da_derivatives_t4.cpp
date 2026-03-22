// DACELIB Benchmark: DA derivative chains (T4)
// Matches: examples/performance/non_mpi/12_da_derivatives/bench_t4.rosy
// Order 10, 3 variables => C(13,3) = 286 monomials per DA vector
// 500,000 iterations

#include <iostream>
#include <chrono>
#include <dace/dace.h>

using namespace DACE;

int main() {
    DA::init(10, 3);

    DA X = DA(1);
    DA Y = DA(2);
    DA Z = DA(3);
    DA F, DFX, DFY, DFZ;

    std::cout << "DA Derivatives Benchmark (DACELIB): order 10, 3 vars, 500000 iterations" << std::endl;

    auto t1 = std::chrono::high_resolution_clock::now();

    for (int i = 0; i < 500000; i++) {
        // Build a complex polynomial
        F = X * X * Y + Y * Y * Z + Z * Z * X + X * Y * Z;
        F = F * F + X * X * X + Y * Y * Y + Z * Z * Z;

        // Take derivatives with respect to each variable
        DFX = F.deriv(1);
        DFY = F.deriv(2);
        DFZ = F.deriv(3);

        // Chain: second derivatives
        DFX = DFX.deriv(1);
        DFY = DFY.deriv(2);
        DFZ = DFZ.deriv(3);

        // Anti-derivative
        F = DFX.integ(1) + DFY.integ(2) + DFZ.integ(3);
    }

    auto t2 = std::chrono::high_resolution_clock::now();
    double elapsed = std::chrono::duration<double>(t2 - t1).count();

    std::cout << "Result constant part: " << cons(F) << std::endl;
    std::cout << "CPUSEC: " << elapsed << std::endl;

    return 0;
}
