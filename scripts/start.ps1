# Nexus Start Script (Windows PowerShell)
# Single command to launch everything
param(
    [string]$Provider = "demo",
    [int]$Port = 9876,
    [int]$GatewayPort = 8080
)

$root = Split-Path -Parent $PSScriptRoot
$nexus = "$root\target\release\nexus.exe"

if (-not (Test-Path $nexus)) {
    Write-Host "Nexus binary not found. Run build-all.ps1 first." -ForegroundColor Red
    exit 1
}

Write-Host "== Nexus Start ==" -ForegroundColor Cyan
Write-Host "Starting agent API + WebChat UI..." -ForegroundColor Yellow
Write-Host ""

& $nexus start --no-browser --provider $Provider --port $Port --gateway-port $GatewayPort