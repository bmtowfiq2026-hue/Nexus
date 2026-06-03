# Nexus Install Script (Windows PowerShell)
# Usage: irm https://raw.githubusercontent.com/nexus/nexus/main/scripts/install.ps1 | iex

$ErrorActionPreference = "Stop"
$Host.UI.RawUI.ForegroundColor = "Cyan"
Write-Host "🦞 Installing Nexus..."
Write-Host ""

# Detect architecture
$ARCH = if ([Environment]::Is64BitOperatingSystem) { "x86_64" } else { "i686" }
Write-Host "Detected: Windows ($ARCH)" -ForegroundColor Yellow

# Install Rust if missing
if (-not (Get-Command "cargo" -ErrorAction SilentlyContinue)) {
    Write-Host "Installing Rust..." -ForegroundColor Yellow
    $rustup = "$env:TEMP\rustup-init.exe"
    Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile $rustup -UseBasicParsing
    Start-Process -Wait -FilePath $rustup -ArgumentList "-y", "--default-host", "x86_64-pc-windows-msvc"
    $env:Path += ";$env:USERPROFILE\.cargo\bin"
    Write-Host "✓ Rust installed" -ForegroundColor Green
} else {
    Write-Host "✓ Rust found: $(cargo --version)" -ForegroundColor Green
}

# Clone repo
$NexusDir = "$env:USERPROFILE\.nexus-repo"
if (Test-Path "$NexusDir\.git") {
    Write-Host "Updating Nexus repository..." -ForegroundColor Yellow
    Set-Location -Path $NexusDir
    git pull --ff-only
} else {
    Write-Host "Cloning Nexus repository..." -ForegroundColor Yellow
    git clone --depth 1 "https://github.com/nexus/nexus.git" $NexusDir
}

# Build
Write-Host "Building Nexus (this may take a few minutes)..." -ForegroundColor Yellow
Set-Location -Path $NexusDir
cargo build --release

# Install binary
$BinDir = "$env:USERPROFILE\.nexus-bin"
New-Item -ItemType Directory -Path $BinDir -Force | Out-Null
Copy-Item "$NexusDir\target\release\nexus-cli.exe" "$BinDir\nexus.exe" -Force

# Add to PATH
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$BinDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$UserPath;$BinDir", "User")
    $env:Path += ";$BinDir"
    Write-Host "✓ Added to user PATH" -ForegroundColor Green
} else {
    Write-Host "✓ Already in PATH" -ForegroundColor Green
}

# Init workspace
Write-Host ""
nexus init
Write-Host ""

$Host.UI.RawUI.ForegroundColor = "Cyan"
Write-Host "✓ Nexus installed!" -ForegroundColor Green
Write-Host ""
Write-Host "  nexus chat        Start chatting (demo mode)" -ForegroundColor Green
Write-Host "  nexus doctor      Check system health" -ForegroundColor Green
Write-Host "  nexus onboard     Guided setup wizard" -ForegroundColor Green
Write-Host ""
Write-Host "Quick start:" -ForegroundColor Yellow
Write-Host "  nexus chat" -ForegroundColor Green
Write-Host ""

$Host.UI.RawUI.ForegroundColor = "White"
