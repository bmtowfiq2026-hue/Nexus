# Nexus Setup Script (Windows PowerShell)
# Usage: .\scripts\setup.ps1

$ErrorActionPreference = "Stop"
$NexusDir = "$env:USERPROFILE\.nexus"

Write-Host "🦞 Nexus Setup" -ForegroundColor Cyan

# Check Rust
if (-not (Get-Command "cargo" -ErrorAction SilentlyContinue)) {
    Write-Host "✗ Rust not found. Install from https://rustup.rs" -ForegroundColor Red
    exit 1
}
Write-Host "✓ Rust: $(cargo --version)" -ForegroundColor Green

# Check Go
if (-not (Get-Command "go" -ErrorAction SilentlyContinue)) {
    Write-Host "✗ Go not found. Install from https://go.dev/dl" -ForegroundColor Red
    exit 1
}
Write-Host "✓ Go: $(go version)" -ForegroundColor Green

# Build
Write-Host "`nBuilding Nexus..." -ForegroundColor Yellow
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "✗ Build failed" -ForegroundColor Red
    exit 1
}
Write-Host "✓ Build complete" -ForegroundColor Green

# Initialize
if (-not (Test-Path $NexusDir)) {
    & ".\target\release\netus.exe" init --path $NexusDir
    Write-Host "✓ Workspace initialized at $NexusDir" -ForegroundColor Green
}

Write-Host "`nNexus is ready!" -ForegroundColor Cyan
Write-Host "  Run: nexus chat" -ForegroundColor White
