# Fast Compile Benchmark Script for Windows PowerShell - Endpoint Testing
# Uses a minimal Rust project (~10 seconds) to quickly test the full pipeline
# Usage: powershell -ExecutionPolicy Bypass -File .\fast_benchmark.ps1

$ErrorActionPreference = "Continue"

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  Fast Compile Benchmark (Endpoint Test)" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Check if Rust is installed
function Test-RustInstalled {
    $cargoPath = "$env:USERPROFILE\.cargo\bin"
    if ((Test-Path "$cargoPath\rustc.exe") -and ($env:PATH -notlike "*$cargoPath*")) {
        $env:PATH = "$cargoPath;$env:PATH"
    }
    try {
        $null = Get-Command rustc -ErrorAction Stop
        $null = Get-Command cargo -ErrorAction Stop
        return $true
    } catch {
        if (Test-Path "$env:USERPROFILE\.cargo\bin\rustc.exe") {
            $env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"
            return $true
        }
        return $false
    }
}

if (-not (Test-RustInstalled)) {
    Write-Host "ERROR: Rust is not installed!" -ForegroundColor Red
    Write-Host "Please install Rust from https://rustup.rs/ and run this script again."
    exit 1
}

Write-Host "Rust installation detected!" -ForegroundColor Green
Write-Host ""

# Get system info
Write-Host "System Information:" -ForegroundColor Yellow
$csModel = (Get-CimInstance Win32_ComputerSystem).Model
if (-not $csModel -or $csModel -match 'System Product Name|To Be Filled|Default string') {
    $board = Get-CimInstance Win32_BaseBoard
    $mfr = $board.Manufacturer -replace '(?i)ASUSTeK COMPUTER INC\.?','ASUS' -replace '(?i)Micro-Star International.*','MSI' -replace '(?i)Gigabyte Technology.*','Gigabyte'
    $csModel = "$mfr $($board.Product)".Trim()
}
Write-Host "  Device: $csModel"
Write-Host "  OS: $([System.Environment]::OSVersion.VersionString)"
Write-Host "  CPU: $((Get-CimInstance Win32_Processor).Name.Trim())"
$gpus = Get-CimInstance Win32_VideoController
$discreteGpu = $gpus | Where-Object { $_.Name -notmatch 'Microsoft Basic|Radeon.*Graphics$' } | Select-Object -First 1
if ($discreteGpu) { $selectedGpuDisplay = $discreteGpu } else { $selectedGpuDisplay = $gpus | Select-Object -First 1 }
$gpuName = $selectedGpuDisplay.Name
Write-Host "  GPU: $gpuName"
$vramDisplay = $selectedGpuDisplay.AdapterRAM
if ($vramDisplay -and $vramDisplay -gt 0) {
    $vramGB = [math]::Round($vramDisplay / 1GB)
} else {
    $regPath = "HKLM:\SYSTEM\ControlSet001\Control\Class\{4d36e968-e325-11ce-bfc1-08002be10318}"
    $vramGB = 0
    Get-ChildItem $regPath -ErrorAction SilentlyContinue | ForEach-Object {
        $desc = (Get-ItemProperty $_.PSPath -ErrorAction SilentlyContinue)."DriverDesc"
        if ($desc -eq $gpuName) {
            $qwMem = (Get-ItemProperty $_.PSPath -ErrorAction SilentlyContinue)."HardwareInformation.qwMemorySize"
            if ($qwMem) { $vramGB = [math]::Round([int64]$qwMem / 1GB) }
        }
    }
}
Write-Host "  VRAM: $vramGB GB"
Write-Host "  Cores: $([System.Environment]::ProcessorCount)"
Write-Host "  RAM: $([math]::Round((Get-CimInstance Win32_ComputerSystem).TotalPhysicalMemory / 1GB, 2)) GB"
Write-Host ""

Write-Host "Rust Version:" -ForegroundColor Yellow
rustc --version
cargo --version
Write-Host ""

