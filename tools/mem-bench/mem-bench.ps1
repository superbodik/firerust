param(
    [string]$Pages = "tests/pages/plain.html,tests/pages/blog.html,tests/pages/tables.html,tests/pages/site/long.html,https://example.com",
    [switch]$Together,
    [int]$Settle = 12,
    [string]$Firefox = "C:\Program Files\Mozilla Firefox\firefox.exe"
)

$pageList = $Pages.Split(",")
$root = Split-Path (Split-Path $PSScriptRoot)
$firerust = Join-Path $root "target\release\fr-shell.exe"
$profileDir = Join-Path $env:TEMP "firerust-bench-profile"

function Get-Descendants($rootIds) {
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

function Measure-Ids($ids) {
    $private = 0.0
    $working = 0.0
    $count = 0
    foreach ($row in Get-CimInstance Win32_PerfFormattedData_PerfProc_Process) {
        if ($ids.Contains([int]$row.IDProcess)) {
            $private += $row.WorkingSetPrivate
            $working += $row.WorkingSet
            $count++
        }
    }
    return [pscustomobject]@{
        Procs = $count
        PrivateMB = [math]::Round($private / 1MB, 1)
        WorkingSetMB = [math]::Round($working / 1MB, 1)
    }
}

function Stop-Ids($ids) {
    foreach ($id in $ids) { Stop-Process -Id $id -Force -ErrorAction SilentlyContinue }
}

function Resolve-Address($page) {
    if ($page -match '^https?://') { return $page }
    return ([System.Uri](Resolve-Path (Join-Path $root $page)).Path).AbsoluteUri
}

function Write-FirefoxProfile {
    if (Test-Path $profileDir) { Remove-Item $profileDir -Recurse -Force }
    New-Item -ItemType Directory -Path $profileDir | Out-Null
    $prefs = @(
        'user_pref("browser.aboutwelcome.enabled", false);',
        'user_pref("browser.shell.checkDefaultBrowser", false);',
        'user_pref("startup.homepage_welcome_url", "");',
        'user_pref("startup.homepage_welcome_url.additional", "");',
        'user_pref("browser.startup.homepage_override.mstone", "ignore");',
        'user_pref("browser.newtabpage.enabled", false);',
        'user_pref("datareporting.policy.dataSubmissionEnabled", false);',
        'user_pref("toolkit.telemetry.reportingpolicy.firstRun", false);',
        'user_pref("browser.sessionstore.resume_from_crash", false);'
    )
    Set-Content -Path (Join-Path $profileDir "user.js") -Value $prefs -Encoding ascii
}

function Measure-Firerust($pageList, $label) {
    $process = Start-Process $firerust -ArgumentList $pageList -PassThru -WorkingDirectory $root
    Start-Sleep -Seconds $Settle
    $ids = Get-Descendants @($process.Id)
    $result = Measure-Ids $ids
    Stop-Ids $ids
    return [pscustomobject]@{ Browser = "firerust"; Case = $label; Procs = $result.Procs; PrivateMB = $result.PrivateMB; WorkingSetMB = $result.WorkingSetMB }
}

function Measure-Firefox($addresses, $label) {
    Write-FirefoxProfile
    $arguments = @("-no-remote", "-new-instance", "-profile", $profileDir) + $addresses
    $null = Start-Process $Firefox -ArgumentList $arguments -PassThru
    Start-Sleep -Seconds ($Settle + 8)
    $roots = @(Get-CimInstance Win32_Process -Filter "Name='firefox.exe'" |
        Where-Object { $_.CommandLine -like "*firerust-bench-profile*" } |
        ForEach-Object { $_.ProcessId })
    $ids = Get-Descendants $roots
    $result = Measure-Ids $ids
    Stop-Ids $ids
    Start-Sleep -Seconds 3
    return [pscustomobject]@{ Browser = "firefox"; Case = $label; Procs = $result.Procs; PrivateMB = $result.PrivateMB; WorkingSetMB = $result.WorkingSetMB }
}

$results = @()
if ($Together) {
    $label = "$($pageList.Count) tabs"
    $results += Measure-Firerust $pageList $label
    $results += Measure-Firefox ($pageList | ForEach-Object { Resolve-Address $_ }) $label
} else {
    foreach ($page in $pageList) {
        $results += Measure-Firerust @($page) $page
        $results += Measure-Firefox @(Resolve-Address $page) $page
    }
}
$results | Format-Table -AutoSize | Out-String -Width 200
