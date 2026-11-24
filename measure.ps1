# Measures the egui spike on this machine and writes target/spike-data/results.json.
# Usage (Windows PowerShell 5.1 or PowerShell 7): powershell -NoProfile -File measure.ps1
# Steps: release build, binary size, 3 x --bench (median), 30 s idle sample with the 1 MB file,
# clean release build time, cargo deny, dependency count. Every app it starts is stopped again.
$ErrorActionPreference = 'Stop'
Set-Location $PSScriptRoot

$BenchRuns = 3
$SettleSeconds = 5
$IdleSeconds = 30
$MiB = 1024 * 1024
$exe = Join-Path $PSScriptRoot 'target\release\amate-spike.exe'
$dataDir = Join-Path $PSScriptRoot 'target\spike-data'
$smallFile = Join-Path $dataDir 'spike-1mb.cpp'

function Invoke-Native([string]$Label, [scriptblock]$Command) {
    & $Command
    if ($LASTEXITCODE -ne 0) { throw "$Label failed with exit code $LASTEXITCODE" }
}

function Get-Median([double[]]$Values) {
    $sorted = @($Values | Sort-Object)
    if ($sorted.Count -eq 0) { return $null }
    $middle = [int][math]::Floor($sorted.Count / 2)
    if ($sorted.Count % 2 -eq 1) { return $sorted[$middle] }
    return ($sorted[$middle - 1] + $sorted[$middle]) / 2
}

# Flattens nested JSON objects into "a.b.c" = number pairs, so every metric gets a median.
function Get-NumericLeaves($Object, [string]$Prefix, [hashtable]$Into) {
    foreach ($property in $Object.PSObject.Properties) {
        $name = if ($Prefix) { "$Prefix.$($property.Name)" } else { $property.Name }
        $value = $property.Value
        if ($value -is [System.Management.Automation.PSCustomObject]) {
            Get-NumericLeaves $value $name $Into
        } elseif ($value -is [double] -or $value -is [int] -or $value -is [long] -or $value -is [decimal]) {
            if (-not $Into.ContainsKey($name)) { $Into[$name] = New-Object System.Collections.Generic.List[double] }
            $Into[$name].Add([double]$value)
        }
    }
}

$results = [ordered]@{
    date = (Get-Date).ToString('s')
    machine = [ordered]@{
        os = (Get-CimInstance Win32_OperatingSystem).Caption
        cpu = (Get-CimInstance Win32_Processor | Select-Object -First 1).Name
        logical_cpus = [Environment]::ProcessorCount
        gpus = @(Get-CimInstance Win32_VideoController | ForEach-Object { "{0} @ {1} Hz" -f $_.Name, $_.CurrentRefreshRate })
    }
    rustc = (& rustc -V)
}

Write-Host '== Release build'
Invoke-Native 'cargo build' { cargo build --release --locked }
$results.binary_bytes = (Get-Item $exe).Length
$results.binary_mb = [math]::Round($results.binary_bytes / $MiB, 2)

Write-Host "== Bench x $BenchRuns (the window must stay visible)"
$runs = @()
for ($run = 1; $run -le $BenchRuns; $run++) {
    $line = & $exe --bench
    if ($LASTEXITCODE -ne 0) { throw "bench run $run failed with exit code $LASTEXITCODE" }
    $runs += ($line | Select-Object -Last 1 | ConvertFrom-Json)
    Write-Host ("   run {0}: startup {1} ms, key p99 {2} ms" -f $run, $runs[-1].startup_to_first_frame_ms, $runs[-1].typing_highlighted.key_latency_ms.p99)
}
$leaves = @{}
foreach ($run in $runs) { Get-NumericLeaves $run '' $leaves }
$medians = [ordered]@{}
foreach ($name in ($leaves.Keys | Sort-Object)) { $medians[$name] = Get-Median $leaves[$name].ToArray() }
$results.bench_median = $medians
$results.bench_runs = $runs

