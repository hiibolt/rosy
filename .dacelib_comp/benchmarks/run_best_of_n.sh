#!/bin/bash
# ============================================================================
#  Rosy (release) vs Rosy (optimized) vs DACELIB (-O3) — Best of N
# ============================================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROSY_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
ROSY_BIN="$ROSY_ROOT/target/release/rosy"
DACE_BENCH_DIR="$ROSY_ROOT/.dacelib_comp/benchmarks/build"
NON_MPI_DIR="$ROSY_ROOT/examples/performance/non_mpi"
BUILD_DIR=$(mktemp -d /tmp/rosy_dace_bench_XXXXXX)
RUNS="${1:-50}"
trap "rm -rf $BUILD_DIR" EXIT

echo ""
echo "================================================================"
echo "  Rosy (release) vs Rosy (optimized) vs DACELIB (-O3)"
echo "  Best-of-$RUNS runs"
echo "================================================================"
echo ""
echo "  Rosy:    $ROSY_BIN ($($ROSY_BIN --version 2>/dev/null || echo 'unknown'))"
echo "  DACELIB: DACE 2.1.0, Clang 21.1.8 -O3"
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

# Run command and return wall-clock ms
run_timed_ms() {
    local cmd="$1"
    local start end ms
    start=$(date +%s%N)
    eval "$cmd" > /dev/null 2>&1 || true
    end=$(date +%s%N)
    ms=$(awk "BEGIN { printf \"%.1f\", ($end - $start) / 1000000 }")
    echo "$ms"
}

# Run N times, return minimum
best_of_n() {
    local cmd="$1"
    local n="$2"
    local best=""
    for ((r=1; r<=n; r++)); do
        local ms
        ms=$(run_timed_ms "$cmd")
        if [[ -z "$best" ]]; then
            best="$ms"
        else
            best=$(awk "BEGIN { if ($ms < $best) print $ms; else print $best }")
        fi
    done
    echo "$best"
}

# ── Pre-build all Rosy binaries ──────────────────────────────────────────────
echo "Building Rosy binaries..."
for tier in t3 t4; do
    for name in "${BENCH_NAMES[@]}"; do
        rosy_file="$NON_MPI_DIR/$name/bench_${tier}.rosy"
        [[ -f "$rosy_file" ]] || continue
        "$ROSY_BIN" build "$rosy_file" --release -d "$BUILD_DIR" -o "$BUILD_DIR/${name}_${tier}_release" 2>/dev/null || true
        "$ROSY_BIN" build "$rosy_file" --optimized -d "$BUILD_DIR" -o "$BUILD_DIR/${name}_${tier}_optimized" 2>/dev/null || true
    done
done
echo "  Done."
echo ""

