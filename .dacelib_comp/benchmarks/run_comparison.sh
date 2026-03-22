#!/bin/bash
# ============================================================================
#  Rosy vs DACELIB DA Performance Comparison
# ============================================================================
#
#  Runs the 7 DA benchmarks at T3 and T4 tiers for both Rosy and DACELIB,
#  then prints a comparison table.
#
#  Usage:
#    ./run_dacelib_comparison.sh
#

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROSY_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
ROSY_BIN="$ROSY_ROOT/target/release/rosy"
DACE_BENCH_DIR="$ROSY_ROOT/.dacelib_comp/benchmarks/build"
NON_MPI_DIR="$ROSY_ROOT/examples/performance/non_mpi"
BUILD_DIR=$(mktemp -d /tmp/rosy_dace_bench_XXXXXX)
trap "rm -rf $BUILD_DIR" EXIT

# Verify tools exist
if [[ ! -x "$ROSY_BIN" ]]; then
    echo "ERROR: Rosy binary not found at '$ROSY_BIN'"
    echo "Run: cargo build --release -p rosy"
    exit 1
fi

if [[ ! -d "$DACE_BENCH_DIR" ]]; then
    echo "ERROR: DACELIB benchmarks not built at '$DACE_BENCH_DIR'"
    exit 1
fi

echo ""
echo "================================================================"
echo "     Rosy vs DACELIB — DA Benchmark Comparison"
echo "================================================================"
echo ""
echo "  Rosy:    $ROSY_BIN"
echo "  DACELIB: $DACE_BENCH_DIR"
echo "  Date:    $(date)"
echo "  Host:    $(hostname)"
echo "  CPU:     $(grep 'model name' /proc/cpuinfo 2>/dev/null | head -1 | sed 's/model name\s*:\s*//' || echo 'unknown')"
echo ""

# ── Benchmark definitions ─────────────────────────────────────────────────────
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
    "DA High-Order Multiply"
    "DA Bending Magnet"
    "DA Aberration"
)

# ── Results arrays ────────────────────────────────────────────────────────────
declare -A ROSY_T3 ROSY_T4 DACE_T3 DACE_T4

# ── Run a single benchmark and return wall-clock ms ───────────────────────────
run_timed_ms() {
    local cmd="$1"
    local start end ms
    start=$(date +%s%N)
    eval "$cmd" > /dev/null 2>&1 || true
    end=$(date +%s%N)
    ms=$(awk "BEGIN { printf \"%.1f\", ($end - $start) / 1000000 }")
    echo "$ms"
}

# ── Run benchmarks ────────────────────────────────────────────────────────────
echo "Running benchmarks (each tier runs once — best-of-1 for speed)..."
echo ""

for idx in "${!BENCH_NAMES[@]}"; do
    name="${BENCH_NAMES[$idx]}"
    label="${BENCH_LABELS[$idx]}"
    rosy_dir="$NON_MPI_DIR/$name"

    echo "  [$((idx+1))/7] $label..."

    # ── Rosy T3 ───────────────────────────────────────────────────────────
    rosy_file="$rosy_dir/bench_t3.rosy"
    if [[ -f "$rosy_file" ]]; then
        "$ROSY_BIN" build "$rosy_file" --release -d "$BUILD_DIR" -o "$BUILD_DIR/bench_rosy" 2>/dev/null
        ROSY_T3[$name]=$(run_timed_ms "$BUILD_DIR/bench_rosy")
        rm -f "$BUILD_DIR/bench_rosy"
    else
        ROSY_T3[$name]="—"
    fi

    # ── Rosy T4 ───────────────────────────────────────────────────────────
    rosy_file="$rosy_dir/bench_t4.rosy"
    if [[ -f "$rosy_file" ]]; then
        "$ROSY_BIN" build "$rosy_file" --release -d "$BUILD_DIR" -o "$BUILD_DIR/bench_rosy" 2>/dev/null
        ROSY_T4[$name]=$(run_timed_ms "$BUILD_DIR/bench_rosy")
        rm -f "$BUILD_DIR/bench_rosy"
    else
        ROSY_T4[$name]="—"
    fi

    # ── DACELIB T3 ────────────────────────────────────────────────────────
    dace_bin="$DACE_BENCH_DIR/${name}_t3"
    if [[ -x "$dace_bin" ]]; then
        DACE_T3[$name]=$(run_timed_ms "$dace_bin")
    else
        DACE_T3[$name]="—"
    fi

    # ── DACELIB T4 ────────────────────────────────────────────────────────
    dace_bin="$DACE_BENCH_DIR/${name}_t4"
    if [[ -x "$dace_bin" ]]; then
        DACE_T4[$name]=$(run_timed_ms "$dace_bin")
    else
        DACE_T4[$name]="—"
    fi
