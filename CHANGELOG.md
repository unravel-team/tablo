# Changelog

All notable changes to tablo are recorded here.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and
tablo uses [semantic versioning](https://semver.org/)

**This file is published.** The release workflow extracts the section matching
the tag being built and uses it as the GitHub release body, which `tauri-action`
also copies into `latest.json` — so it becomes the release notes every installed
copy of tablo sees when it checks for updates. A tag with no matching section
here fails the release before anything is built. Write the entry as you merge,
not at tag time.

## [2.2.2] - 2026-09-21

2.2.2 stops tablo from using the GPU while it is sitting idle. On laptops this
was enough to keep the fans up with the cat asleep.

### Added

- **Software rendering option (Windows).** On a hybrid graphics laptop whose
  discrete GPU still refuses to idle, set `"softwareRendering": true` in
  `config.json` to run tablo's windows without GPU acceleration. Off by default.

### Fixed

- **Hidden windows no longer render.** The panel drew its session list off
  screen from the moment tablo launched, even if you never opened it, and kept
  animating there for the whole session. Hidden windows are now suspended, which
  cut tablo's idle GPU use by about 4x on the Windows laptop this was reported
  from.
- **The cat's glow is cheaper to animate.** The breathing glow redrew on every
  frame for as long as tablo ran. It now updates a few times a second, which
  looks the same and leaves the GPU idle in between.

## [2.2.1] - 2026-08-17

2.2.1 fixes the cat's caution pip, which only ever counted tool approvals and
left a context-window warning invisible on the widget. Everything else below
shipped in 2.2.0 and is repeated here for anyone updating straight from 2.1.x.

### Added

- **OpenCode support.** Tablo watches **OpenCode** sessions alongside Claude
  Code and Codex — same avatar count, same panel and dashboard, with an
  `opencode` tag on each row. On by default (Settings → Watch OpenCode). Jump
  and tool approvals aren't available for OpenCode.
- **Running subagents, per session.** When a session fans out, its row grows a
  foldable **"N agents · <age>"** line listing each running agent and how long
  it's been going. It clears as they return, and doesn't inflate the cat's
  count — a fan-out is still one session you're watching.
- **Settings in the menu-bar menu.** The tray menu now carries a **Settings**
  item next to Show/Hide widget and Dashboard.

### Changed

- **Tool approvals only prompt in the default permission mode.** In
  accept-edits/auto, plan, or bypass mode you've already said you don't want to
  be asked per call, so tablo steps aside instead of asking you again. The
  `mode :` badge on each row shows which sessions will prompt.

### Fixed

- **The caution pip now covers context warnings, not just approvals.** The red
  triangle at the cat's bottom-right counted pending approvals only, so once the
  cat had calmed down from its startle, a session past the warn threshold left
  no mark on the widget. It now counts every session wanting attention and stays
  up until the alarm clears.
- **The shocked cat no longer stays shocked.** An alarm used to pin the startled
  sprite for as long as it lasted, which could be all afternoon. The cat now
  startles for a few seconds, then goes back to trotting or sleeping while the
  count pips carry the alert. A new permission request re-startles it.

## [2.2.0] - 2026-08-15

2.2.0 adds a third agent — **OpenCode** — shows the **subagents** each session
has running, stops tool approvals from asking you twice when you've already
picked a permission mode that says don't ask, and calms the cat down a few
seconds after an alarm instead of leaving it shocked all day.

### Added

- **OpenCode support.** Tablo now watches **OpenCode** sessions
  (`~/.local/share/opencode`) alongside Claude Code and Codex — same avatar
  count, same panel and dashboard, with a neutral `opencode` tag on each row.
  On by default; turn it off under Settings → Watch OpenCode. The context
  window comes from the models.dev catalog OpenCode caches locally, so it's
  exact for essentially every model it can run; a model missing from that
  catalog shows a greyed meter rather than a guessed percentage. OpenCode keeps
  its sessions in SQLite rather than JSONL, so tablo reads the database
  read-only and only re-reads when it actually changes. Jump and tool approvals
  are **not** available for OpenCode — it exposes neither a process registry nor
  a hook file to carry them.
- **Running subagents, per session.** When a session fans out, its row grows a
  foldable **"N agents · <age>"** line in both the panel and the dashboard,
  listing each running agent by its description with its own elapsed time. The
  line disappears as the agents return. Fold state is shared across both
  windows. Subagents are read off the session's own transcript — no extra files
  are opened, so a 7-way fan-out costs nothing extra — and they don't inflate
  the cat's count: a fan-out is still one session you're watching. Agents that
  keep running past the end of their turn stay listed until they actually report
  back. OpenCode's `task` child sessions appear the same way.
- **Settings in the menu-bar menu.** The tray menu now carries a **Settings**
  item next to Show/Hide widget and Dashboard, so preferences are one click away
  whether or not the dashboard is already up — it opens straight to the settings
  pane, and switches an already-open dashboard over to it.

### Changed

- **Tool approvals only prompt in the default permission mode.** If a session is
  in accept-edits/auto, plan, or bypass mode, you've already said you don't want
  to be asked per call — tablo now steps aside and lets Claude Code apply that
  mode itself, instead of prompting you again in the widget. It defers rather
  than approving, so tablo can never be more permissive than the mode you
  picked, and the `mode :` badge on each row tells you exactly which sessions
  will prompt.

### Fixed

- **The shocked cat no longer stays shocked.** An alarm — context past the warn
  threshold or a pending tool approval — used to pin the startled sprite for as
  long as the condition lasted, which could be all afternoon. Now the cat
  startles for a few seconds, then goes back to trotting or sleeping; the count
  pips carry the ongoing alert, with the permission count sharpened into a
  hard-edged triangle at the cat's bottom-right. A new permission request
  re-startles the cat, so a fresh ask never slips by silently.

## [2.1.5] - 2026-08-09

2.1.5 fixes sessions that lingered after they ended, a cat that could start up
off-screen, and context meters that showed a percentage they had guessed.

### Added

**2.1.0 changelog:**
- **What's new after an update.** Auto-update used to restart tablo without
  saying anything. It now announces itself once, and the dashboard shows this
  release's notes until you dismiss them.
- **Notification sound.** The waiting toast now plays a soft chime. **On by
  default** — existing installs will start making a sound after updating. Turn
  it off under Settings → Notification sound.
- **Cat animations toggle.** Settings → Cat animations. Off holds the cat on a
  still pose with a steady glow; the state colour still tells you what's
  happening.
- **Automatic updates setting.** On by default. Turning it off doesn't stop
  tablo looking for releases — it stops it applying them: you get a
  notification and a one-tap Install button in Settings instead of a silent
  background upgrade.

### Changed

- The dashboard window is built when you first open it rather than at launch,
  which drops idle memory by roughly one webview process.

### Fixed

- Sessions no longer look active when they aren't. Recency is now judged by the
  last real conversational line rather than the transcript's file time, so the
  housekeeping writes Claude Code makes to idle sessions stop resurrecting them
  — and a session whose Claude process has exited drops off immediately instead
  of aging out.
- The context meter no longer presents a guessed percentage as fact. When the
  window size can't be determined, the bar greys out and reads "irresolvable"
  instead of dividing by a default limit; the raw token count stays visible, and
  the meter fills in the moment the real window is known. A guessed window can
  no longer colour the meter or alarm the cat.
- A saved avatar position that no longer lands on any connected screen — after a
  migration or unplugging a monitor — falls back to the default corner instead
  of leaving the cat invisible.
- The panel and the waiting toast no longer slide underneath the macOS Dock.
  Both are now clamped to the screen's work area instead of the full display,
  so their lower rows stay visible and clickable.
- Jump to session resolves from the session registry and re-resolves at click
  time, so it no longer targets a window that has since moved or closed.

## [2.1.4] - 2026-08-02

2.1.4 hotfixes the windows installer verification step and bugged dashboard.

### Added

**2.1.0 changelog:**
- **What's new after an update.** Auto-update used to restart tablo without
  saying anything. It now announces itself once, and the dashboard shows this
  release's notes until you dismiss them.
- **Notification sound.** The waiting toast now plays a soft chime. **On by
  default** — existing installs will start making a sound after updating. Turn
  it off under Settings → Notification sound.
- **Cat animations toggle.** Settings → Cat animations. Off holds the cat on a
  still pose with a steady glow; the state colour still tells you what's
  happening.
- **Automatic updates setting.** On by default. Turning it off doesn't stop
  tablo looking for releases — it stops it applying them: you get a
  notification and a one-tap Install button in Settings instead of a silent
  background upgrade.

### Changed

- The dashboard window is built when you first open it rather than at launch,
  which drops idle memory by roughly one webview process.

### Fixed

- The panel and the waiting toast no longer slide underneath the macOS Dock.
  Both are now clamped to the screen's work area instead of the full display,
  so their lower rows stay visible and clickable.
- Jump to session resolves from the session registry and re-resolves at click
  time, so it no longer targets a window that has since moved or closed.
- Windows releases now publish their `.sha256` checksum sidecars. v2.0.0
  shipped without them because the workflow's path matcher didn't handle
  backslash separators, which broke checksum verification in `install.ps1`.

## [2.1.3] - 2026-08-02

2.1.3 hotfixes the windows installer verification step.

### Added

**2.1.0 changelog:**
- **What's new after an update.** Auto-update used to restart tablo without
  saying anything. It now announces itself once, and the dashboard shows this
  release's notes until you dismiss them.
- **Notification sound.** The waiting toast now plays a soft chime. **On by
  default** — existing installs will start making a sound after updating. Turn
  it off under Settings → Notification sound.
- **Cat animations toggle.** Settings → Cat animations. Off holds the cat on a
  still pose with a steady glow; the state colour still tells you what's
  happening.
- **Automatic updates setting.** On by default. Turning it off doesn't stop
  tablo looking for releases — it stops it applying them: you get a
  notification and a one-tap Install button in Settings instead of a silent
  background upgrade.

### Changed

- The dashboard window is built when you first open it rather than at launch,
  which drops idle memory by roughly one webview process.

### Fixed

- The panel and the waiting toast no longer slide underneath the macOS Dock.
  Both are now clamped to the screen's work area instead of the full display,
  so their lower rows stay visible and clickable.
- Jump to session resolves from the session registry and re-resolves at click
  time, so it no longer targets a window that has since moved or closed.
- Windows releases now publish their `.sha256` checksum sidecars. v2.0.0
  shipped without them because the workflow's path matcher didn't handle
  backslash separators, which broke checksum verification in `install.ps1`.

## [2.1.1] - 2026-08-01

2.1.1 hotfixes a flaky jump condition and false fable context window

### Added

**2.1.0 changelog:**
- **What's new after an update.** Auto-update used to restart tablo without
  saying anything. It now announces itself once, and the dashboard shows this
  release's notes until you dismiss them.
- **Notification sound.** The waiting toast now plays a soft chime. **On by
  default** — existing installs will start making a sound after updating. Turn
  it off under Settings → Notification sound.
- **Cat animations toggle.** Settings → Cat animations. Off holds the cat on a
  still pose with a steady glow; the state colour still tells you what's
  happening.
- **Automatic updates setting.** On by default. Turning it off doesn't stop
  tablo looking for releases — it stops it applying them: you get a
  notification and a one-tap Install button in Settings instead of a silent
  background upgrade.

### Changed

- The dashboard window is built when you first open it rather than at launch,
  which drops idle memory by roughly one webview process.

### Fixed

- The panel and the waiting toast no longer slide underneath the macOS Dock.
  Both are now clamped to the screen's work area instead of the full display,
  so their lower rows stay visible and clickable.
- Jump to session resolves from the session registry and re-resolves at click
  time, so it no longer targets a window that has since moved or closed.
- Windows releases now publish their `.sha256` checksum sidecars. v2.0.0
  shipped without them because the workflow's path matcher didn't handle
  backslash separators, which broke checksum verification in `install.ps1`.

## [2.1.0] - 2026-08-01

### Added

- **What's new after an update.** Auto-update used to restart tablo without
  saying anything. It now announces itself once, and the dashboard shows this
  release's notes until you dismiss them.
- **Notification sound.** The waiting toast now plays a soft chime. **On by
  default** — existing installs will start making a sound after updating. Turn
  it off under Settings → Notification sound.
- **Cat animations toggle.** Settings → Cat animations. Off holds the cat on a
  still pose with a steady glow; the state colour still tells you what's
  happening.
- **Automatic updates setting.** On by default. Turning it off doesn't stop
  tablo looking for releases — it stops it applying them: you get a
  notification and a one-tap Install button in Settings instead of a silent
  background upgrade.

### Changed

- The dashboard window is built when you first open it rather than at launch,
  which drops idle memory by roughly one webview process.

### Fixed

- The panel and the waiting toast no longer slide underneath the macOS Dock.
  Both are now clamped to the screen's work area instead of the full display,
  so their lower rows stay visible and clickable.
- Jump to session resolves from the session registry and re-resolves at click
  time, so it no longer targets a window that has since moved or closed.
- Windows releases now publish their `.sha256` checksum sidecars. v2.0.0
  shipped without them because the workflow's path matcher didn't handle
  backslash separators, which broke checksum verification in `install.ps1`.

## [2.0.0] - 2026-07-27

### Added

- Anonymous usage stats so active installs can be counted — no session data,
  paths, prompts, or tokens. Opt out under Settings.
- A one-time notice on first launch explaining the locator hook that tablo
  installs into Claude Code for "jump to session".
- unravel.tech branding across the app and site.

### Security

- The loopback IPC server used by the approvals hook is now authenticated and
  bounded.
- `install.sh` / `install.ps1` verify the installer's checksum and fail closed
  on a mismatch, and no longer bypass OS gatekeepers.
- Fonts are self-hosted and HTML output is guarded against injection.
- Release workflow actions are pinned to commit SHAs and the signing job runs
  with least privilege behind a protected environment.

### Changed

- The website moved from Netlify to Cloudflare Pages.

## [1.0.1] - 2026-07-11

### Added

- The Working and Waiting groups collapse and expand, and the state is shared
  between the panel and the dashboard.

### Fixed

- Settings that only apply on macOS are hidden on Windows and Linux instead of
  showing as dead toggles.
- Typography moved to the mono family for session data.

## [1.0.0] - 2026-07-10

### Added

- AeroSpace support (macOS): tablo follows the focused workspace.

### Fixed

- Improved de-focus behaviour when dismissing the panel.

## [0.5.2] - 2026-07-10

### Fixed

- Jump to session was flaky; marked experimental while it stabilised.

## [0.5.1] - 2026-07-10

### Added

- Credits in the dashboard.

### Fixed

- Jump to session under tmux, and when the editor is Zed.

## [0.5.0] - 2026-07-10

First public release.