# ── Run both tiers ────────────────────────────────────────────────────────────
for TIER in T3 T4; do
    tier_lc=$(echo "$TIER" | tr 'A-Z' 'a-z')
    echo "================================================================"
    CURRENT_RUNS=$RUNS
    echo "  $TIER Results (best of $CURRENT_RUNS)"
    echo "================================================================"
    printf "%-20s %10s %10s %10s %11s %11s\n" "Benchmark" "Rosy Rel" "Rosy Opt" "DACELIB" "Rel/DACE" "Opt/DACE"
    printf "%-20s %10s %10s %10s %11s %11s\n" "--------------------" "----------" "----------" "----------" "-----------" "-----------"

    TOTAL_REL=0; TOTAL_OPT=0; TOTAL_DACE=0
    ROSY_REL_WINS=0; ROSY_OPT_WINS=0; DACE_WINS_REL=0; DACE_WINS_OPT=0

    for idx in "${!BENCH_NAMES[@]}"; do
        name="${BENCH_NAMES[$idx]}"
        label="${BENCH_LABELS[$idx]}"

        printf "  Running %s %s...\r" "$label" "$TIER" >&2

        # Rosy release
        bin="$BUILD_DIR/${name}_${tier_lc}_release"
        if [[ -x "$bin" ]]; then r_rel=$(best_of_n "$bin" $RUNS); else r_rel="—"; fi

        # Rosy optimized
        bin="$BUILD_DIR/${name}_${tier_lc}_optimized"
        if [[ -x "$bin" ]]; then r_opt=$(best_of_n "$bin" $RUNS); else r_opt="—"; fi

        # DACELIB
        dace_bin="$DACE_BENCH_DIR/${name}_${tier_lc}"
        if [[ -x "$dace_bin" ]]; then d=$(best_of_n "$dace_bin" $RUNS); else d="—"; fi

        # Ratios
        ratio_rel="—"; ratio_opt="—"
        if [[ "$r_rel" != "—" && "$d" != "—" ]]; then
            ratio_rel=$(awk "BEGIN { if ($r_rel > 0.01) printf \"%.2fx\", $d / $r_rel; else print \"INF\" }")
            TOTAL_REL=$(awk "BEGIN { printf \"%.1f\", $TOTAL_REL + $r_rel }")
            TOTAL_DACE=$(awk "BEGIN { printf \"%.1f\", $TOTAL_DACE + $d }")
            w=$(awk "BEGIN { v=$d/$r_rel; if (v > 1.05) print 1; else print 0 }")
            ROSY_REL_WINS=$((ROSY_REL_WINS + w))
            w2=$(awk "BEGIN { v=$d/$r_rel; if (v < 0.95) print 1; else print 0 }")
            DACE_WINS_REL=$((DACE_WINS_REL + w2))
        fi
        if [[ "$r_opt" != "—" && "$d" != "—" ]]; then
            ratio_opt=$(awk "BEGIN { if ($r_opt > 0.01) printf \"%.2fx\", $d / $r_opt; else print \"INF\" }")
            TOTAL_OPT=$(awk "BEGIN { printf \"%.1f\", $TOTAL_OPT + $r_opt }")
            w=$(awk "BEGIN { v=$d/$r_opt; if (v > 1.05) print 1; else print 0 }")
            ROSY_OPT_WINS=$((ROSY_OPT_WINS + w))
            w2=$(awk "BEGIN { v=$d/$r_opt; if (v < 0.95) print 1; else print 0 }")
            DACE_WINS_OPT=$((DACE_WINS_OPT + w2))
        fi

        printf "\r%80s\r" "" >&2
        printf "%-20s %10s %10s %10s %11s %11s\n" "$label" "$r_rel" "$r_opt" "$d" "$ratio_rel" "$ratio_opt"
    done

    # Totals
    printf "%-20s %10s %10s %10s %11s %11s\n" "--------------------" "----------" "----------" "----------" "-----------" "-----------"
    total_ratio_rel=$(awk "BEGIN { if ($TOTAL_REL > 0.01) printf \"%.2fx\", $TOTAL_DACE / $TOTAL_REL; else print \"INF\" }")
    total_ratio_opt=$(awk "BEGIN { if ($TOTAL_OPT > 0.01) printf \"%.2fx\", $TOTAL_DACE / $TOTAL_OPT; else print \"INF\" }")
    printf "%-20s %10s %10s %10s %11s %11s\n" "TOTAL" "$TOTAL_REL" "$TOTAL_OPT" "$TOTAL_DACE" "$total_ratio_rel" "$total_ratio_opt"
    echo ""
    TIES_REL=$((7 - ROSY_REL_WINS - DACE_WINS_REL))
    TIES_OPT=$((7 - ROSY_OPT_WINS - DACE_WINS_OPT))
    echo "  Release:   Rosy wins $ROSY_REL_WINS, DACELIB wins $DACE_WINS_REL, ties $TIES_REL"
    echo "  Optimized: Rosy wins $ROSY_OPT_WINS, DACELIB wins $DACE_WINS_OPT, ties $TIES_OPT"
    echo ""
done

echo "================================================================"
echo "  Ratio > 1.0x = Rosy faster   |  Ratio < 1.0x = DACELIB faster"
echo "================================================================"