done

# ── Print results ─────────────────────────────────────────────────────────────
echo ""
echo "================================================================"
echo "  T3 Results (Representative Workload)"
echo "================================================================"
printf "%-26s %11s %11s %10s\n" "Benchmark" "Rosy (ms)" "DACE (ms)" "Ratio"
printf "%-26s %11s %11s %10s\n" "--------------------------" "-----------" "-----------" "----------"

for idx in "${!BENCH_NAMES[@]}"; do
    name="${BENCH_NAMES[$idx]}"
    label="${BENCH_LABELS[$idx]}"
    r="${ROSY_T3[$name]}"
    d="${DACE_T3[$name]}"
    if [[ "$r" != "—" && "$d" != "—" ]]; then
        ratio=$(awk "BEGIN { if ($r > 0.01) printf \"%.2fx\", $d / $r; else print \"INF\" }")
    else
        ratio="—"
    fi
    printf "%-26s %11s %11s %10s\n" "$label" "$r" "$d" "$ratio"
done

echo ""
echo "================================================================"
echo "  T4 Results (Stress Test)"
echo "================================================================"
printf "%-26s %11s %11s %10s\n" "Benchmark" "Rosy (ms)" "DACE (ms)" "Ratio"
printf "%-26s %11s %11s %10s\n" "--------------------------" "-----------" "-----------" "----------"

TOTAL_ROSY=0
TOTAL_DACE=0
ROSY_WINS=0
DACE_WINS=0
TIES=0

for idx in "${!BENCH_NAMES[@]}"; do
    name="${BENCH_NAMES[$idx]}"
    label="${BENCH_LABELS[$idx]}"
    r="${ROSY_T4[$name]}"
    d="${DACE_T4[$name]}"
    if [[ "$r" != "—" && "$d" != "—" ]]; then
        ratio=$(awk "BEGIN { if ($r > 0.01) printf \"%.2fx\", $d / $r; else print \"INF\" }")
        TOTAL_ROSY=$(awk "BEGIN { print $TOTAL_ROSY + $r }")
        TOTAL_DACE=$(awk "BEGIN { print $TOTAL_DACE + $d }")
        winner_val=$(awk "BEGIN { v=$d/$r; if (v > 1.05) print \"rosy\"; else if (v < 0.95) print \"dace\"; else print \"tie\" }")
        if [[ "$winner_val" == "rosy" ]]; then ROSY_WINS=$((ROSY_WINS+1)); fi
        if [[ "$winner_val" == "dace" ]]; then DACE_WINS=$((DACE_WINS+1)); fi
        if [[ "$winner_val" == "tie" ]]; then TIES=$((TIES+1)); fi
    else
        ratio="—"
    fi
    printf "%-26s %11s %11s %10s\n" "$label" "$r" "$d" "$ratio"
done

echo ""
printf "%-26s %11.1f %11.1f" "TOTAL" "$TOTAL_ROSY" "$TOTAL_DACE"
TOTAL_RATIO=$(awk "BEGIN { if ($TOTAL_ROSY > 0.01) printf \"%.2fx\", $TOTAL_DACE / $TOTAL_ROSY; else print \"INF\" }")
printf " %10s\n" "$TOTAL_RATIO"

echo ""
echo "  Scorecard: Rosy wins $ROSY_WINS, DACELIB wins $DACE_WINS, ties $TIES"
echo "  (Ratio > 1.0x = Rosy faster, < 1.0x = DACELIB faster)"
echo "================================================================"