Write-Host "== Idle sample: 1 MB file, $SettleSeconds s settle, $IdleSeconds s sampling"
$app = Start-Process -FilePath $exe -ArgumentList ('"{0}"' -f $smallFile) -PassThru -NoNewWindow
try {
    Start-Sleep -Seconds $SettleSeconds
    $app.Refresh()
    $cpuStart = $app.TotalProcessorTime
    $private = New-Object System.Collections.Generic.List[double]
    $workingSet = New-Object System.Collections.Generic.List[double]
    for ($second = 0; $second -lt $IdleSeconds; $second++) {
        $app.Refresh()
        $private.Add($app.PrivateMemorySize64 / $MiB)
        $workingSet.Add($app.WorkingSet64 / $MiB)
        Start-Sleep -Seconds 1
    }
    $app.Refresh()
    $results.idle_1mb = [ordered]@{
        private_mb_median = [math]::Round((Get-Median $private.ToArray()), 1)
        private_mb_max = [math]::Round(($private | Measure-Object -Maximum).Maximum, 1)
        working_set_mb_median = [math]::Round((Get-Median $workingSet.ToArray()), 1)
        working_set_mb_max = [math]::Round(($workingSet | Measure-Object -Maximum).Maximum, 1)
        cpu_ms_over_window = [math]::Round(($app.TotalProcessorTime - $cpuStart).TotalMilliseconds, 1)
        window_seconds = $IdleSeconds
    }
} finally {
    Stop-Process -Id $app.Id -Force -ErrorAction SilentlyContinue
    Wait-Process -Id $app.Id -Timeout 10 -ErrorAction SilentlyContinue
}

Write-Host '== Dependency count (cargo tree, normal + build edges)'
function Get-DependencyCount([string]$Target) {
    $lines = & cargo tree --locked --prefix none --target $Target -e normal,build --format '{p}'
    if ($LASTEXITCODE -ne 0) { throw "cargo tree --target $Target failed" }
    $unique = $lines | ForEach-Object { ($_ -replace ' \(\*\)$', '').Trim() } | Where-Object { $_ } | Sort-Object -Unique
    return ($unique | Measure-Object).Count - 1  # minus amate-spike itself
}
$results.dependencies = [ordered]@{
    windows = Get-DependencyCount 'x86_64-pc-windows-msvc'
    linux = Get-DependencyCount 'x86_64-unknown-linux-gnu'
    macos = Get-DependencyCount 'aarch64-apple-darwin'
    all_targets = Get-DependencyCount 'all'
}

Write-Host '== cargo deny check'
$denyText = cmd /c 'cargo deny check 2>&1'
$results.cargo_deny = [ordered]@{
    exit_code = $LASTEXITCODE
    summary = (($denyText | Where-Object { $_ -match '^(advisories|bans|licenses|sources) ' }) -join '; ')
}
$denyLog = $denyText -join [Environment]::NewLine

Write-Host '== Clean release build (cargo clean, then cargo build --release --locked)'
Invoke-Native 'cargo clean' { cargo clean }
$watch = [System.Diagnostics.Stopwatch]::StartNew()
Invoke-Native 'clean cargo build' { cargo build --release --locked }
$results.clean_release_build_seconds = [math]::Round($watch.Elapsed.TotalSeconds, 1)

New-Item -ItemType Directory -Force -Path $dataDir | Out-Null
Set-Content -Path (Join-Path $dataDir 'cargo-deny.txt') -Value $denyLog -Encoding UTF8
$json = $results | ConvertTo-Json -Depth 10
Set-Content -Path (Join-Path $dataDir 'results.json') -Value $json -Encoding UTF8
Write-Host "== Wrote $(Join-Path $dataDir 'results.json')"
$leftover = Get-Process amate-spike -ErrorAction SilentlyContinue | Where-Object { $_.Path -eq $exe }
if ($leftover) { $leftover | Stop-Process -Force; Write-Host 'Stopped a leftover amate-spike process.' }
