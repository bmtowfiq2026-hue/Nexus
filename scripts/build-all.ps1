# Nexus Build-All Script (Windows PowerShell)
# Builds both the Rust CLI and Go gateway

$ErrorActionPreference = "Stop"
$Host.UI.RawUI.ForegroundColor = "Cyan"
Write-Host "== Building Nexus ==" -ForegroundColor Cyan
$RootDir = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
Write-Host ""

# 1. Build Rust CLI
Write-Host "[1/2] Building Rust CLI..." -ForegroundColor Yellow
Set-Location $RootDir
cargo build --release 2>&1 | Select-Object -Last 3
if (Test-Path "$RootDir\target\release\nexus.exe") {
    Write-Host "  nexus binary: $RootDir\target\release\nexus.exe" -ForegroundColor Green
} else {
    Write-Host "  Rust build failed" -ForegroundColor Red
    exit 1
}

# 2. Build Go gateway
Write-Host "`n[2/2] Building Go gateway..." -ForegroundColor Yellow
$goPath = Get-Command "go" -ErrorAction SilentlyContinue
if ($goPath) {
    Set-Location "$RootDir\gateway"
    go build -o "$RootDir\target\release\nexus-gateway.exe" .
    Write-Host "  gateway binary: $RootDir\target\release\nexus-gateway.exe" -ForegroundColor Green
} else {
    Write-Host "  Go not found. Install from https://go.dev/dl" -ForegroundColor Red
    exit 1
}

Write-Host "`nBuild complete!" -ForegroundColor Green
Write-Host ""
Write-Host "  nexus.exe         Rust CLI agent" -ForegroundColor Green
Write-Host "  nexus-gateway.exe Go multi-channel gateway" -ForegroundColor Green
Write-Host ""
Write-Host "Run: .\target\release\nexus start"
