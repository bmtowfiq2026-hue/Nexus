# Nexus Install Script (Windows PowerShell)
# Usage: git clone https://github.com/bmtowfiq2026-hue/Nexus.git; cd Nexus; powershell -File scripts/install.ps1

$ErrorActionPreference = "Stop"
$Host.UI.RawUI.ForegroundColor = "Cyan"
Write-Host "== Installing Nexus ==" -ForegroundColor Cyan
Write-Host ""

# Detect architecture
$ARCH = if ([Environment]::Is64BitOperatingSystem) { "x86_64" } else { "i686" }
Write-Host "Detected: Windows ($ARCH)" -ForegroundColor Yellow

# Install Rust if missing
if (-not (Get-Command "cargo" -ErrorAction SilentlyContinue)) {
    Write-Host "Installing Rust..." -ForegroundColor Yellow
    $rustup = "$env:TEMP\rustup-init.exe"
    Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile $rustup -UseBasicParsing
    Start-Process -Wait -FilePath $rustup -ArgumentList "-y", "--default-host", "x86_64-pc-windows-gnu"
    $env:Path += ";$env:USERPROFILE\.cargo\bin"
    Write-Host "Rust installed" -ForegroundColor Green
} else {
    Write-Host "Rust found: $(cargo --version)" -ForegroundColor Green
}

# Clone repo
$NexusDir = "$env:USERPROFILE\.nexus-repo"
if (Test-Path "$NexusDir\.git") {
    Write-Host "Updating Nexus repository..." -ForegroundColor Yellow
    Set-Location -Path $NexusDir
    git pull --ff-only
} else {
    Write-Host "Cloning Nexus repository..." -ForegroundColor Yellow
    git clone --depth 1 "https://github.com/bmtowfiq2026-hue/Nexus.git" $NexusDir
}

# Build
Write-Host "Building Nexus (this may take a few minutes)..." -ForegroundColor Yellow
Set-Location -Path $NexusDir
cargo build --release

# Build Go gateway (if Go is installed)
if (Get-Command "go" -ErrorAction SilentlyContinue) {
    Write-Host "Building gateway..." -ForegroundColor Yellow
    Set-Location -Path "$NexusDir\gateway"
    go build -o "$NexusDir\target\release\nexus-gateway.exe" .
    Write-Host "Gateway built" -ForegroundColor Green
} else {
    Write-Host "Go not found. Gateway not built (install Go from https://go.dev/dl for `nexus start`)." -ForegroundColor Yellow
}

# Install binaries
$BinDir = "$env:USERPROFILE\.nexus-bin"
New-Item -ItemType Directory -Path $BinDir -Force | Out-Null
Copy-Item "$NexusDir\target\release\nexus.exe" "$BinDir\nexus.exe" -Force
if (Test-Path "$NexusDir\target\release\nexus-gateway.exe") {
    Copy-Item "$NexusDir\target\release\nexus-gateway.exe" "$BinDir\nexus-gateway.exe" -Force
}

# Add to PATH
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$BinDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$UserPath;$BinDir", "User")
    $env:Path += ";$BinDir"
    Write-Host "Added to user PATH" -ForegroundColor Green
} else {
    Write-Host "Already in PATH" -ForegroundColor Green
}

# Init workspace
Write-Host ""
nexus init
Write-Host ""

$Host.UI.RawUI.ForegroundColor = "Cyan"
Write-Host "Nexus installed!" -ForegroundColor Green
Write-Host ""
Write-Host "  nexus chat        Start chatting (demo mode)" -ForegroundColor Green
Write-Host "  nexus start       Launch agent API + WebChat UI" -ForegroundColor Green
Write-Host "  nexus doctor      Check system health" -ForegroundColor Green
Write-Host "  nexus onboard     Guided setup wizard" -ForegroundColor Green
Write-Host ""
Write-Host "Quick start:" -ForegroundColor Yellow
Write-Host "  nexus chat" -ForegroundColor Green
Write-Host "  nexus start (opens http://localhost:8080)" -ForegroundColor Green

$Host.UI.RawUI.ForegroundColor = "White"
