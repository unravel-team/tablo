#!/usr/bin/env bash
# tablo installer — macOS & Linux. Pulls a build from GitHub Releases, verifies
# its published SHA-256 checksum, and installs it. Fails closed: a missing or
# mismatched checksum aborts before anything is installed or made executable.
#   curl -fsSL https://raw.githubusercontent.com/unravel-team/tablo/tablo-v2.2.2/install.sh | bash
# Pin a specific version with TABLO_VERSION=tablo-vX.Y.Z; otherwise the latest
# release is used. No build toolchain required.
set -euo pipefail

REPO="unravel-team/tablo"
API_BASE="https://api.github.com/repos/${REPO}/releases"
API="${API_BASE}/latest"
[ -n "${TABLO_VERSION:-}" ] && API="${API_BASE}/tags/${TABLO_VERSION}"

say() { printf '\033[1;32m==>\033[0m %s\n' "$1"; }
die() { printf '\033[1;31merror:\033[0m %s\n' "$1" >&2; exit 1; }

# First asset download URL whose filename matches the given regex.
asset_url() {
  curl -fsSL "$API" \
    | grep -o '"browser_download_url": *"[^"]*"' \
    | sed 's/.*"\(https[^"]*\)"/\1/' \
    | grep -iE "$1" \
    | head -1
}

# SHA-256 of a file as a bare lowercase hex string, using whichever tool exists.
sha256_of() {
  if command -v shasum >/dev/null 2>&1; then
    shasum -a 256 "$1" | awk '{print $1}'
  elif command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$1" | awk '{print $1}'
  else
    die "no sha256 tool (shasum/sha256sum) found to verify the download"
  fi
}

# Verify $1 against the "<file>.sha256" published next to its release asset ($2).
# Aborts on a missing checksum (fail closed) or any mismatch.
verify_sha() {
  local file="$1" url="$2" sums expected actual
  say "Verifying checksum…"
  sums="$(curl -fsSL "${url}.sha256")" \
    || die "no published checksum for $(basename "$url") — refusing to install"
  expected="$(printf '%s\n' "$sums" | awk '{print $1}' | head -1)"
  [ -n "$expected" ] || die "empty checksum for $(basename "$url")"
  actual="$(sha256_of "$file")"
  [ "$expected" = "$actual" ] \
    || die "checksum mismatch for $(basename "$url") (expected $expected, got $actual) — aborting"
}

os="$(uname -s)"
case "$os" in
  Darwin)
    say "Fetching the tablo release…"
    url="$(asset_url '\.dmg$')" || true
    [ -n "${url:-}" ] || die "no macOS .dmg in that release"
    tmp="$(mktemp -d)"
    trap 'hdiutil detach "$tmp/mnt" -quiet 2>/dev/null || true; rm -rf "$tmp"' EXIT
    say "Downloading $(basename "$url")…"
    curl -fSL --progress-bar "$url" -o "$tmp/tablo.dmg"
    verify_sha "$tmp/tablo.dmg" "$url"
    mkdir -p "$tmp/mnt"
    hdiutil attach "$tmp/tablo.dmg" -nobrowse -quiet -mountpoint "$tmp/mnt"
    app="$(/usr/bin/find "$tmp/mnt" -maxdepth 1 -name '*.app' | head -1)"
    [ -n "$app" ] || die "no .app inside the dmg"
    say "Installing to /Applications…"
    rm -rf "/Applications/$(basename "$app")"
    cp -R "$app" /Applications/
    # tablo isn't notarized yet; strip the quarantine flag so it opens without the
    # Gatekeeper prompt. The download was already checksum-verified above — the
    # Gatekeeper prompt on an unsigned app is only a click-through, not an
    # integrity check, so this trades a speed bump we've already covered.
    xattr -dr com.apple.quarantine "/Applications/$(basename "$app")" 2>/dev/null || true
    say "Done — launch tablo from /Applications or Spotlight."
    ;;
  Linux)
    say "Fetching the tablo release…"
    url="$(asset_url '\.AppImage$')" || true
    [ -n "${url:-}" ] || die "no Linux .AppImage in that release"
    tmp="$(mktemp -d)"
    trap 'rm -rf "$tmp"' EXIT
    say "Downloading $(basename "$url")…"
    curl -fSL --progress-bar "$url" -o "$tmp/tablo.AppImage"
    verify_sha "$tmp/tablo.AppImage" "$url"
    dest="${HOME}/.local/bin"
    mkdir -p "$dest"
    out="$dest/tablo.AppImage"
    # Only mark executable after verification — no runnable artifact on failure.
    install -m 0755 "$tmp/tablo.AppImage" "$out"
    say "Installed to $out"
    case ":$PATH:" in
      *":$dest:"*) : ;;
      *) say "Tip: add $dest to your PATH to run 'tablo.AppImage' from anywhere." ;;
    esac
    ;;
  *)
    die "unsupported OS: $os — on Windows use install.ps1"
    ;;
esac
