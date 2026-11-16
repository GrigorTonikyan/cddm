# CDDM Standalone Windows PowerShell Installer
# Usage: irm https://git.gt-web-dev.com/gt-dev/cddm/raw/branch/main/packaging/install.ps1 | iex

$ErrorActionPreference = 'Stop'
$GiteaHost = if ($env:CDDM_GITEA_HOST) { $env:CDDM_GITEA_HOST } else { "git.gt-web-dev.com" }
$Repo = if ($env:CDDM_REPO) { $env:CDDM_REPO } else { "gt-dev/cddm" }
$InstallDir = if ($env:CDDM_INSTALL_DIR) { $env:CDDM_INSTALL_DIR } else { "$env:USERPROFILE\.cddm\bin" }
$Version = if ($env:CDDM_VERSION) { $env:CDDM_VERSION } else { "latest" }

Write-Host "--> Initializing CDDM Windows Installer (host: $GiteaHost)..." -ForegroundColor Cyan

$Arch = if ([IntPtr]::Size -eq 8) {
    if ($env:PROCESSOR_ARCHITECTURE -match 'ARM64') { 'aarch64' } else { 'x86_64' }
} else {
    throw "Error: 32-bit Windows architectures are not supported."
}

$Headers = @{ "User-Agent" = "CDDM-Installer" }
if ($Version -eq 'latest') {
    $ApiUrl = "https://$GiteaHost/api/v1/repos/$Repo/releases/latest"
} else {
    $NormalizedTag = if ($Version.StartsWith("v")) { $Version } else { "v$Version" }
    $ApiUrl = "https://$GiteaHost/api/v1/repos/$Repo/releases/tags/$NormalizedTag"
}

try {
    $ReleaseData = Invoke-RestMethod -Uri $ApiUrl -Headers $Headers
} catch {
    throw "Failed to query release metadata from ${ApiUrl}: $($_.Exception.Message)"
}

if (-not $ReleaseData -or -not $ReleaseData.tag_name) {
    throw "Release metadata from ${ApiUrl} did not contain a valid tag_name. Refusing silent version downgrade."
}

$ReleaseTag = [string]$ReleaseData.tag_name
$PreferredCandidates = @(
    "cddm-$ReleaseTag-$Arch-pc-windows-msvc.zip",
    "cddm-$ReleaseTag-$Arch-pc-windows-gnu.zip"
)

$AvailableAssets = @()
if ($ReleaseData.assets) {
    $AvailableAssets = @($ReleaseData.assets | ForEach-Object { [string]$_.name })
}

$ZipName = $null
if ($AvailableAssets.Count -gt 0) {
    foreach ($Candidate in $PreferredCandidates) {
        if ($AvailableAssets -contains $Candidate) {
            $ZipName = $Candidate
            break
        }
    }
    if (-not $ZipName) {
        $AssetList = $AvailableAssets -join ", "
        throw "No Windows ($Arch) archive found in release $ReleaseTag. Available assets: $AssetList"
    }
} else {
    $ZipName = $PreferredCandidates[0]
}

$BaseDownloadUrl = "https://$GiteaHost/$Repo/releases/download/$ReleaseTag"
$DownloadUrl = "$BaseDownloadUrl/$ZipName"
$ChecksumsUrl = "$BaseDownloadUrl/SHA256SUMS.txt"

$TempDir = Join-Path ([System.IO.Path]::GetTempPath()) ("cddm-install-" + [System.IO.Path]::GetRandomFileName())
New-Item -ItemType Directory -Path $TempDir -Force | Out-Null

try {
    $TempZip = Join-Path $TempDir $ZipName
    $TempSums = Join-Path $TempDir "SHA256SUMS.txt"

    Write-Host "--> Downloading CDDM $ReleaseTag ($ZipName)..." -ForegroundColor Yellow
    Invoke-WebRequest -Uri $DownloadUrl -OutFile $TempZip -Headers $Headers

    Write-Host "--> Verifying SHA-256 cryptographic checksum via SHA256SUMS.txt..." -ForegroundColor Yellow
    Invoke-WebRequest -Uri $ChecksumsUrl -OutFile $TempSums -Headers $Headers

    $ExpectedHash = $null
    foreach ($Line in (Get-Content -Path $TempSums)) {
        $Trimmed = $Line.Trim()
        if ($Trimmed -match '^([0-9a-fA-F]{64})\s+\*?(.+)$') {
            $EntryHash = $Matches[1].ToLowerInvariant()
            $EntryFile = [System.IO.Path]::GetFileName($Matches[2].Trim())
            if ($EntryFile -eq $ZipName) {
                $ExpectedHash = $EntryHash
                break
            }
        }
    }

    if (-not $ExpectedHash) {
        throw "SHA256SUMS.txt for release $ReleaseTag does not contain an entry for $ZipName."
    }
    if ($ExpectedHash -match '^0+$') {
        throw "SHA256SUMS.txt contains a forbidden all-zero placeholder digest for $ZipName."
    }

    $ActualHash = (Get-FileHash -Path $TempZip -Algorithm SHA256).Hash.ToLowerInvariant()
    if ($ActualHash -ne $ExpectedHash) {
        throw "SHA-256 checksum mismatch for ${ZipName}: expected $ExpectedHash, got $ActualHash."
    }
    Write-Host "--> [PASS] Verified SHA-256 digest: $ActualHash" -ForegroundColor Green

    if (-not (Test-Path $InstallDir)) {
        New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    }

    Write-Host "--> Extracting binaries to $InstallDir..." -ForegroundColor Yellow
    Expand-Archive -Path $TempZip -DestinationPath $InstallDir -Force
} finally {
    if (Test-Path $TempDir) {
        Remove-Item -Path $TempDir -Recurse -Force -ErrorAction SilentlyContinue
    }
}

$UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if (-not $UserPath) {
    $UserPath = ""
}
if ($UserPath -notmatch [regex]::Escape($InstallDir)) {
    $NewUserPath = if ($UserPath) { "$InstallDir;$UserPath" } else { $InstallDir }
    [Environment]::SetEnvironmentVariable("PATH", $NewUserPath, "User")
    $env:PATH = "$InstallDir;$env:PATH"
    Write-Host "--> Added $InstallDir to User PATH." -ForegroundColor Green
}

Write-Host "[SUCCESS] CDDM $ReleaseTag installed to $InstallDir" -ForegroundColor Green
Write-Host "Run 'cddm --help' in a new terminal window to get started." -ForegroundColor Cyan
