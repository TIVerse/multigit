# MultiGit PowerShell Installation Script
# For Windows users

$ErrorActionPreference = "Stop"

Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan
Write-Host "🚀 MultiGit Windows Installer (PowerShell)" -ForegroundColor Cyan
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan
Write-Host ""

# Determine install directory
$InstallDir = "$env:USERPROFILE\.cargo\bin"
Write-Host "Install directory: $InstallDir" -ForegroundColor Yellow

# Create install directory if it doesn't exist
if (-not (Test-Path $InstallDir)) {
    Write-Host "Creating $InstallDir..." -ForegroundColor Yellow
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

# Check if binaries exist
$ReleasePath = "target\release"
if (-not (Test-Path "$ReleasePath\multigit.exe")) {
    Write-Host "❌ Error: No release build found" -ForegroundColor Red
    Write-Host "Please run: cargo build --release" -ForegroundColor Yellow
    exit 1
}

Write-Host ""
Write-Host "📦 Installing binaries..." -ForegroundColor Green

# Copy binaries
try {
    Copy-Item -Path "$ReleasePath\multigit.exe" -Destination "$InstallDir\multigit.exe" -Force
    Copy-Item -Path "$ReleasePath\mg.exe" -Destination "$InstallDir\mg.exe" -Force
    Write-Host "  ✅ Copied multigit.exe" -ForegroundColor Green
    Write-Host "  ✅ Copied mg.exe" -ForegroundColor Green
}
catch {
    Write-Host "❌ Error copying binaries: $_" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan
Write-Host "✅ Installation Complete!" -ForegroundColor Green
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan
Write-Host ""
Write-Host "Installed to: $InstallDir" -ForegroundColor Yellow
Write-Host ""

# Check if in PATH
$PathArray = $env:PATH -split ';'
if ($PathArray -contains $InstallDir) {
    Write-Host "✅ Install directory is in PATH" -ForegroundColor Green
    Write-Host ""
    Write-Host "Verify installation:" -ForegroundColor Yellow
    Write-Host "  multigit --version"
    Write-Host "  mg --version"
    Write-Host ""
    Write-Host "Get started:" -ForegroundColor Yellow
    Write-Host "  mg --help"
    Write-Host "  mg init"
}
else {
    Write-Host "⚠️  Warning: Install directory is NOT in PATH" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "To add to PATH permanently (run as Administrator):" -ForegroundColor Yellow
    Write-Host "  [Environment]::SetEnvironmentVariable('Path', `$env:Path + ';$InstallDir', 'User')" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Or manually add this to your PATH:" -ForegroundColor Yellow
    Write-Host "  $InstallDir" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "After adding to PATH, restart your terminal and run:" -ForegroundColor Yellow
    Write-Host "  mg --version"
}

Write-Host ""
Write-Host "Press any key to exit..."
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
