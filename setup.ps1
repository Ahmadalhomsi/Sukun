# Sukun - Setup and Installation Script
# Run this with: powershell -ExecutionPolicy Bypass -File setup.ps1

Write-Host "================================================" -ForegroundColor Cyan
Write-Host "  Sukun Prayer Times Manager - Setup Script" -ForegroundColor Cyan
Write-Host "================================================" -ForegroundColor Cyan
Write-Host ""

# Check if bun is installed
Write-Host "Checking for Bun..." -ForegroundColor Yellow
if (Get-Command bun -ErrorAction SilentlyContinue) {
    Write-Host "✓ Bun found" -ForegroundColor Green
    $useBun = $true
} else {
    Write-Host "✗ Bun not found" -ForegroundColor Red
    Write-Host "Checking for npm..." -ForegroundColor Yellow
    if (Get-Command npm -ErrorAction SilentlyContinue) {
        Write-Host "✓ npm found" -ForegroundColor Green
        $useBun = $false
    } else {
        Write-Host "✗ Neither Bun nor npm found!" -ForegroundColor Red
        Write-Host "Please install Node.js or Bun first." -ForegroundColor Red
        exit 1
    }
}

# Check if Rust is installed
Write-Host "Checking for Rust..." -ForegroundColor Yellow
if (Get-Command cargo -ErrorAction SilentlyContinue) {
    Write-Host "✓ Rust/Cargo found" -ForegroundColor Green
} else {
    Write-Host "✗ Rust not found!" -ForegroundColor Red
    Write-Host "Please install Rust from https://rustup.rs/" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "Installing frontend dependencies..." -ForegroundColor Yellow

try {
    if ($useBun) {
        bun install
    } else {
        npm install
    }
    Write-Host "✓ Frontend dependencies installed" -ForegroundColor Green
} catch {
    Write-Host "✗ Failed to install dependencies" -ForegroundColor Red
    Write-Host $_.Exception.Message -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "================================================" -ForegroundColor Cyan
Write-Host "  Setup Complete!" -ForegroundColor Green
Write-Host "================================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Yellow
Write-Host "  1. Run development server:" -ForegroundColor White
if ($useBun) {
    Write-Host "     bun run tauri dev" -ForegroundColor Cyan
} else {
    Write-Host "     npm run tauri dev" -ForegroundColor Cyan
}
Write-Host ""
Write-Host "  2. Or build for production:" -ForegroundColor White
if ($useBun) {
    Write-Host "     bun run tauri build" -ForegroundColor Cyan
} else {
    Write-Host "     npm run tauri build" -ForegroundColor Cyan
}
Write-Host ""
Write-Host "  3. Read the documentation:" -ForegroundColor White
Write-Host "     README.md - Full documentation" -ForegroundColor Cyan
Write-Host "     QUICKSTART.md - Quick guide" -ForegroundColor Cyan
Write-Host "     PROJECT_SUMMARY.md - Project overview" -ForegroundColor Cyan
Write-Host ""
Write-Host "Note: Rust dependencies will be compiled on first run" -ForegroundColor Yellow
Write-Host "This may take a few minutes." -ForegroundColor Yellow
Write-Host ""
