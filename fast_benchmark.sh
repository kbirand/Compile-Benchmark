#!/bin/bash
# Fast Compile Benchmark Script for macOS/Linux - Endpoint Testing
# Uses a minimal Rust project (~10 seconds) to quickly test the full pipeline
# Usage: chmod +x fast_benchmark.sh && ./fast_benchmark.sh

set -e

echo "========================================"
echo "  Fast Compile Benchmark (Endpoint Test)"
echo "========================================"
echo ""

# Power monitoring variables
POWER_LOG_FILE="/tmp/power_benchmark_$$.log"
POWER_PID=""
POWER_AVAILABLE=false

# Check if we have sudo/root for power monitoring
check_power_monitoring() {
    if [[ "$OSTYPE" == "darwin"* ]]; then
        if command -v powermetrics &> /dev/null; then
            if [[ $EUID -eq 0 ]]; then
                POWER_AVAILABLE=true
                echo "Power monitoring: ENABLED (powermetrics)"
            else
                echo "Power monitoring: DISABLED (run with sudo for power monitoring)"
            fi
        else
            echo "Power monitoring: NOT AVAILABLE (powermetrics not found)"
        fi
    else
        if command -v turbostat &> /dev/null; then
            if [[ $EUID -eq 0 ]]; then
                POWER_AVAILABLE=true
                echo "Power monitoring: ENABLED (turbostat)"
            fi
        elif [[ -r /sys/class/powercap/intel-rapl/intel-rapl:0/energy_uj ]]; then
            if [[ $EUID -eq 0 ]] || [[ -r /sys/class/powercap/intel-rapl/intel-rapl:0/energy_uj ]]; then
                POWER_AVAILABLE=true
                echo "Power monitoring: ENABLED (RAPL)"
            fi
        else
            echo "Power monitoring: DISABLED"
        fi
    fi
    echo ""
}

start_power_monitoring() {
    if [[ "$POWER_AVAILABLE" != "true" ]]; then return; fi
    echo "Starting power monitoring..."
    if [[ "$OSTYPE" == "darwin"* ]]; then
        powermetrics --samplers cpu_power -i 1000 -n 0 2>/dev/null | while read line; do
            if [[ "$line" == *"CPU Power"* ]] || [[ "$line" == *"Package Power"* ]]; then
                echo "$(date +%s.%N) $line" >> "$POWER_LOG_FILE"
            fi
        done &
        POWER_PID=$!
    else
        if command -v turbostat &> /dev/null; then
            turbostat --quiet --show PkgWatt,CorWatt -i 1 2>/dev/null | while read line; do
                echo "$(date +%s.%N) $line" >> "$POWER_LOG_FILE"
            done &
            POWER_PID=$!
        else
            while true; do
                if [[ -r /sys/class/powercap/intel-rapl/intel-rapl:0/energy_uj ]]; then
                    energy=$(cat /sys/class/powercap/intel-rapl/intel-rapl:0/energy_uj)
                    echo "$(date +%s.%N) energy_uj: $energy" >> "$POWER_LOG_FILE"
                fi
                sleep 1
            done &
            POWER_PID=$!
        fi
    fi
    sleep 0.5
}

stop_power_monitoring() {
    if [[ -n "$POWER_PID" ]]; then
        kill $POWER_PID 2>/dev/null || true
        wait $POWER_PID 2>/dev/null || true
        POWER_PID=""
    fi
}

