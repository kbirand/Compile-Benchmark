# Compile Benchmark Script for Windows PowerShell
# Usage: .\benchmark.ps1

# Don't use Stop mode as it conflicts with native commands
$ErrorActionPreference = "Continue"

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  Rust Compile Time Benchmark" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Check if Rust is installed (also check common install location)
function Test-RustInstalled {
    # First, try to add cargo to PATH if it exists but isn't in PATH
    $cargoPath = "$env:USERPROFILE\.cargo\bin"
    if ((Test-Path "$cargoPath\rustc.exe") -and ($env:PATH -notlike "*$cargoPath*")) {
        $env:PATH = "$cargoPath;$env:PATH"
    }
    
    try {
        $null = Get-Command rustc -ErrorAction Stop
        $null = Get-Command cargo -ErrorAction Stop
        return $true
    } catch {
        # Also check if rustc exists in the default location even if not in PATH
        if (Test-Path "$env:USERPROFILE\.cargo\bin\rustc.exe") {
            $env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"
            return $true
        }
        return $false
    }
}

function Install-Rust {
    Write-Host "Downloading Rust installer..." -ForegroundColor Yellow
    $rustupUrl = "https://win.rustup.rs/x86_64"
    $rustupPath = "$env:TEMP\rustup-init.exe"
    
    try {
        [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
        Invoke-WebRequest -Uri $rustupUrl -OutFile $rustupPath -UseBasicParsing
        
        Write-Host "Installing Rust (this may take a few minutes)..." -ForegroundColor Yellow
        Write-Host ""
        
        # Run rustup-init with default options (-y for unattended)
        Start-Process -FilePath $rustupPath -ArgumentList "-y" -Wait -NoNewWindow
        
        # Add cargo to current session PATH
        $cargoPath = "$env:USERPROFILE\.cargo\bin"
        if (Test-Path $cargoPath) {
            $env:PATH = "$cargoPath;$env:PATH"
        }
        
        # Clean up installer
        Remove-Item $rustupPath -Force -ErrorAction SilentlyContinue
        
        Write-Host ""
        Write-Host "Rust installation complete!" -ForegroundColor Green
        Write-Host ""
        return $true
    } catch {
        Write-Host "Failed to install Rust: $_" -ForegroundColor Red
        return $false
    }
}

if (-not (Test-RustInstalled)) {
    Write-Host "Rust is not installed." -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Rust is required to run this benchmark." -ForegroundColor Yellow
    Write-Host ""
    
    $install = Read-Host "Would you like to install Rust now? (Y/N)"
    if ($install -eq 'Y' -or $install -eq 'y') {
        $installed = Install-Rust
        
        if (-not $installed -or -not (Test-RustInstalled)) {
            Write-Host ""
            Write-Host "Rust installation may require a terminal restart." -ForegroundColor Yellow
            Write-Host "Please restart PowerShell and run this script again."
            Write-Host ""
            Write-Host "Press any key to exit..."
            $null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
            exit 1
        }
    } else {
        Write-Host ""
        Write-Host "Please install Rust manually from https://rustup.rs/ and run this script again."
        Write-Host ""
        Write-Host "Press any key to exit..."
        $null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
        exit 1
    }
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
if ($discreteGpu) { $gpuName = $discreteGpu.Name } else { $gpuName = ($gpus | Select-Object -First 1).Name }
Write-Host "  GPU: $gpuName"
Write-Host "  Cores: $([System.Environment]::ProcessorCount)"
Write-Host "  RAM: $([math]::Round((Get-CimInstance Win32_ComputerSystem).TotalPhysicalMemory / 1GB, 2)) GB"
Write-Host ""

# Check Rust installation
Write-Host "Rust Version:" -ForegroundColor Yellow
rustc --version
cargo --version
Write-Host ""

# Clean previous build
Write-Host "Cleaning previous build..." -ForegroundColor Yellow
& cargo clean 2>&1 | Out-Null
Write-Host ""

# Run debug build benchmark
Write-Host "Starting DEBUG build benchmark..." -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
$debugTime = Measure-Command {
    & cargo build 2>&1 | ForEach-Object { 
        $line = if ($_ -is [System.Management.Automation.ErrorRecord]) { $_.ToString() } else { $_ }
        # Filter out future-incompat warnings and unused config warnings
        if ($line -notmatch "future.*(version|incompat)" -and $line -notmatch "unused config key") {
            Write-Host $line
        }
    }
}
Write-Host ""
Write-Host "DEBUG build completed in: $($debugTime.TotalSeconds) seconds ($($debugTime.ToString('hh\:mm\:ss\.fff')))" -ForegroundColor Green
Write-Host ""

# Clean for release build
Write-Host "Cleaning for release build..." -ForegroundColor Yellow
& cargo clean 2>&1 | Out-Null
Write-Host ""

# Run release build benchmark
Write-Host "Starting RELEASE build benchmark..." -ForegroundColor Magenta
Write-Host "========================================" -ForegroundColor Magenta
$releaseTime = Measure-Command {
    & cargo build --release 2>&1 | ForEach-Object { 
        $line = if ($_ -is [System.Management.Automation.ErrorRecord]) { $_.ToString() } else { $_ }
        # Filter out future-incompat warnings and unused config warnings
        if ($line -notmatch "future.*(version|incompat)" -and $line -notmatch "unused config key") {
            Write-Host $line
        }
    }
}
Write-Host ""
Write-Host "RELEASE build completed in: $($releaseTime.TotalSeconds) seconds ($($releaseTime.ToString('hh\:mm\:ss\.fff')))" -ForegroundColor Magenta
Write-Host ""

# Summary
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  BENCHMARK RESULTS" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "  Debug Build:   $($debugTime.TotalSeconds.ToString('F2')) seconds" -ForegroundColor Green
Write-Host "  Release Build: $($releaseTime.TotalSeconds.ToString('F2')) seconds" -ForegroundColor Magenta
Write-Host ""
Write-Host "  Date: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')"
Write-Host "========================================" -ForegroundColor Cyan

# Save results to file
$results = @"
Compile Benchmark Results
=========================
Date: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')

System:
  Device: $csModel
  OS: $([System.Environment]::OSVersion.VersionString)
  CPU: $((Get-CimInstance Win32_Processor).Name)
  GPU: $gpuName
  Cores: $([System.Environment]::ProcessorCount)
  RAM: $([math]::Round((Get-CimInstance Win32_ComputerSystem).TotalPhysicalMemory / 1GB, 2)) GB

Rust:
  $(rustc --version)
  $(cargo --version)

Results:
  Debug Build:   $($debugTime.TotalSeconds.ToString('F2')) seconds
  Release Build: $($releaseTime.TotalSeconds.ToString('F2')) seconds
"@

$results | Out-File -FilePath "benchmark-results.txt" -Encoding UTF8
Write-Host "Results saved to benchmark-results.txt" -ForegroundColor Gray

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
    if ($discrete) { $sysGPU = $discrete.Name } else { $sysGPU = ($gpuList | Select-Object -First 1).Name }
    $sysCores = [System.Environment]::ProcessorCount
    $sysRAM = [math]::Floor((Get-CimInstance Win32_ComputerSystem).TotalPhysicalMemory / 1GB)

    $payload = @{
        timestamp = (Get-Date).ToUniversalTime().ToString("yyyy-MM-ddTHH:mm:ssZ")
        system = @{
            device = $sysDevice
            os = $sysOS
            cpu = $sysCPU
            gpu = $sysGPU
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
    } | ConvertTo-Json -Depth 4

    # Save JSON to file
    $payload | Out-File -FilePath "benchmark-results.json" -Encoding UTF8
    Write-Host "Results saved to benchmark-results.json" -ForegroundColor Gray

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
