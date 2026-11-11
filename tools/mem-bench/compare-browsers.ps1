param(
    [string]$Pages = "tests/pages/plain.html,tests/pages/blog.html,https://example.com,https://news.ycombinator.com,https://en.wikipedia.org/wiki/Rust_(programming_language)",
    [string]$Tabs = "https://example.com,https://news.ycombinator.com,https://en.wikipedia.org/wiki/Rust_(programming_language),https://lobste.rs,https://www.rust-lang.org,https://doc.rust-lang.org/book/,https://text.npr.org,https://developer.mozilla.org/en-US/",
    [switch]$Together,
    [int]$Settle = 12,
    [int]$TabSettle = 40,
    [string]$OutFile = "",
    [string]$FirerustExe = "",
    [string]$FirerustCommit = ""
)

$root = Split-Path (Split-Path $PSScriptRoot)
$firerust = if ($FirerustExe -ne "") { $FirerustExe } else { Join-Path $root "target\release\fr-shell.exe" }
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

function Run-Firerust($addresses, $wait) {
    $process = Start-Process $firerust -ArgumentList $addresses -PassThru -WorkingDirectory $root
    Start-Sleep -Seconds $wait
    $ids = Get-Tree @($process.Id)
    $result = Measure-Tree $ids
    Stop-Tree $ids
    return $result
}

function Run-Firefox($addresses, $wait) {
    $profile = Fresh-Profile "firefox"
    $prefs = @(
        'user_pref("browser.aboutwelcome.enabled", false);',
        'user_pref("browser.shell.checkDefaultBrowser", false);',
        'user_pref("browser.newtabpage.enabled", false);',
        'user_pref("datareporting.policy.dataSubmissionEnabled", false);',
        'user_pref("browser.sessionstore.resume_from_crash", false);'
    )
    Set-Content -Path (Join-Path $profile "user.js") -Value $prefs -Encoding ascii
    $null = Start-Process $firefox -ArgumentList (@("-no-remote", "-new-instance", "-profile", $profile) + $addresses) -PassThru
    Start-Sleep -Seconds ($wait + 8)
    $roots = @(Get-CimInstance Win32_Process -Filter "Name='firefox.exe'" |
        Where-Object { $_.CommandLine -like "*$marker*" } |
        ForEach-Object { $_.ProcessId })
    $ids = Get-Tree $roots
    $result = Measure-Tree $ids
    Stop-Tree $ids
    Start-Sleep -Seconds 3
    return $result
}

function Run-Chromium($exe, $process, $name, $addresses, $wait) {
    $profile = Fresh-Profile $name
    $arguments = @("--user-data-dir=$profile", "--no-first-run", "--no-default-browser-check", "--disable-sync") + $addresses
    $null = Start-Process $exe -ArgumentList $arguments -PassThru
    Start-Sleep -Seconds ($wait + 8)
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

function Version-Of($path) {
    if (-not (Test-Path $path)) { return "not installed" }
    return (Get-Item $path).VersionInfo.ProductVersion
}

function Measure-Case($label, $items, $wait) {
    $addresses = @($items | ForEach-Object { Resolve-Address $_ })
    $rows = @()
    $rows += [pscustomobject]@{ Case = $label; Browser = "firerust"; Procs = 0; PrivateMB = 0 }
    $first = Run-Firerust $addresses $wait
    $rows[0].Procs = $first.Procs
    $rows[0].PrivateMB = $first.PrivateMB
    $others = @(
        @{ Name = "firefox"; Run = { Run-Firefox $addresses $wait } },
        @{ Name = "chrome"; Run = { Run-Chromium $chrome "chrome.exe" "chrome" $addresses $wait } },
        @{ Name = "edge"; Run = { Run-Chromium $edge "msedge.exe" "edge" $addresses $wait } }
    )
    foreach ($entry in $others) {
        $result = & $entry.Run
        $rows += [pscustomobject]@{ Case = $label; Browser = $entry.Name; Procs = $result.Procs; PrivateMB = $result.PrivateMB }
    }
    return $rows
}

$commit = if ($FirerustCommit -ne "") { $FirerustCommit } else { (git -C $root rev-parse --short HEAD 2>$null) }
$header = @(
    "firerust memory comparison",
    "date: $(Get-Date -Format 'yyyy-MM-dd HH:mm')",
    "firerust commit: $commit",
    "firefox: $(Version-Of $firefox)",
    "chrome: $(Version-Of $chrome)",
    "edge: $(Version-Of $edge)",
    "metric: sum of private working set of every process of the browser, MB",
    "each browser is started with a fresh profile and measured after the settle time",
    ""
)

$rows = @()
if ($Together) {
    $items = $Tabs.Split(",")
    $rows += Measure-Case "$($items.Count) real tabs" $items $TabSettle
    $header += "tabs: $($items -join ' ')"
    $header += "settle: $TabSettle s (+8 s for the other browsers)"
} else {
    foreach ($page in $Pages.Split(",")) {
        $rows += Measure-Case $page @($page) $Settle
    }
    $header += "settle: $Settle s (+8 s for the other browsers)"
}

$sizes = @(
    [pscustomobject]@{ Browser = "firerust"; InstallMB = [math]::Round((Get-Item $firerust).Length / 1MB, 0) },
    [pscustomobject]@{ Browser = "firefox"; InstallMB = Folder-Size (Split-Path $firefox) },
    [pscustomobject]@{ Browser = "chrome"; InstallMB = Folder-Size (Split-Path (Split-Path $chrome)) },
    [pscustomobject]@{ Browser = "edge"; InstallMB = Folder-Size (Split-Path (Split-Path $edge)) }
)

$report = @($header) + @(($rows | Format-Table -AutoSize | Out-String -Width 200), ($sizes | Format-Table -AutoSize | Out-String -Width 200))
$report | ForEach-Object { Write-Output $_ }
if ($OutFile -ne "") {
    $report | Out-File -FilePath $OutFile -Encoding utf8
}