calculate_power_stats() {
    local log_file="$1"
    if [[ ! -f "$log_file" ]]; then echo "N/A"; return; fi
    if [[ "$OSTYPE" == "darwin"* ]]; then
        local powers=$(grep -E "CPU Power|Package Power" "$log_file" | grep -oE '[0-9]+\.?[0-9]* mW' | grep -oE '[0-9]+\.?[0-9]*')
        if [[ -n "$powers" ]]; then
            local sum=0 count=0
            for p in $powers; do sum=$(echo "$sum + $p" | bc); count=$((count + 1)); done
            if [[ $count -gt 0 ]]; then echo "$(echo "scale=2; $sum / $count / 1000" | bc)W"; return; fi
        fi
    else
        local powers=$(grep -oE 'PkgWatt[[:space:]]+[0-9]+\.?[0-9]*' "$log_file" | grep -oE '[0-9]+\.?[0-9]*$')
        if [[ -n "$powers" ]]; then
            local sum=0 count=0
            for p in $powers; do sum=$(echo "$sum + $p" | bc); count=$((count + 1)); done
            if [[ $count -gt 0 ]]; then echo "$(echo "scale=2; $sum / $count" | bc)W"; return; fi
        fi
        local first_energy=$(head -1 "$log_file" 2>/dev/null | grep -oE 'energy_uj: [0-9]+' | grep -oE '[0-9]+')
        local last_energy=$(tail -1 "$log_file" 2>/dev/null | grep -oE 'energy_uj: [0-9]+' | grep -oE '[0-9]+')
        local first_time=$(head -1 "$log_file" 2>/dev/null | cut -d' ' -f1)
        local last_time=$(tail -1 "$log_file" 2>/dev/null | cut -d' ' -f1)
        if [[ -n "$first_energy" ]] && [[ -n "$last_energy" ]] && [[ -n "$first_time" ]] && [[ -n "$last_time" ]]; then
            local energy_diff=$(echo "$last_energy - $first_energy" | bc)
            local time_diff=$(echo "$last_time - $first_time" | bc)
            if [[ $(echo "$time_diff > 0" | bc) -eq 1 ]]; then
                echo "$(echo "scale=2; $energy_diff / $time_diff / 1000000" | bc)W"; return
            fi
        fi
    fi
    echo "N/A"
}

cleanup() {
    stop_power_monitoring
    rm -f "$POWER_LOG_FILE" "/tmp/power_debug_$$.log" "/tmp/power_release_$$.log"
}
trap cleanup EXIT

# Helper to get CPU model on Linux
get_linux_cpu() {
    local cpu
    cpu=$(grep 'model name' /proc/cpuinfo 2>/dev/null | head -1 | cut -d':' -f2 | xargs)
    if [[ -z "$cpu" ]]; then cpu=$(lscpu 2>/dev/null | grep 'Model name' | cut -d':' -f2 | xargs); fi
    if [[ -z "$cpu" ]]; then cpu=$(uname -m); fi
    echo "$cpu"
}

check_power_monitoring

# Check if Rust is installed
check_rust_installed() {
    if [ -f "$HOME/.cargo/bin/rustc" ] && [[ ":$PATH:" != *":$HOME/.cargo/bin:"* ]]; then
        export PATH="$HOME/.cargo/bin:$PATH"
    fi
    if [ -f "$HOME/.cargo/env" ]; then source "$HOME/.cargo/env"; fi
    if command -v rustc &> /dev/null && command -v cargo &> /dev/null; then return 0; fi
    if [ -f "$HOME/.cargo/bin/rustc" ]; then export PATH="$HOME/.cargo/bin:$PATH"; return 0; fi
    return 1
}

if ! check_rust_installed; then
    echo "ERROR: Rust is not installed!"
    echo "Please install Rust: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo "Rust installation detected!"
echo ""

# Get system info
echo "System Information:"
if [[ "$OSTYPE" == "darwin"* ]]; then
    echo "  Device: $(system_profiler SPHardwareDataType | grep 'Model Name' | cut -d':' -f2 | xargs)"
    echo "  OS: macOS $(sw_vers -productVersion)"
    echo "  CPU: $(sysctl -n machdep.cpu.brand_string)"
    echo "  GPU: $(system_profiler SPDisplaysDataType | grep 'Chipset Model' | cut -d':' -f2 | xargs)"
    echo "  GPU Cores: $(system_profiler SPDisplaysDataType | grep 'Total Number of Cores' | cut -d':' -f2 | xargs)"
    echo "  Cores: $(sysctl -n hw.ncpu)"
    echo "  RAM: $(( $(sysctl -n hw.memsize) / 1073741824 )) GB"