# Use the fast-bench sub-project
$fastDir = Join-Path $PSScriptRoot "fast-bench"
if (-not (Test-Path (Join-Path $fastDir "Cargo.toml"))) {
    Write-Host "ERROR: fast-bench directory not found at $fastDir" -ForegroundColor Red
    exit 1
}

Push-Location $fastDir

# Clean previous build
Write-Host "Cleaning previous build..." -ForegroundColor Yellow
& cargo clean 2>&1 | Out-Null
Write-Host ""

# Debug build
Write-Host "Starting DEBUG build benchmark..." -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
$debugTime = Measure-Command {
    & cargo build 2>&1 | ForEach-Object {
        $line = if ($_ -is [System.Management.Automation.ErrorRecord]) { $_.ToString() } else { $_ }
        if ($line -notmatch "future.*(version|incompat)" -and $line -notmatch "unused config key") {
            Write-Host $line
        }
    }
}
Write-Host ""
Write-Host "DEBUG build completed in: $($debugTime.TotalSeconds) seconds ($($debugTime.ToString('hh\:mm\:ss\.fff')))" -ForegroundColor Green
Write-Host ""

# Clean for release
Write-Host "Cleaning for release build..." -ForegroundColor Yellow
& cargo clean 2>&1 | Out-Null
Write-Host ""

# Release build
Write-Host "Starting RELEASE build benchmark..." -ForegroundColor Magenta
Write-Host "========================================" -ForegroundColor Magenta
$releaseTime = Measure-Command {
    & cargo build --release 2>&1 | ForEach-Object {
        $line = if ($_ -is [System.Management.Automation.ErrorRecord]) { $_.ToString() } else { $_ }
        if ($line -notmatch "future.*(version|incompat)" -and $line -notmatch "unused config key") {
            Write-Host $line
        }
    }
}
Write-Host ""
Write-Host "RELEASE build completed in: $($releaseTime.TotalSeconds) seconds ($($releaseTime.ToString('hh\:mm\:ss\.fff')))" -ForegroundColor Magenta
Write-Host ""

Pop-Location

# Summary
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  FAST BENCHMARK RESULTS (Endpoint Test)" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "  Debug Build:   $($debugTime.TotalSeconds.ToString('F2')) seconds" -ForegroundColor Green
Write-Host "  Release Build: $($releaseTime.TotalSeconds.ToString('F2')) seconds" -ForegroundColor Magenta
Write-Host ""
Write-Host "  Date: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')"
Write-Host "========================================" -ForegroundColor Cyan

# Save results to file
$results = @"
Fast Compile Benchmark Results (Endpoint Test)
===============================================
Date: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')

System:
  Device: $csModel
  OS: $([System.Environment]::OSVersion.VersionString)
  CPU: $((Get-CimInstance Win32_Processor).Name.Trim())
  GPU: $gpuName
  VRAM: $vramGB GB
  Cores: $([System.Environment]::ProcessorCount)
  RAM: $([math]::Round((Get-CimInstance Win32_ComputerSystem).TotalPhysicalMemory / 1GB, 2)) GB

Rust:
  $(rustc --version)
  $(cargo --version)

Results:
  Debug Build:   $($debugTime.TotalSeconds.ToString('F2')) seconds
  Release Build: $($releaseTime.TotalSeconds.ToString('F2')) seconds
"@

$results | Out-File -FilePath "fast-benchmark-results.txt" -Encoding UTF8
Write-Host "Results saved to fast-benchmark-results.txt" -ForegroundColor Gray

# Send results to endpoint
$BenchmarkApiUrl = (Get-Content "$PSScriptRoot\benchmark-config.cfg" -First 1).Trim()

