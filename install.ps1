# tablo installer — Windows. Pulls a build from GitHub Releases, verifies its
# published SHA-256 checksum, then runs the installer. Fails closed: a missing or
# mismatched checksum aborts and removes the download before anything runs.
#   irm https://raw.githubusercontent.com/unravel-team/tablo/tablo-v2.2.2/install.ps1 | iex
# Pin a specific version with $env:TABLO_VERSION = 'tablo-vX.Y.Z'.
$ErrorActionPreference = 'Stop'

$repo = 'unravel-team/tablo'
$api  = if ($env:TABLO_VERSION) {
  "https://api.github.com/repos/$repo/releases/tags/$($env:TABLO_VERSION)"
} else {
  "https://api.github.com/repos/$repo/releases/latest"
}

function Say($m) { Write-Host "==> $m" -ForegroundColor Green }

Say "Fetching the tablo release..."
$release = Invoke-RestMethod -Uri $api -Headers @{ 'User-Agent' = 'tablo-install' }

# Prefer the NSIS setup .exe; fall back to the .msi.
$asset = $release.assets | Where-Object { $_.name -match '-setup\.exe$' } | Select-Object -First 1
if (-not $asset) { $asset = $release.assets | Where-Object { $_.name -match '\.msi$' } | Select-Object -First 1 }
if (-not $asset) { throw "no Windows installer (.exe / .msi) in that release" }

$out = Join-Path $env:TEMP $asset.name
Say "Downloading $($asset.name)..."
Invoke-WebRequest -Uri $asset.browser_download_url -OutFile $out -UseBasicParsing

# Verify the published SHA-256 (fail closed) before running anything. The
# "<file>.sha256" is published next to each release asset.
Say "Verifying checksum..."
try {
  $resp = Invoke-WebRequest -Uri "$($asset.browser_download_url).sha256" -UseBasicParsing
} catch {
  Remove-Item $out -Force -ErrorAction SilentlyContinue
  throw "no published checksum for $($asset.name) - refusing to install"
}
# GitHub serves the .sha256 as octet-stream, so .Content arrives as byte[] —
# decode it, or .Trim() explodes on System.Byte.
$sumText = if ($resp.Content -is [byte[]]) {
  [System.Text.Encoding]::ASCII.GetString($resp.Content)
} else {
  [string]$resp.Content
}
$expected = ($sumText.Trim() -split '\s+')[0].ToLower()
$actual   = (Get-FileHash -Path $out -Algorithm SHA256).Hash.ToLower()
if (-not $expected -or $expected -ne $actual) {
  Remove-Item $out -Force -ErrorAction SilentlyContinue
  throw "checksum mismatch for $($asset.name) (expected '$expected', got '$actual') - aborting"
}

# Clear the "mark of the web" so SmartScreen doesn't prompt. The download was
# already checksum-verified above; on an unsigned installer the SmartScreen prompt
# is only a click-through, not an integrity check, so this removes a speed bump
# we've already covered.
Unblock-File -Path $out

Say "Launching the installer..."
Start-Process -FilePath $out -Wait
Say "Done. tablo updates itself automatically from here on."