else
    echo "  Device: $(cat /sys/devices/virtual/dmi/id/product_name 2>/dev/null || echo 'Unknown')"
    echo "  OS: $(cat /etc/os-release | grep PRETTY_NAME | cut -d'"' -f2)"
    echo "  CPU: $(get_linux_cpu)"
    echo "  GPU: $(lspci 2>/dev/null | grep -i 'vga\|3d\|display' | head -1 | cut -d':' -f3 | xargs || echo 'Unknown')"
    GPU_CORES_LINUX=""
    if command -v nvidia-smi &>/dev/null; then
        GPU_CORES_LINUX="$(nvidia-settings -q CUDACores -t 2>/dev/null | head -1)"
    elif [ -d /sys/class/drm/card0/device ]; then
        GPU_CORES_LINUX="$(cat /sys/class/drm/card0/device/pp_num_compute_units 2>/dev/null || echo '')"
    fi
    echo "  GPU Cores: ${GPU_CORES_LINUX:-N/A}"
    echo "  Cores: $(nproc)"
    echo "  RAM: $(( $(cat /proc/meminfo | grep MemTotal | awk '{print $2}') / 1048576 )) GB"
fi
echo ""

echo "Rust Version:"
rustc --version
cargo --version
echo ""

# Use the fast-bench sub-project
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FAST_DIR="$SCRIPT_DIR/fast-bench"

if [[ ! -f "$FAST_DIR/Cargo.toml" ]]; then
    echo "ERROR: fast-bench directory not found at $FAST_DIR"
    exit 1
fi

cd "$FAST_DIR"

# Clean previous build
echo "Cleaning previous build..."
cargo clean 2>/dev/null || true
echo ""

# Debug build
echo "Starting DEBUG build benchmark..."
echo "========================================"
DEBUG_POWER_LOG="/tmp/power_debug_$$.log"
rm -f "$DEBUG_POWER_LOG"
POWER_LOG_FILE="$DEBUG_POWER_LOG"
start_power_monitoring

DEBUG_START=$(date +%s.%N)
cargo build 2>&1 | grep -v -E "(future.*(version|incompat)|unused config key)"
DEBUG_END=$(date +%s.%N)

stop_power_monitoring
DEBUG_TIME=$(echo "$DEBUG_END - $DEBUG_START" | bc)
DEBUG_POWER=$(calculate_power_stats "$DEBUG_POWER_LOG")
DEBUG_ENERGY="N/A"
if [[ "$DEBUG_POWER" != "N/A" ]]; then
    power_val=$(echo "$DEBUG_POWER" | grep -oE '[0-9]+\.?[0-9]*')
    DEBUG_ENERGY=$(echo "scale=2; $power_val * $DEBUG_TIME / 3600" | bc)
    DEBUG_ENERGY="${DEBUG_ENERGY}Wh"
fi

echo ""
echo "DEBUG build completed in: ${DEBUG_TIME} seconds"
if [[ "$POWER_AVAILABLE" == "true" ]]; then
    echo "  Average Power: ${DEBUG_POWER}"
    echo "  Energy Used:   ${DEBUG_ENERGY}"
fi
echo ""

# Clean for release
echo "Cleaning for release build..."
cargo clean
echo ""

# Release build
echo "Starting RELEASE build benchmark..."
echo "========================================"
RELEASE_POWER_LOG="/tmp/power_release_$$.log"
rm -f "$RELEASE_POWER_LOG"
POWER_LOG_FILE="$RELEASE_POWER_LOG"
start_power_monitoring

RELEASE_START=$(date +%s.%N)
cargo build --release 2>&1 | grep -v -E "(future.*(version|incompat)|unused config key)"
RELEASE_END=$(date +%s.%N)

stop_power_monitoring
RELEASE_TIME=$(echo "$RELEASE_END - $RELEASE_START" | bc)
RELEASE_POWER=$(calculate_power_stats "$RELEASE_POWER_LOG")
RELEASE_ENERGY="N/A"
if [[ "$RELEASE_POWER" != "N/A" ]]; then
    power_val=$(echo "$RELEASE_POWER" | grep -oE '[0-9]+\.?[0-9]*')
    RELEASE_ENERGY=$(echo "scale=2; $power_val * $RELEASE_TIME / 3600" | bc)
    RELEASE_ENERGY="${RELEASE_ENERGY}Wh"
fi

echo ""
echo "RELEASE build completed in: ${RELEASE_TIME} seconds"
if [[ "$POWER_AVAILABLE" == "true" ]]; then
    echo "  Average Power: ${RELEASE_POWER}"
    echo "  Energy Used:   ${RELEASE_ENERGY}"
