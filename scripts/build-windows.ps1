$ErrorActionPreference = "Stop"
$project = Split-Path -Parent $PSScriptRoot
Set-Location $project
if (Get-Process -Name PiStreamTerminal -ErrorAction SilentlyContinue) { throw "Close PiStreamTerminal before rebuilding." }
# GMP/MPFR requires the GNU toolchain and MSYS2 build tools on Windows.
foreach ($tool in @("gcc", "make", "m4", "windres")) {
    if (-not (Get-Command $tool -ErrorAction SilentlyContinue)) { throw "Missing $tool. Install MSYS2 UCRT64/MinGW build tools and add them to PATH." }
}
$target = "x86_64-pc-windows-gnu"
rustup target add $target
if ($LASTEXITCODE -ne 0) { throw "Cannot install the Windows GNU target." }
cargo build --release --locked --target $target --bin PiStreamTerminal
if ($LASTEXITCODE -ne 0) { throw "Cargo build failed." }
$output = Join-Path $project "dist/windows-x64"
New-Item -ItemType Directory -Path $output -Force | Out-Null
$targetDir = if ($env:CARGO_TARGET_DIR) { $env:CARGO_TARGET_DIR } else { Join-Path $project "target" }
Copy-Item (Join-Path $targetDir "$target/release/PiStreamTerminal.exe") $output -Force
$compilerDir = Split-Path -Parent (Get-Command gcc).Source
foreach ($dll in @("libgcc_s_seh-1.dll", "libstdc++-6.dll", "libwinpthread-1.dll")) {
    $path = Join-Path $compilerDir $dll
    if (Test-Path $path) { Copy-Item $path $output -Force }
}
Copy-Item README.md $output -Force
Compress-Archive -Path "$output/*" -DestinationPath "dist/PiStreamTerminal-windows-x64.zip" -Force
Write-Output "Built: $output/PiStreamTerminal.exe"
