# scripts/pre-commit.ps1
Write-Host "=== Pre-commit kontrolleri ===" -ForegroundColor Cyan

Write-Host "1. cargo fmt..." -ForegroundColor Yellow
cargo fmt --all
if ($LASTEXITCODE -ne 0) { exit 1 }

Write-Host "2. cargo clippy..." -ForegroundColor Yellow
cargo clippy --all-targets -- -D warnings
if ($LASTEXITCODE -ne 0) { exit 1 }

Write-Host "3. cargo test..." -ForegroundColor Yellow
cargo test
if ($LASTEXITCODE -ne 0) { exit 1 }

Write-Host "=== Tum kontroller gecti! ===" -ForegroundColor Green