fi
echo ""

# Summary
echo "========================================"
echo "  FAST BENCHMARK RESULTS (Endpoint Test)"
echo "========================================"
echo ""
echo "  Debug Build:"
echo "    Time:   ${DEBUG_TIME} seconds"
if [[ "$POWER_AVAILABLE" == "true" ]]; then
    echo "    Power:  ${DEBUG_POWER} (avg)"
    echo "    Energy: ${DEBUG_ENERGY}"
fi
echo ""
echo "  Release Build:"
echo "    Time:   ${RELEASE_TIME} seconds"
if [[ "$POWER_AVAILABLE" == "true" ]]; then
    echo "    Power:  ${RELEASE_POWER} (avg)"
    echo "    Energy: ${RELEASE_ENERGY}"
fi
echo ""
echo "  Date: $(date '+%Y-%m-%d %H:%M:%S')"
echo "========================================"

# Save results to file
cd "$SCRIPT_DIR"
cat > fast-benchmark-results.txt << EOF
Fast Compile Benchmark Results (Endpoint Test)
===============================================
Date: $(date '+%Y-%m-%d %H:%M:%S')

System:
$(if [[ "$OSTYPE" == "darwin"* ]]; then
    echo "  Device: $(system_profiler SPHardwareDataType | grep 'Model Name' | cut -d':' -f2 | xargs)"
    echo "  OS: macOS $(sw_vers -productVersion)"
    echo "  CPU: $(sysctl -n machdep.cpu.brand_string)"
    echo "  GPU: $(system_profiler SPDisplaysDataType | grep 'Chipset Model' | cut -d':' -f2 | xargs)"
    echo "  GPU Cores: $(system_profiler SPDisplaysDataType | grep 'Total Number of Cores' | cut -d':' -f2 | xargs)"
    echo "  Cores: $(sysctl -n hw.ncpu)"
    echo "  RAM: $(( $(sysctl -n hw.memsize) / 1073741824 )) GB"
else
    echo "  Device: $(cat /sys/devices/virtual/dmi/id/product_name 2>/dev/null || echo 'Unknown')"
    echo "  OS: $(cat /etc/os-release | grep PRETTY_NAME | cut -d'"' -f2)"
    echo "  CPU: $(get_linux_cpu)"
    echo "  GPU: $(lspci 2>/dev/null | grep -i 'vga\|3d\|display' | head -1 | cut -d':' -f3 | xargs || echo 'Unknown')"
    GPU_CORES_TXT=""
    if command -v nvidia-smi &>/dev/null; then
        GPU_CORES_TXT="$(nvidia-settings -q CUDACores -t 2>/dev/null | head -1)"
    elif [ -d /sys/class/drm/card0/device ]; then
        GPU_CORES_TXT="$(cat /sys/class/drm/card0/device/pp_num_compute_units 2>/dev/null || echo '')"
    fi
    echo "  GPU Cores: ${GPU_CORES_TXT:-N/A}"
    echo "  Cores: $(nproc)"
    echo "  RAM: $(( $(cat /proc/meminfo | grep MemTotal | awk '{print $2}') / 1048576 )) GB"
fi)

Rust:
  $(rustc --version)
  $(cargo --version)

Results:
  Debug Build:
    Time:   ${DEBUG_TIME} seconds
    Power:  ${DEBUG_POWER} (avg)
    Energy: ${DEBUG_ENERGY}
    
  Release Build:
    Time:   ${RELEASE_TIME} seconds
    Power:  ${RELEASE_POWER} (avg)
    Energy: ${RELEASE_ENERGY}

Power Monitoring: $(if [[ "$POWER_AVAILABLE" == "true" ]]; then echo "Enabled"; else echo "Disabled (run with sudo)"; fi)
EOF

echo "Results saved to fast-benchmark-results.txt"

# Send results to endpoint
BENCHMARK_API_URL="$(head -1 "$SCRIPT_DIR/benchmark-config.cfg" | tr -d '[:space:]')"

