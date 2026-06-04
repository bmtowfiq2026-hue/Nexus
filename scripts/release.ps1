# Nexus Release Script
# Usage: .\scripts\release.ps1 -Version "0.6.0"
# Requires: gh CLI authenticated, cargo, go

param(
    [Parameter(Mandatory = $true)]
    [string]$Version
)

$ErrorActionPreference = "Stop"
$RepoRoot = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)
$DistDir = "$RepoRoot\dist"
$GhPath = "$env:TEMP\gh-cli\bin\gh.exe"

if (-not (Test-Path -LiteralPath $GhPath)) {
    Write-Error "gh CLI not found at $GhPath. Run: .\scripts\bootstrap-gh.ps1"
    exit 1
}

Write-Host "== Nexus Release v$Version ==" -ForegroundColor Cyan

# Clean dist
if (Test-Path -LiteralPath $DistDir) { Remove-Item -Recurse -Force -LiteralPath $DistDir }
New-Item -ItemType Directory -Path $DistDir -Force | Out-Null

# Build Rust release
Write-Host "Building Rust CLI..." -ForegroundColor Yellow
Set-Location -LiteralPath $RepoRoot
cargo build --release
if ($LASTEXITCODE -ne 0) { throw "Rust build failed" }

# Build Go gateway
Write-Host "Building Go gateway..." -ForegroundColor Yellow
Set-Location -LiteralPath "$RepoRoot\gateway"
go build -o "$DistDir\nexus-gateway.exe" .
if ($LASTEXITCODE -ne 0) { throw "Go build failed" }

# Package platforms
$Platforms = @(
    @{ Name = "windows-x86_64";  Bin = "nexus.exe";     Gateway = "nexus-gateway.exe"; Script = "install.ps1" }
)

foreach ($p in $Platforms) {
    $pkgDir = "$DistDir\nexus-v$Version-$($p.Name)"
    New-Item -ItemType Directory -Path $pkgDir -Force | Out-Null

    Copy-Item "$RepoRoot\target\release\$($p.Bin)" "$pkgDir\nexus.exe" -Force
    Copy-Item "$DistDir\$($p.Gateway)" "$pkgDir\" -Force
    Copy-Item "$RepoRoot\gateway\gateway.json" "$pkgDir\" -Force
    if (Test-Path "$RepoRoot\scripts\$($p.Script)") {
        Copy-Item "$RepoRoot\scripts\$($p.Script)" "$pkgDir\" -Force
    }

    Add-Type -AssemblyName System.IO.Compression.FileSystem
    [System.IO.Compression.ZipFile]::CreateFromDirectory(
        $pkgDir,
        "$DistDir\nexus-v$Version-$($p.Name).zip",
        [System.IO.Compression.CompressionLevel]::Optimal,
        $false
    )

    Remove-Item -Recurse -Force -LiteralPath $pkgDir
    Write-Host "  Packaged: nexus-v$Version-$($p.Name).zip" -ForegroundColor Green
}

# Create GitHub release
Write-Host "Creating GitHub release v$Version..." -ForegroundColor Yellow
$notesFile = "$DistDir\RELEASE_NOTES.md"
Set-Location -LiteralPath $RepoRoot
& $GhPath release create "v$Version" `
    --title "Nexus v$Version" `
    --notes-file $notesFile `
    --draft `
    "$DistDir\nexus-v$Version-windows-x86_64.zip"

if ($LASTEXITCODE -eq 0) {
    Write-Host "Release v$Version created as draft!" -ForegroundColor Green
    Write-Host "  Review and publish at: https://github.com/bmtowfiq2026-hue/Nexus/releases" -ForegroundColor Cyan
} else {
    Write-Error "Release creation failed"
}

Write-Host ""
Write-Host "Done!" -ForegroundColor Green
