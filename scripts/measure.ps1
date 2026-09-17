# CactAgent ölçüm script'i
Write-Host "=== CactAgent Olcumleri ===" -ForegroundColor Cyan
Write-Host ""

# 1. Binary boyutu
Write-Host "1. Binary boyutu:" -ForegroundColor Yellow
$binary = "target\release\cactagent.exe"
if (Test-Path $binary) {
    $size = (Get-Item $binary).Length
    $sizeMB = [math]::Round($size / 1MB, 2)
    Write-Host "   $binary : $sizeMB MB ($size byte)"
} else {
    Write-Host "   Binary bulunamadi. Once 'cargo build --release' calistirin."
}

# 2. Model dosyası boyutu
Write-Host ""
Write-Host "2. Model dosyasi:" -ForegroundColor Yellow
$model = "weights\needle2.cact"
if (Test-Path $model) {
    $size = (Get-Item $model).Length
    $sizeMB = [math]::Round($size / 1MB, 2)
    Write-Host "   $model : $sizeMB MB"
} else {
    Write-Host "   Model bulunamadi."
}

# 3. Sistem bilgisi
Write-Host ""
Write-Host "3. Sistem bilgisi:" -ForegroundColor Yellow
$cpu = Get-CimInstance Win32_Processor | Select-Object -First 1
Write-Host "   CPU: $($cpu.Name)"
Write-Host "   Cekirdek: $($cpu.NumberOfCores) cekirdek / $($cpu.NumberOfLogicalProcessors) mantiksal"
$ram = [math]::Round((Get-CimInstance Win32_ComputerSystem).TotalPhysicalMemory / 1GB, 2)
Write-Host "   RAM: $ram GB"

# 4. Test çalıştırması (bellek ölçümü)
Write-Host ""
Write-Host "4. Test calistirmasi (bellek olcumu):" -ForegroundColor Yellow
Write-Host "   Binary calistiriliyor, bellek kullanimi izleniyor..."
Write-Host "   (Bu islem 10-15 saniye surebilir)"
Write-Host ""

$process = Start-Process -FilePath $binary `
    -ArgumentList '"Search the web for Rust news"' `
    -PassThru -WindowStyle Hidden

$maxMemory = 0
$startTime = Get-Date
$timeout = 30

while (-not $process.HasExited -and ((Get-Date) - $startTime).TotalSeconds -lt $timeout) {
    Start-Sleep -Milliseconds 500
    try {
        $proc = Get-Process -Id $process.Id -ErrorAction SilentlyContinue
        if ($proc) {
            $mem = [math]::Round($proc.WorkingSet64 / 1MB, 2)
            if ($mem -gt $maxMemory) {
                $maxMemory = $mem
            }
        }
    } catch {}
}

if (-not $process.HasExited) {
    $process.Kill()
    Write-Host "   Timeout, surec sonlandirildi."
} else {
    Write-Host "   Surec tamamlandi."
}

Write-Host ""
Write-Host "   Maksimum bellek kullanimi: $maxMemory MB" -ForegroundColor Green
Write-Host ""

Write-Host "=== Olcumler Tamamlandi ===" -ForegroundColor Cyan