param(
    [string]$Pages = "tests/pages/plain.html,tests/pages/blog.html,https://example.com,https://news.ycombinator.com,https://en.wikipedia.org/wiki/Rust_(programming_language)",
    [int]$Settle = 12
)

$root = Split-Path (Split-Path $PSScriptRoot)
$firerust = Join-Path $root "target\release\fr-shell.exe"
$firefox = "C:\Program Files\Mozilla Firefox\firefox.exe"
$chrome = "C:\Program Files\Google\Chrome\Application\chrome.exe"
$edge = "C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe"
$marker = "firerust-compare"
$profileRoot = Join-Path $env:TEMP $marker

function Resolve-Address($page) {
    if ($page -match '^https?://') { return $page }
    return ([System.Uri](Resolve-Path (Join-Path $root $page)).Path).AbsoluteUri
}

function Get-Tree($rootIds) {
    $all = Get-CimInstance Win32_Process | Select-Object ProcessId, ParentProcessId
    $found = New-Object System.Collections.Generic.HashSet[int]
    foreach ($id in $rootIds) { [void]$found.Add([int]$id) }
    $changed = $true
    while ($changed) {
        $changed = $false
        foreach ($p in $all) {
            if ($found.Contains([int]$p.ParentProcessId) -and -not $found.Contains([int]$p.ProcessId)) {
                [void]$found.Add([int]$p.ProcessId)
                $changed = $true
            }
        }
    }
    return $found
}

function Measure-Tree($ids) {
    $private = 0.0
    $count = 0
    foreach ($row in Get-CimInstance Win32_PerfFormattedData_PerfProc_Process) {
        if ($ids.Contains([int]$row.IDProcess)) {
            $private += $row.WorkingSetPrivate
            $count++
        }
    }
    return [pscustomobject]@{ Procs = $count; PrivateMB = [math]::Round($private / 1MB, 1) }
}

function Stop-Tree($ids) {
    foreach ($id in $ids) { Stop-Process -Id $id -Force -ErrorAction SilentlyContinue }
}

function Fresh-Profile($name) {
    $path = Join-Path $profileRoot $name
    if (Test-Path $path) { Remove-Item $path -Recurse -Force }
    New-Item -ItemType Directory -Path $path | Out-Null
    return $path
}

function Run-Firerust($page) {
    $process = Start-Process $firerust -ArgumentList @($page) -PassThru -WorkingDirectory $root
    Start-Sleep -Seconds $Settle
    $ids = Get-Tree @($process.Id)
    $result = Measure-Tree $ids
    Stop-Tree $ids
    return $result
}

function Run-Firefox($address) {
    $profile = Fresh-Profile "firefox"
    $prefs = @(
        'user_pref("browser.aboutwelcome.enabled", false);',
        'user_pref("browser.shell.checkDefaultBrowser", false);',
        'user_pref("browser.newtabpage.enabled", false);',
        'user_pref("datareporting.policy.dataSubmissionEnabled", false);',
        'user_pref("browser.sessionstore.resume_from_crash", false);'
    )
    Set-Content -Path (Join-Path $profile "user.js") -Value $prefs -Encoding ascii
    $null = Start-Process $firefox -ArgumentList @("-no-remote", "-new-instance", "-profile", $profile, $address) -PassThru
    Start-Sleep -Seconds ($Settle + 8)
    $roots = @(Get-CimInstance Win32_Process -Filter "Name='firefox.exe'" |
        Where-Object { $_.CommandLine -like "*$marker*" } |
        ForEach-Object { $_.ProcessId })
    $ids = Get-Tree $roots
    $result = Measure-Tree $ids
    Stop-Tree $ids
    Start-Sleep -Seconds 3
    return $result
}

function Run-Chromium($exe, $process, $name, $address) {
    $profile = Fresh-Profile $name
    $null = Start-Process $exe -ArgumentList @("--user-data-dir=$profile", "--no-first-run", "--no-default-browser-check", "--disable-sync", $address) -PassThru
    Start-Sleep -Seconds ($Settle + 8)
    $roots = @(Get-CimInstance Win32_Process -Filter "Name='$process'" |
        Where-Object { $_.CommandLine -like "*$marker*" } |
        ForEach-Object { $_.ProcessId })
    $ids = Get-Tree $roots
    $result = Measure-Tree $ids
    Stop-Tree $ids
    Start-Sleep -Seconds 3
    return $result
}

function Folder-Size($path) {
    if (-not (Test-Path $path)) { return $null }
    $bytes = (Get-ChildItem $path -Recurse -File -ErrorAction SilentlyContinue | Measure-Object Length -Sum).Sum
    return [math]::Round($bytes / 1MB, 0)
}

$rows = @()
foreach ($page in $Pages.Split(",")) {
    $address = Resolve-Address $page
    $entries = @(
        @{ Name = "firerust"; Run = { Run-Firerust $page } },
        @{ Name = "firefox"; Run = { Run-Firefox $address } },
        @{ Name = "chrome"; Run = { Run-Chromium $chrome "chrome.exe" "chrome" $address } },
        @{ Name = "edge"; Run = { Run-Chromium $edge "msedge.exe" "edge" $address } }
    )
    foreach ($entry in $entries) {
        $result = & $entry.Run
        $rows += [pscustomobject]@{ Page = $page; Browser = $entry.Name; Procs = $result.Procs; PrivateMB = $result.PrivateMB }
    }
}

$rows | Format-Table -AutoSize | Out-String -Width 200

$sizes = @(
    [pscustomobject]@{ Browser = "firerust"; InstallMB = [math]::Round((Get-Item $firerust).Length / 1MB, 0) },
    [pscustomobject]@{ Browser = "firefox"; InstallMB = Folder-Size (Split-Path $firefox) },
    [pscustomobject]@{ Browser = "chrome"; InstallMB = Folder-Size (Split-Path (Split-Path $chrome)) },
    [pscustomobject]@{ Browser = "edge"; InstallMB = Folder-Size (Split-Path (Split-Path $edge)) }
)
$sizes | Format-Table -AutoSize | Out-String -Width 200