function Send-BenchmarkResults {
    Write-Host ""
    Write-Host "Sending results to $BenchmarkApiUrl..." -ForegroundColor Yellow

    $sysModel = (Get-CimInstance Win32_ComputerSystem).Model
    if (-not $sysModel -or $sysModel -match 'System Product Name|To Be Filled|Default string') {
        $board = Get-CimInstance Win32_BaseBoard
        $mfr = $board.Manufacturer -replace '(?i)ASUSTeK COMPUTER INC\.?','ASUS' -replace '(?i)Micro-Star International.*','MSI' -replace '(?i)Gigabyte Technology.*','Gigabyte'
        $sysDevice = "$mfr $($board.Product)".Trim()
    } else {
        $sysDevice = $sysModel
    }
    $sysOS = [System.Environment]::OSVersion.VersionString
    $sysCPU = (Get-CimInstance Win32_Processor).Name.Trim()
    $gpuList = Get-CimInstance Win32_VideoController
    $discrete = $gpuList | Where-Object { $_.Name -notmatch 'Microsoft Basic|Radeon.*Graphics$' } | Select-Object -First 1
    if ($discrete) { $selectedGpuPayload = $discrete } else { $selectedGpuPayload = $gpuList | Select-Object -First 1 }
    $sysGPU = $selectedGpuPayload.Name
    $vramBytes = $selectedGpuPayload.AdapterRAM
    if ($vramBytes -and $vramBytes -gt 0) {
        $sysVRAM = [math]::Round($vramBytes / 1GB)
    } else {
        $regPath = "HKLM:\SYSTEM\ControlSet001\Control\Class\{4d36e968-e325-11ce-bfc1-08002be10318}"
        $sysVRAM = 0
        Get-ChildItem $regPath -ErrorAction SilentlyContinue | ForEach-Object {
            $desc = (Get-ItemProperty $_.PSPath -ErrorAction SilentlyContinue)."DriverDesc"
            if ($desc -eq $sysGPU) {
                $qwMem = (Get-ItemProperty $_.PSPath -ErrorAction SilentlyContinue)."HardwareInformation.qwMemorySize"
                if ($qwMem) { $sysVRAM = [math]::Round([int64]$qwMem / 1GB) }
            }
        }
    }
    $sysCores = [System.Environment]::ProcessorCount
    $sysRAM = [math]::Floor((Get-CimInstance Win32_ComputerSystem).TotalPhysicalMemory / 1GB)

    $payload = @{
        timestamp = (Get-Date).ToUniversalTime().ToString("yyyy-MM-ddTHH:mm:ssZ")
        system = @{
            device = $sysDevice
            os = $sysOS
            cpu = $sysCPU
            gpu = $sysGPU
            vram_gb = $sysVRAM
            cores = $sysCores
            ram_gb = $sysRAM
        }
        rust = @{
            rustc = (rustc --version) -join ""
            cargo = (cargo --version) -join ""
        }
        results = @{
            debug = @{
                time_seconds = [math]::Round($debugTime.TotalSeconds, 3)
                avg_power = "N/A"
                energy = "N/A"
            }
            release = @{
                time_seconds = [math]::Round($releaseTime.TotalSeconds, 3)
                avg_power = "N/A"
                energy = "N/A"
            }
        }
        power_monitoring_enabled = $false
        benchmark_type = "fast"
    } | ConvertTo-Json -Depth 4

    $payload | Out-File -FilePath "fast-benchmark-results.json" -Encoding UTF8
    Write-Host "Results saved to fast-benchmark-results.json" -ForegroundColor Gray

    try {
        $response = Invoke-WebRequest -Uri $BenchmarkApiUrl `
            -Method Post `
            -ContentType "application/json" `
            -Body $payload `
            -TimeoutSec 30 `
            -UseBasicParsing `
            -ErrorAction Stop

        Write-Host "Results sent successfully! (HTTP $($response.StatusCode))" -ForegroundColor Green
    } catch {
        $statusCode = $null
        if ($_.Exception.Response) {
            $statusCode = [int]$_.Exception.Response.StatusCode
        }

        if ($statusCode) {
            Write-Host "WARNING: Server responded with HTTP $statusCode" -ForegroundColor Red
        } else {
            Write-Host "WARNING: Could not connect to $BenchmarkApiUrl" -ForegroundColor Red
            Write-Host "  Error: $($_.Exception.Message)" -ForegroundColor Red
        }
    }
}

Send-BenchmarkResults
