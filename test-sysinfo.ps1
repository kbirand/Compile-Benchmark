# Test script to verify system info detection on Windows
# Usage: .\test-sysinfo.ps1

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  System Info Detection Test" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Platform: Windows"
Write-Host ""

# Device
Write-Host "Device:" -ForegroundColor Yellow
try {
    $device = (Get-CimInstance Win32_ComputerSystem).Model
    Write-Host "  Value: '$device'"
    if ($device) { Write-Host "  Status: OK" -ForegroundColor Green } else { Write-Host "  Status: FAILED" -ForegroundColor Red }
} catch {
    Write-Host "  Status: FAILED ($_)" -ForegroundColor Red
}
Write-Host ""

# OS
Write-Host "OS:" -ForegroundColor Yellow
try {
    $os = [System.Environment]::OSVersion.VersionString
    Write-Host "  Value: '$os'"
    if ($os) { Write-Host "  Status: OK" -ForegroundColor Green } else { Write-Host "  Status: FAILED" -ForegroundColor Red }
} catch {
    Write-Host "  Status: FAILED ($_)" -ForegroundColor Red
}
Write-Host ""

# CPU
Write-Host "CPU:" -ForegroundColor Yellow
try {
    $cpu = (Get-CimInstance Win32_Processor).Name
    Write-Host "  Value: '$cpu'"
    if ($cpu) { Write-Host "  Status: OK" -ForegroundColor Green } else { Write-Host "  Status: FAILED" -ForegroundColor Red }
} catch {
    Write-Host "  Status: FAILED ($_)" -ForegroundColor Red
}
Write-Host ""

# GPU
Write-Host "GPU:" -ForegroundColor Yellow
try {
    $gpu = (Get-CimInstance Win32_VideoController | Select-Object -First 1).Name
    Write-Host "  Value: '$gpu'"
    if ($gpu) { Write-Host "  Status: OK" -ForegroundColor Green } else { Write-Host "  Status: FAILED" -ForegroundColor Red }
} catch {
    Write-Host "  Status: FAILED ($_)" -ForegroundColor Red
}
Write-Host ""

# Cores
Write-Host "Cores:" -ForegroundColor Yellow
try {
    $cores = [System.Environment]::ProcessorCount
    Write-Host "  Value: '$cores'"
    if ($cores) { Write-Host "  Status: OK" -ForegroundColor Green } else { Write-Host "  Status: FAILED" -ForegroundColor Red }
} catch {
    Write-Host "  Status: FAILED ($_)" -ForegroundColor Red
}
Write-Host ""

# RAM
Write-Host "RAM:" -ForegroundColor Yellow
try {
    $ram = [math]::Floor((Get-CimInstance Win32_ComputerSystem).TotalPhysicalMemory / 1GB)
    Write-Host "  Value: '$ram GB'"
    if ($ram) { Write-Host "  Status: OK" -ForegroundColor Green } else { Write-Host "  Status: FAILED" -ForegroundColor Red }
} catch {
    Write-Host "  Status: FAILED ($_)" -ForegroundColor Red
}
Write-Host ""

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  JSON Preview" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

$payload = @{
    system = @{
        device = $device
        os = $os
        cpu = $cpu
        gpu = $gpu
        cores = $cores
        ram_gb = $ram
    }
} | ConvertTo-Json -Depth 4

Write-Host $payload
