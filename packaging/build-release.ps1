$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $PSScriptRoot
Set-Location $root

cargo build --release -p fr-shell
if ($LASTEXITCODE -ne 0) {
    throw "the release build failed"
}

$match = Select-String -Path "Cargo.toml" -Pattern '^version = "(.+)"' | Select-Object -First 1
$version = $match.Matches[0].Groups[1].Value
$name = "firerust-$version-windows-x64"
$stage = Join-Path $root "dist\$name"

if (Test-Path $stage) {
    Remove-Item -Recurse -Force $stage
}
New-Item -ItemType Directory -Force $stage | Out-Null

Copy-Item "target\release\fr-shell.exe" (Join-Path $stage "firerust.exe")
Copy-Item "packaging\README.txt" (Join-Path $stage "README.txt")
Copy-Item "packaging\SUPPORTED.txt" (Join-Path $stage "SUPPORTED.txt")
Copy-Item "crates\fr-icons\icons\LICENSE.txt" (Join-Path $stage "LICENSE-bootstrap-icons.txt")

$archive = Join-Path $root "dist\$name.zip"
if (Test-Path $archive) {
    Remove-Item -Force $archive
}
Compress-Archive -Path (Join-Path $stage "*") -DestinationPath $archive

$size = [math]::Round((Get-Item $archive).Length / 1MB, 1)
Write-Output "built $archive ($size MB)"
