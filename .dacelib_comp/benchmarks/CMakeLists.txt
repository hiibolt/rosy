cmake_minimum_required(VERSION 3.10)
project(DACEBenchmarks CXX)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_BUILD_TYPE Release)

# Find the DACE library we just built
set(DACE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/../dace/build")
set(DACE_SRC "${CMAKE_CURRENT_SOURCE_DIR}/../dace")

# Include directories
include_directories(
    ${DACE_SRC}/interfaces/cxx/include
    ${DACE_SRC}/core/include
    ${DACE_DIR}/core/include    # for generated config.h
)

# Link the static library
link_directories(${DACE_DIR})

# Benchmark executables
set(BENCHMARKS
    02_da_multiply_t3
    02_da_multiply_t4
    03_da_trig_t3
    03_da_trig_t4
    12_da_derivatives_t3
    12_da_derivatives_t4
    14_da_transfer_map_t3
    14_da_transfer_map_t4
    15_da_high_order_multiply_t3
    15_da_high_order_multiply_t4
    16_da_bending_magnet_t3
    16_da_bending_magnet_t4
    17_da_aberration_t3
    17_da_aberration_t4
)

foreach(bench ${BENCHMARKS})
    add_executable(${bench} ${bench}.cpp)
    target_link_libraries(${bench} ${DACE_DIR}/libdace_s.a)
endforeach()
