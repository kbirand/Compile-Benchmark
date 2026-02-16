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
    Write-Host "  Raw Model: '$device'"
    if (-not $device -or $device -match 'System Product Name|To Be Filled|Default string') {
        $board = Get-CimInstance Win32_BaseBoard
        Write-Host "  Raw Manufacturer: '$($board.Manufacturer)'"
        Write-Host "  Raw Product: '$($board.Product)'"
        $mfr = $board.Manufacturer -replace '(?i)ASUSTeK COMPUTER INC\.?','ASUS' -replace '(?i)Micro-Star International.*','MSI' -replace '(?i)Gigabyte Technology.*','Gigabyte'
        $device = "$mfr $($board.Product)".Trim()
        Write-Host "  Cleaned up: '$device'"
    }
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
    $cpu = (Get-CimInstance Win32_Processor).Name.Trim()
    Write-Host "  Value: '$cpu'"
    if ($cpu) { Write-Host "  Status: OK" -ForegroundColor Green } else { Write-Host "  Status: FAILED" -ForegroundColor Red }
} catch {
    Write-Host "  Status: FAILED ($_)" -ForegroundColor Red
}
Write-Host ""

# GPU
Write-Host "GPU:" -ForegroundColor Yellow
try {
    $gpus = Get-CimInstance Win32_VideoController
    Write-Host "  All GPUs found:"
    foreach ($g in $gpus) { Write-Host "    - $($g.Name)" }
    $discrete = $gpus | Where-Object { $_.Name -notmatch 'Microsoft Basic|Radeon.*Graphics$' } | Select-Object -First 1
    if ($discrete) { $gpu = $discrete.Name } else { $gpu = ($gpus | Select-Object -First 1).Name }
    Write-Host "  Selected: '$gpu'"
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
