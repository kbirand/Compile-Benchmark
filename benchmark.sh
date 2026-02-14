#!/bin/bash
# Compile Benchmark Script for macOS/Linux with Power Monitoring
# Usage: chmod +x benchmark.sh && ./benchmark.sh
# For power monitoring: sudo ./benchmark.sh

set -e

echo "========================================"
echo "  Rust Compile Time Benchmark"
echo "  (with Power Monitoring)"
echo "========================================"
echo ""

# Power monitoring variables
POWER_LOG_FILE="/tmp/power_benchmark_$$.log"
POWER_PID=""
POWER_AVAILABLE=false

# Check if we have sudo/root for power monitoring
check_power_monitoring() {
    if [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS - check for powermetrics
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
        # Linux - check for turbostat or perf
        if command -v turbostat &> /dev/null; then
            if [[ $EUID -eq 0 ]]; then
                POWER_AVAILABLE=true
                echo "Power monitoring: ENABLED (turbostat)"
            else
                echo "Power monitoring: DISABLED (run with sudo for power monitoring)"
            fi
        elif [[ -r /sys/class/powercap/intel-rapl/intel-rapl:0/energy_uj ]]; then
            if [[ $EUID -eq 0 ]] || [[ -r /sys/class/powercap/intel-rapl/intel-rapl:0/energy_uj ]]; then
                POWER_AVAILABLE=true
                echo "Power monitoring: ENABLED (RAPL)"
            fi
        else
            echo "Power monitoring: NOT AVAILABLE (install turbostat or check RAPL)"
        fi
    fi
    echo ""
}

# Start power monitoring in background
start_power_monitoring() {
    if [[ "$POWER_AVAILABLE" != "true" ]]; then
        return
    fi
    
    echo "Starting power monitoring..."
    
    if [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS powermetrics - sample every 1 second
        powermetrics --samplers cpu_power -i 1000 -n 0 2>/dev/null | while read line; do
            if [[ "$line" == *"CPU Power"* ]] || [[ "$line" == *"Package Power"* ]]; then
                echo "$(date +%s.%N) $line" >> "$POWER_LOG_FILE"
            fi
        done &
        POWER_PID=$!
    else
        # Linux - try turbostat first
        if command -v turbostat &> /dev/null; then
            turbostat --quiet --show PkgWatt,CorWatt -i 1 2>/dev/null | while read line; do
                echo "$(date +%s.%N) $line" >> "$POWER_LOG_FILE"
            done &
            POWER_PID=$!
        else
            # Fallback to RAPL direct reading
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
    
    # Give it a moment to start
    sleep 0.5
}

# Stop power monitoring
stop_power_monitoring() {
    if [[ -n "$POWER_PID" ]]; then
        kill $POWER_PID 2>/dev/null || true
        wait $POWER_PID 2>/dev/null || true
        POWER_PID=""
    fi
}

# Calculate average power from log
calculate_power_stats() {
    local log_file="$1"
    local start_time="$2"
    local end_time="$3"
    
    if [[ ! -f "$log_file" ]]; then
        echo "N/A"
        return
    fi
    
    if [[ "$OSTYPE" == "darwin"* ]]; then
        # Parse macOS powermetrics output
        local powers=$(grep -E "CPU Power|Package Power" "$log_file" | grep -oE '[0-9]+\.?[0-9]* mW' | grep -oE '[0-9]+\.?[0-9]*')
        if [[ -n "$powers" ]]; then
            local sum=0
            local count=0
            for p in $powers; do
                sum=$(echo "$sum + $p" | bc)
                count=$((count + 1))
            done
            if [[ $count -gt 0 ]]; then
                local avg=$(echo "scale=2; $sum / $count / 1000" | bc)
                echo "${avg}W"
                return
            fi
        fi
    else
        # Parse Linux turbostat/RAPL output
        local powers=$(grep -oE 'PkgWatt[[:space:]]+[0-9]+\.?[0-9]*' "$log_file" | grep -oE '[0-9]+\.?[0-9]*$')
        if [[ -n "$powers" ]]; then
            local sum=0
            local count=0
            for p in $powers; do
                sum=$(echo "$sum + $p" | bc)
                count=$((count + 1))
            done
            if [[ $count -gt 0 ]]; then
                local avg=$(echo "scale=2; $sum / $count" | bc)
                echo "${avg}W"
                return
            fi
        fi
        
        # Try RAPL energy calculation
        local first_energy=$(head -1 "$log_file" 2>/dev/null | grep -oE 'energy_uj: [0-9]+' | grep -oE '[0-9]+')
        local last_energy=$(tail -1 "$log_file" 2>/dev/null | grep -oE 'energy_uj: [0-9]+' | grep -oE '[0-9]+')
        local first_time=$(head -1 "$log_file" 2>/dev/null | cut -d' ' -f1)
        local last_time=$(tail -1 "$log_file" 2>/dev/null | cut -d' ' -f1)
        
        if [[ -n "$first_energy" ]] && [[ -n "$last_energy" ]] && [[ -n "$first_time" ]] && [[ -n "$last_time" ]]; then
            local energy_diff=$(echo "$last_energy - $first_energy" | bc)
            local time_diff=$(echo "$last_time - $first_time" | bc)
            if [[ $(echo "$time_diff > 0" | bc) -eq 1 ]]; then
                local avg_power=$(echo "scale=2; $energy_diff / $time_diff / 1000000" | bc)
                echo "${avg_power}W"
                return
            fi
        fi
    fi
    
    echo "N/A"
}

# Cleanup on exit
cleanup() {
    stop_power_monitoring
    rm -f "$POWER_LOG_FILE" "/tmp/power_debug_$$.log" "/tmp/power_release_$$.log"
}
trap cleanup EXIT

# Check power monitoring availability
check_power_monitoring

# Check if Rust is installed (also check common install location)
check_rust_installed() {
    # First, try to add cargo to PATH if it exists but isn't in PATH
    if [ -f "$HOME/.cargo/bin/rustc" ] && [[ ":$PATH:" != *":$HOME/.cargo/bin:"* ]]; then
        export PATH="$HOME/.cargo/bin:$PATH"
    fi
    
    # Source cargo env if it exists
    if [ -f "$HOME/.cargo/env" ]; then
        source "$HOME/.cargo/env"
    fi
    
    if command -v rustc &> /dev/null && command -v cargo &> /dev/null; then
        return 0
    else
        # Also check if rustc exists in the default location
        if [ -f "$HOME/.cargo/bin/rustc" ]; then
            export PATH="$HOME/.cargo/bin:$PATH"
            return 0
        fi
        return 1
    fi
}

install_rust() {
    echo "Installing Rust via rustup..."
    echo ""
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    
    # Source the cargo environment
    if [ -f "$HOME/.cargo/env" ]; then
        source "$HOME/.cargo/env"
    fi
    
    echo ""
    echo "Rust installation complete!"
    echo ""
}

if ! check_rust_installed; then
    echo "ERROR: Rust is not installed!"
    echo ""
    echo "Rust is required to run this benchmark."
    echo ""
    echo "To install Rust manually:"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo ""
    
    read -p "Would you like to install Rust now? (Y/N): " install_choice
    
    if [[ "$install_choice" =~ ^[Yy]$ ]]; then
        install_rust
        
        # Verify installation
        if ! check_rust_installed; then
            echo ""
            echo "Rust installation may require a terminal restart."
            echo "Please restart your terminal and run this script again."
            exit 1
        fi
    else
        echo ""
        echo "Please install Rust and run this script again."
        echo "Visit: https://rustup.rs/"
        exit 1
    fi
fi

echo "Rust installation detected!"
echo ""

# Get system info
echo "System Information:"
if [[ "$OSTYPE" == "darwin"* ]]; then
    echo "  OS: macOS $(sw_vers -productVersion)"
    echo "  CPU: $(sysctl -n machdep.cpu.brand_string)"
    echo "  Cores: $(sysctl -n hw.ncpu)"
    echo "  RAM: $(( $(sysctl -n hw.memsize) / 1073741824 )) GB"
else
    echo "  OS: $(cat /etc/os-release | grep PRETTY_NAME | cut -d'"' -f2)"
    echo "  CPU: $(cat /proc/cpuinfo | grep 'model name' | head -1 | cut -d':' -f2 | xargs)"
    echo "  Cores: $(nproc)"
    echo "  RAM: $(( $(cat /proc/meminfo | grep MemTotal | awk '{print $2}') / 1048576 )) GB"
fi
echo ""

# Check Rust installation
echo "Rust Version:"
rustc --version
cargo --version
echo ""

# Clean previous build
echo "Cleaning previous build..."
cargo clean 2>/dev/null || true
echo ""

# Run debug build benchmark with power monitoring
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
DEBUG_POWER=$(calculate_power_stats "$DEBUG_POWER_LOG" "$DEBUG_START" "$DEBUG_END")
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

# Clean for release build
echo "Cleaning for release build..."
cargo clean
echo ""

# Run release build benchmark with power monitoring
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
RELEASE_POWER=$(calculate_power_stats "$RELEASE_POWER_LOG" "$RELEASE_START" "$RELEASE_END")
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
echo "  BENCHMARK RESULTS"
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
cat > benchmark-results.txt << EOF
Compile Benchmark Results
=========================
Date: $(date '+%Y-%m-%d %H:%M:%S')

System:
$(if [[ "$OSTYPE" == "darwin"* ]]; then
    echo "  OS: macOS $(sw_vers -productVersion)"
    echo "  CPU: $(sysctl -n machdep.cpu.brand_string)"
    echo "  Cores: $(sysctl -n hw.ncpu)"
    echo "  RAM: $(( $(sysctl -n hw.memsize) / 1073741824 )) GB"
else
    echo "  OS: $(cat /etc/os-release | grep PRETTY_NAME | cut -d'"' -f2)"
    echo "  CPU: $(cat /proc/cpuinfo | grep 'model name' | head -1 | cut -d':' -f2 | xargs)"
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

echo "Results saved to benchmark-results.txt"