send_results_to_endpoint() {
    echo ""
    echo "Sending results to ${BENCHMARK_API_URL}..."

    if [[ "$OSTYPE" == "darwin"* ]]; then
        SYS_DEVICE="$(system_profiler SPHardwareDataType | grep 'Model Name' | cut -d':' -f2 | xargs)"
        SYS_OS="macOS $(sw_vers -productVersion)"
        SYS_CPU="$(sysctl -n machdep.cpu.brand_string)"
        SYS_GPU="$(system_profiler SPDisplaysDataType | grep 'Chipset Model' | cut -d':' -f2 | xargs)"
        SYS_GPU_CORES="$(system_profiler SPDisplaysDataType | grep 'Total Number of Cores' | cut -d':' -f2 | xargs)"
        SYS_CORES="$(sysctl -n hw.ncpu)"
        SYS_RAM="$(( $(sysctl -n hw.memsize) / 1073741824 ))"
    else
        SYS_DEVICE="$(cat /sys/devices/virtual/dmi/id/product_name 2>/dev/null || echo 'Unknown')"
        SYS_OS="$(cat /etc/os-release | grep PRETTY_NAME | cut -d'"' -f2)"
        SYS_CPU="$(get_linux_cpu)"
        SYS_GPU="$(lspci 2>/dev/null | grep -i 'vga\|3d\|display' | head -1 | cut -d':' -f3 | xargs || echo 'Unknown')"
        SYS_GPU_CORES=""
        if command -v nvidia-smi &>/dev/null; then
            SYS_GPU_CORES="$(nvidia-settings -q CUDACores -t 2>/dev/null | head -1)"
        elif [ -d /sys/class/drm/card0/device ]; then
            SYS_GPU_CORES="$(cat /sys/class/drm/card0/device/pp_num_compute_units 2>/dev/null || echo '')"
        fi
        SYS_CORES="$(nproc)"
        SYS_RAM="$(( $(cat /proc/meminfo | grep MemTotal | awk '{print $2}') / 1048576 ))"
    fi

    JSON_PAYLOAD=$(cat <<EOJSON
{
  "timestamp": "$(date -u '+%Y-%m-%dT%H:%M:%SZ')",
  "system": {
    "device": "${SYS_DEVICE}",
    "os": "${SYS_OS}",
    "cpu": "${SYS_CPU}",
    "gpu": "${SYS_GPU}",
    "gpu_cores": "${SYS_GPU_CORES:-N/A}",
    "cores": ${SYS_CORES},
    "ram_gb": ${SYS_RAM}
  },
  "rust": {
    "rustc": "$(rustc --version)",
    "cargo": "$(cargo --version)"
  },
  "results": {
    "debug": {
      "time_seconds": ${DEBUG_TIME},
      "avg_power": "${DEBUG_POWER}",
      "energy": "${DEBUG_ENERGY}"
    },
    "release": {
      "time_seconds": ${RELEASE_TIME},
      "avg_power": "${RELEASE_POWER}",
      "energy": "${RELEASE_ENERGY}"
    }
  },
  "power_monitoring_enabled": $(if [[ "$POWER_AVAILABLE" == "true" ]]; then echo "true"; else echo "false"; fi),
  "benchmark_type": "fast"
}
EOJSON
)

    echo "$JSON_PAYLOAD" > fast-benchmark-results.json
    echo "Results saved to fast-benchmark-results.json"

    HTTP_STATUS=$(curl -s -o /tmp/benchmark_response_$$.txt -w "%{http_code}" \
        -X POST \
        -H "Content-Type: application/json" \
        -d "$JSON_PAYLOAD" \
        --connect-timeout 10 \
        --max-time 30 \
        "$BENCHMARK_API_URL" 2>&1) || true

    if [[ "$HTTP_STATUS" =~ ^2[0-9][0-9]$ ]]; then
        echo "Results sent successfully! (HTTP ${HTTP_STATUS})"
    elif [[ -z "$HTTP_STATUS" || "$HTTP_STATUS" == "000" ]]; then
        echo "WARNING: Could not connect to ${BENCHMARK_API_URL}"
    else
        echo "WARNING: Server responded with HTTP ${HTTP_STATUS}"
        if [[ -f /tmp/benchmark_response_$$.txt ]]; then
            echo "  Response: $(cat /tmp/benchmark_response_$$.txt)"
        fi
    fi
    rm -f /tmp/benchmark_response_$$.txt
}

send_results_to_endpoint
