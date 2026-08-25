# CDDM Standalone Windows PowerShell Installer
# Usage: irm https://raw.githubusercontent.com/GrigorTonikyan/cddm/main/packaging/install.ps1 | iex

$ErrorActionPreference = 'Stop'
$Repo = "GrigorTonikyan/cddm"
$InstallDir = if ($env:CDDM_INSTALL_DIR) { $env:CDDM_INSTALL_DIR } else { "$env:USERPROFILE\.cddm\bin" }
$Version = if ($env:CDDM_VERSION) { $env:CDDM_VERSION } else { "latest" }

Write-Host "--> Initializing CDDM Windows Installer..." -ForegroundColor Cyan

$Arch = if ([IntPtr]::Size -eq 8) {
    if ($env:PROCESSOR_ARCHITECTURE -match 'ARM64') { 'aarch64' } else { 'x86_64' }
} else {
    Write-Error "Error: 32-bit architectures are not supported."
    exit 1
}

$Target = "$Arch-pc-windows-msvc"

if ($Version -eq 'latest') {
    try {
        $ReleaseData = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases/latest" -Headers @{ "User-Agent" = "CDDM-Installer" }
        $ReleaseTag = $ReleaseData.tag_name
    } catch {
        $ReleaseTag = "v1.7.0"
    }
} else {
    $ReleaseTag = $Version
}

$ZipName = "cddm-$ReleaseTag-$Target.zip"
$DownloadUrl = "https://github.com/$Repo/releases/download/$ReleaseTag/$ZipName"
$TempZip = Join-Path $env:TEMP $ZipName

Write-Host "--> Downloading CDDM $ReleaseTag for $Target..." -ForegroundColor Yellow
Invoke-WebRequest -Uri $DownloadUrl -OutFile $TempZip

if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

Write-Host "--> Extracting binaries to $InstallDir..." -ForegroundColor Yellow
Expand-Archive -Path $TempZip -DestinationPath $InstallDir -Force
Remove-Item -Path $TempZip -Force

$UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($UserPath -notmatch [regex]::Escape($InstallDir)) {
    [Environment]::SetEnvironmentVariable("PATH", "$InstallDir;$UserPath", "User")
    $env:PATH = "$InstallDir;$env:PATH"
    Write-Host "--> Added $InstallDir to User PATH." -ForegroundColor Green
}

Write-Host "[SUCCESS] CDDM successfully installed to $InstallDir" -ForegroundColor Green
Write-Host "Run 'cddm --help' in a new terminal window to get started." -ForegroundColor Cyan
