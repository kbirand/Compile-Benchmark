#!/bin/bash
# Test script to verify system info detection on macOS/Linux
# Usage: chmod +x test-sysinfo.sh && ./test-sysinfo.sh

echo "========================================"
echo "  System Info Detection Test"
echo "========================================"
echo ""

if [[ "$OSTYPE" == "darwin"* ]]; then
    echo "Platform: macOS"
    echo ""

    echo "Device:"
    DEVICE="$(system_profiler SPHardwareDataType | grep 'Model Name' | cut -d':' -f2 | xargs)"
    echo "  Value: '${DEVICE}'"
    [[ -n "$DEVICE" ]] && echo "  Status: OK" || echo "  Status: FAILED"
    echo ""

    echo "OS:"
    OS="macOS $(sw_vers -productVersion)"
    echo "  Value: '${OS}'"
    [[ -n "$OS" ]] && echo "  Status: OK" || echo "  Status: FAILED"
    echo ""

    echo "CPU:"
    CPU="$(sysctl -n machdep.cpu.brand_string)"
    echo "  Value: '${CPU}'"
    [[ -n "$CPU" ]] && echo "  Status: OK" || echo "  Status: FAILED"
    echo ""

    echo "GPU:"
    GPU="$(system_profiler SPDisplaysDataType | grep 'Chipset Model' | cut -d':' -f2 | xargs)"
    echo "  Value: '${GPU}'"
    [[ -n "$GPU" ]] && echo "  Status: OK" || echo "  Status: FAILED (may be integrated in CPU)"
    echo ""

    echo "GPU Cores:"
    GPU_CORES="$(system_profiler SPDisplaysDataType | grep 'Total Number of Cores' | cut -d':' -f2 | xargs)"
    echo "  Value: '${GPU_CORES}'"
    [[ -n "$GPU_CORES" ]] && echo "  Status: OK" || echo "  Status: FAILED"
    echo ""

    echo "Cores:"
    CORES="$(sysctl -n hw.ncpu)"
    echo "  Value: '${CORES}'"
    [[ -n "$CORES" ]] && echo "  Status: OK" || echo "  Status: FAILED"
    echo ""

    echo "RAM:"
    RAM="$(( $(sysctl -n hw.memsize) / 1073741824 ))"
    echo "  Value: '${RAM} GB'"
    [[ -n "$RAM" ]] && echo "  Status: OK" || echo "  Status: FAILED"
    echo ""

else
    echo "Platform: Linux"
    echo ""

    echo "Device:"
    DEVICE="$(cat /sys/devices/virtual/dmi/id/product_name 2>/dev/null || echo '')"
    echo "  Value: '${DEVICE}'"
    [[ -n "$DEVICE" ]] && echo "  Status: OK" || echo "  Status: FAILED (DMI not available)"
    echo ""

    echo "OS:"
    OS="$(cat /etc/os-release 2>/dev/null | grep PRETTY_NAME | cut -d'"' -f2)"
    echo "  Value: '${OS}'"
    [[ -n "$OS" ]] && echo "  Status: OK" || echo "  Status: FAILED"
    echo ""

    echo "CPU:"
    CPU="$(cat /proc/cpuinfo | grep 'model name' | head -1 | cut -d':' -f2 | xargs)"
    echo "  Value: '${CPU}'"
    [[ -n "$CPU" ]] && echo "  Status: OK" || echo "  Status: FAILED"
    echo ""

    echo "GPU:"
    GPU="$(lspci 2>/dev/null | grep -i 'vga\|3d\|display' | head -1 | cut -d':' -f3 | xargs)"
    echo "  Value: '${GPU}'"
    [[ -n "$GPU" ]] && echo "  Status: OK" || echo "  Status: FAILED (lspci not available or no GPU)"
    echo ""

    echo "GPU Cores:"
    if command -v nvidia-smi &>/dev/null; then
        GPU_CORES="$(nvidia-smi --query-gpu=count --format=csv,noheader,nounits 2>/dev/null | head -1)"
        # Try CUDA cores via nvidia-settings or fallback
        CUDA_CORES="$(nvidia-settings -q CUDACores -t 2>/dev/null | head -1)"
        [[ -n "$CUDA_CORES" ]] && GPU_CORES="$CUDA_CORES"
    elif [ -d /sys/class/drm/card0/device ]; then
        # AMD GPUs expose compute units
        GPU_CORES="$(cat /sys/class/drm/card0/device/pp_num_compute_units 2>/dev/null || echo '')"
    else
        GPU_CORES=""
    fi
    echo "  Value: '${GPU_CORES}'"
    [[ -n "$GPU_CORES" ]] && echo "  Status: OK" || echo "  Status: FAILED (could not detect GPU cores)"
    echo ""

    echo "Cores:"
    CORES="$(nproc)"
    echo "  Value: '${CORES}'"
    [[ -n "$CORES" ]] && echo "  Status: OK" || echo "  Status: FAILED"
    echo ""

    echo "RAM:"
    RAM="$(( $(cat /proc/meminfo | grep MemTotal | awk '{print $2}') / 1048576 ))"
    echo "  Value: '${RAM} GB'"
    [[ -n "$RAM" ]] && echo "  Status: OK" || echo "  Status: FAILED"
    echo ""
fi

echo "========================================"
echo "  JSON Preview"
echo "========================================"
cat <<EOJSON
{
  "system": {
    "device": "${DEVICE}",
    "os": "${OS}",
    "cpu": "${CPU}",
    "gpu": "${GPU}",
    "gpu_cores": "${GPU_CORES:-N/A}",
    "cores": ${CORES},
    "ram_gb": ${RAM}
  }
}
EOJSON
