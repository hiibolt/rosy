#!/bin/bash
# ============================================================================
#  Rosy (release) vs Rosy (optimized) vs DACELIB — DA Benchmark Comparison
# ============================================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROSY_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
ROSY_BIN="$ROSY_ROOT/target/release/rosy"
DACE_BENCH_DIR="$ROSY_ROOT/.dacelib_comp/benchmarks/build"
NON_MPI_DIR="$ROSY_ROOT/examples/performance/non_mpi"
BUILD_DIR=$(mktemp -d /tmp/rosy_dace_bench_XXXXXX)
trap "rm -rf $BUILD_DIR" EXIT

echo ""
echo "================================================================"
echo "  Rosy (release) vs Rosy (optimized) vs DACELIB (-O3)"
echo "================================================================"
echo ""
echo "  Rosy:    $ROSY_BIN"
echo "  DACELIB: $DACE_BENCH_DIR"
echo "  Date:    $(date)"
echo "  Host:    $(hostname)"
echo "  CPU:     $(grep 'model name' /proc/cpuinfo 2>/dev/null | head -1 | sed 's/model name\s*:\s*//' || echo 'unknown')"
echo ""

declare -a BENCH_NAMES=(
    "02_da_multiply"
    "03_da_trig"
    "12_da_derivatives"
    "14_da_transfer_map"
    "15_da_high_order_multiply"
    "16_da_bending_magnet"
    "17_da_aberration"
)
declare -a BENCH_LABELS=(
    "DA Multiply"
    "DA Trig"
    "DA Derivatives"
    "DA Transfer Map"
    "DA High-Order Mul"
    "DA Bending Magnet"
    "DA Aberration"
)

run_timed_ms() {
    local cmd="$1"
    local start end ms
    start=$(date +%s%N)
    eval "$cmd" > /dev/null 2>&1 || true
    end=$(date +%s%N)
    ms=$(awk "BEGIN { printf \"%.1f\", ($end - $start) / 1000000 }")
    echo "$ms"
}

# ── Pre-build all Rosy binaries ──────────────────────────────────────────────
echo "Building Rosy binaries..."
for tier in t3 t4; do
    for name in "${BENCH_NAMES[@]}"; do
        rosy_file="$NON_MPI_DIR/$name/bench_${tier}.rosy"
        [[ -f "$rosy_file" ]] || continue
        # Release
        "$ROSY_BIN" build "$rosy_file" --release -d "$BUILD_DIR" -o "$BUILD_DIR/${name}_${tier}_release" 2>/dev/null || echo "  WARN: failed to build release $name $tier"
        # Optimized
        "$ROSY_BIN" build "$rosy_file" --optimized -d "$BUILD_DIR" -o "$BUILD_DIR/${name}_${tier}_optimized" 2>/dev/null || echo "  WARN: failed to build optimized $name $tier"
    done
done
echo "  Done building."
echo ""

# ── Run T3 ────────────────────────────────────────────────────────────────────
for TIER in T3 T4; do
    tier_lc=$(echo "$TIER" | tr 'A-Z' 'a-z')
    echo "================================================================"
    echo "  $TIER Results"
    echo "================================================================"
    printf "%-20s %10s %10s %10s %11s %11s\n" "Benchmark" "Rosy Rel" "Rosy Opt" "DACELIB" "Rel/DACE" "Opt/DACE"
    printf "%-20s %10s %10s %10s %11s %11s\n" "--------------------" "----------" "----------" "----------" "-----------" "-----------"

    TOTAL_REL=0; TOTAL_OPT=0; TOTAL_DACE=0

    for idx in "${!BENCH_NAMES[@]}"; do
        name="${BENCH_NAMES[$idx]}"
        label="${BENCH_LABELS[$idx]}"

        # Rosy release
        bin="$BUILD_DIR/${name}_${tier_lc}_release"
        if [[ -x "$bin" ]]; then
            r_rel=$(run_timed_ms "$bin")
        else
            r_rel="—"
        fi

        # Rosy optimized
        bin="$BUILD_DIR/${name}_${tier_lc}_optimized"
        if [[ -x "$bin" ]]; then
            r_opt=$(run_timed_ms "$bin")
        else
            r_opt="—"
        fi

        # DACELIB
        dace_bin="$DACE_BENCH_DIR/${name}_${tier_lc}"
        if [[ -x "$dace_bin" ]]; then
            d=$(run_timed_ms "$dace_bin")
        else
            d="—"
        fi

        # Ratios (DACE/Rosy — >1 means Rosy faster)
        if [[ "$r_rel" != "—" && "$d" != "—" ]]; then
            ratio_rel=$(awk "BEGIN { if ($r_rel > 0.01) printf \"%.2fx\", $d / $r_rel; else print \"INF\" }")
            TOTAL_REL=$(awk "BEGIN { print $TOTAL_REL + $r_rel }")
            TOTAL_DACE_REL=$(awk "BEGIN { print ${TOTAL_DACE:-0} + $d }")
        else
            ratio_rel="—"
        fi
        if [[ "$r_opt" != "—" && "$d" != "—" ]]; then
            ratio_opt=$(awk "BEGIN { if ($r_opt > 0.01) printf \"%.2fx\", $d / $r_opt; else print \"INF\" }")
            TOTAL_OPT=$(awk "BEGIN { print $TOTAL_OPT + $r_opt }")
            TOTAL_DACE=$(awk "BEGIN { print ${TOTAL_DACE:-0} + $d }")
        else
            ratio_opt="—"
        fi

        printf "%-20s %10s %10s %10s %11s %11s\n" "$label" "$r_rel" "$r_opt" "$d" "$ratio_rel" "$ratio_opt"
    done

    echo ""
done

echo "================================================================"
echo "  Ratio > 1.0x = Rosy faster than DACELIB"
echo "  Ratio < 1.0x = DACELIB faster than Rosy"
echo "================================================================"
