# tablo

A tiny floating cat that watches your coding agents — **Claude Code, OpenAI
Codex, and OpenCode** — from a corner of your screen. Tablo reflects live activity through a
small animated cat and a soft colored glow: it **sleeps and breathes sage** when
idle, **trots and pulses amber** while agents are working, and turns **shocked
and flushes coral** when a session needs your input or its context window crosses
the danger line. **Tap Tablo** to open a panel with a live context meter per
session; from there, open a fuller **dashboard** with settings.

Built with **Tauri 2** (Rust) + **SvelteKit / Svelte 5**. macOS is the primary
platform today.

## Install

Prebuilt installers for macOS, Windows, and Linux live on the
[Releases](https://github.com/unravel-team/tablo/releases) page. The one-liners
below download the latest build, **verify its published SHA-256 checksum**, and
install it quietly — aborting if the checksum is missing or doesn't match. The
URLs are pinned to a release tag (not a moving branch); if you'd rather read the
script first, open that raw URL or grab `install.sh` / `install.ps1` and run it
locally.

**macOS / Linux**

```bash
curl -fsSL https://raw.githubusercontent.com/unravel-team/tablo/tablo-v2.2.2/install.sh | bash
```

**Windows** (PowerShell)

```powershell
irm https://raw.githubusercontent.com/unravel-team/tablo/tablo-v2.2.2/install.ps1 | iex
```

tablo isn't code-signed / notarized yet, so after verifying the download the
installer clears the OS quarantine flag (macOS) / mark-of-the-web (Windows) so it
opens without a Gatekeeper / SmartScreen prompt. Once installed, tablo keeps
itself up to date — it checks for new releases and updates in the background.

## The four surfaces

- **Avatar** — a tiny (126×134) transparent, always-on-top window that is
  *click-through everywhere except the cat*. Drag to reposition (remembered
  between launches); tap to toggle the panel. The cat's animation, glow, and
  count pips are a pure function of your aggregate live state.
- **Panel** — opens on tap, anchored by the avatar. Grouped, live session
  meters. Dismisses on a second tap, on tap-away/blur, or with **Esc**.
- **Dashboard** — a larger, resizable window with per-session gauges, a live
  terminal-style activity preview, permission approvals, and a settings pane.
- **Toast** — a small, quiet notification that flies out of the avatar when a
  working session finishes and starts waiting on you (toggleable).

## The cat

The avatar is a small animated cat whose state is legible at a glance, with
precedence **alarmed → running → idle**:

| State | Cat | Glow | Meaning |
|-------|-----|------|---------|
| **idle** | curled asleep | sage, slow breathe | nothing active, or every session is just waiting on you |
| **running** | trotting | amber, quicker pulse | at least one session is actively working |
| **alarmed** | ears-back, wide-eyed | coral, urgent flush | a session crossed the context warning line, **or** a tool call is waiting for your approval |

Transitions are animated (the cat wakes and settles rather than snapping), and
escalation is glow-only — no jitter. Up to three **count pips** stack off the
top-right, each shown only when non-zero:

- **coral** — sessions with a pending permission request,
- **sage** — sessions waiting on you,
- **amber** — sessions actively working.

## Live context meters

Tablo tails each agent's session transcripts and computes every active session's
context-window occupancy in real time, pushing updates to all windows. A session
counts as *active* if its transcript changed within a configurable window
(default 15 min), and finished sessions clear from the panel after their own
(shorter) timer, reappearing the instant you prompt them again. Compaction
snap-backs animate down rather than flicker.

In the **panel**, sessions group by state, most urgent first:

1. **Context window warning** (pinned top) — any session past the warning line,
   under a red `> N%` header (`N` is your configured limit).
2. **Permission Request** — tool calls awaiting approve/deny.
3. **Waiting** — handed back to you.
4. **Working** — actively running.

Each row shows the project, an optional AI-generated session title, a live
one-line activity ("editing scanner.rs", "running cargo check", "waiting for
you") with a colored status dot, a read-only permission-mode badge, the agent
source (**claude** / **codex** / **opencode**), and a segmented-LED context bar with the raw
token count — colored sage → amber → coral by threshold. Filter chips toggle the
Waiting / Working groups when more than one session is active.

## Three agents, one cat

Tablo watches **Claude Code** (`~/.claude/projects/`), **OpenAI Codex**
(`~/.codex/sessions/`), and **OpenCode** (`~/.local/share/opencode/`) sessions
side by side in the same avatar count, panel, and dashboard. A small neutral
**`claude` / `codex` / `opencode` tag** on each row marks which agent it came
from. Codex and OpenCode are on by default and can each be switched off in
Settings.

For Claude sessions, Tablo auto-detects each session's context window (the
standard window vs. the 1M-token beta) from on-disk signals — no manual toggle;
the per-session `used / limit` readout shows what was detected. Codex reports its
window directly, so it's always exact. OpenCode's window comes from the
models.dev catalog it caches locally, which covers essentially every model it can
run; a model missing from that catalog shows a greyed meter rather than a guess.

OpenCode stores its sessions in SQLite rather than JSONL, so Tablo reads them
read-only and re-queries only when the database actually changes. Its
`task`-tool child sessions appear as subagents on the parent row, the same as
Claude Code's. Jump and tool approvals are **not** available for OpenCode — it
exposes neither a process registry nor a hook file to carry them.

## Permissions — approve / deny

Tablo can gate Claude Code tool calls behind a tap:

- A `PreToolUse` hook sends each intercepted tool call to Tablo and blocks.
- Tablo forces the avatar to **alarmed**, raises a count pip, and surfaces the
  request in the panel's **Permission Request** group and on the dashboard.
- You tap **Approve** / **Deny**; the decision releases or blocks the tool.
- Only mutating tools are intercepted by default
  (`Bash`/`Write`/`Edit`/`MultiEdit`/`NotebookEdit`); read-only tools never pay
  the round-trip.
- **Only the default permission mode prompts.** If the session is in
  accept-edits, plan, or bypass mode, you've already said you don't want to be
  asked per call — Tablo steps aside and Claude Code applies that mode itself.
  It defers rather than approving, so Tablo can never be more permissive than
  the mode you picked. The `mode :` badge on each row tells you which sessions
  will prompt.
- **Fail-closed:** if you never decide within the timeout (~10 min), it
  **denies**. If Tablo is *down*, the hook fails fast and Claude Code proceeds
  normally — it never hangs.

Approvals are off until you enable them in Settings, and only then is Claude
Code's hook config touched.

## Jump to session *(experimental)*

Every session card — and every toast — has a **jump** button that focuses the
terminal a session is running in. Sessions self-report their location through a
passive hook; jump then switches tmux to the exact pane (if any) and brings the
host terminal to the front.

Pinpointing the *exact tab* needs a terminal that's scriptable — either a
per-tab tty (Terminal.app / iTerm2) or a surface API (Ghostty). Support on macOS:

| Terminal | No tmux | Inside tmux |
|----------|---------|-------------|
| Terminal.app | Exact window/tab | Exact pane |
| iTerm2 | Exact tab \* | Exact pane |
| Ghostty | Exact tab \*\* | Exact pane |
| WezTerm, kitty, Alacritty, Warp, Hyper, Tabby | App only \*\*\* | Exact pane |

\* The iTerm2 path exists but is currently unverified.

\*\* Ghostty exposes no per-tab tty, so Tablo matches the exact surface by its
working directory + tab title (Claude/Codex title the tab with the session name)
through Ghostty's AppleScript dictionary.

\*\*\* Brings the app to the front. With a single tab that's the right one; with
multiple tabs and no tmux it lands on the current tab — these terminals expose no
scriptable per-tab tty. **Inside tmux, jump is always exact**: tmux selects the
pane and Tablo just foregrounds the app.

Across platforms, the tmux pane-switch works everywhere (including WSL); the GUI
window-raise is macOS-only, so on Linux and Windows jump still switches the tmux
pane but doesn't raise the window, and Wayland stays a no-op.

**Claude and Codex jump.** The jump logic is shared and agent-agnostic. Claude
jump is on by default; **Codex jump is opt-in** in Settings (it installs a hook
into `~/.codex/hooks.json`, which Codex asks you to trust once). The two paths
are fully independent. **OpenCode sessions show no jump button** — OpenCode
records neither the session's process nor a hook file, so there is nothing to
locate its terminal by.

## Notifications

When a working session finishes and starts waiting on you, a gentle toast slides
out of the avatar showing `project · title` and a **jump** button. Its on-screen
time is configurable, and the whole feature can be switched off. Toasts
overshadow rather than stack.

## Dashboard & Settings

The dashboard is a larger view with per-session context gauges, a live
terminal-style activity preview (`$ live preview`) of what each agent is doing,
and the same live approvals. A compact headline shows *active · waiting ·
projects*. A gear opens an in-window **Settings** pane:

- **Tool approvals** — turn Claude Code approve/deny on or off (prompts only in
  the default permission mode).
- **Jump to Claude session** *(experimental)* — enable/disable the jump buttons
  for Claude sessions.
- **Watch Codex** / **Jump to Codex session** *(experimental)* — watch Codex CLI
  sessions (on by default), and enable jump for them (opt-in; Codex asks you to
  trust the hook once).
- **Watch OpenCode** — watch OpenCode sessions (on by default).
- **Panel shortcut** — summon the panel from anywhere with **Ctrl+Cmd+P**.
- **Follow AeroSpace workspace** — keep the widget with you across AeroSpace
  workspace switches (appears only when AeroSpace is detected — see below).
- **Context window limit** — the warning threshold, `1`–`100`% (default `60`).
  Applies live everywhere.
- **Cancelled-prompt grace** — how long Tablo waits before treating a Ctrl-C'd
  prompt as done rather than thinking (minutes).
- **Clear waiting sessions** — how long a finished session lingers in the panel
  before it clears (minutes); it returns the moment you prompt it again.
- **Waiting notifications** — toggle the toast and set its on-screen time.
- **Anonymous usage stats** — send an anonymous ping so active users can be
  counted (on by default; opt out here). No session data — see **Privacy** below.
- **Theme** — dark / light.

## Themes

Warm **dark** (the hero mode) and **light**. The toggle lives in Settings and
syncs instantly across every window; your choice is remembered.

## Quiet by default

Tablo runs as a macOS **Accessory** app — hidden from the Dock and Cmd+Tab, with
a small **menu-bar icon** to show/hide the widget, open the dashboard, or quit. It
surfaces in Cmd+Tab only while the dashboard is open, then drops back to the
corner. No emoji anywhere; status is shown with the cat, colored glows, and
LED-style dots.

## Privacy

Tablo reads your local Claude Code / Codex / OpenCode sessions and never sends their
contents anywhere. The one thing it reports off your machine is **anonymous usage
stats** (via [Aptabase](https://aptabase.com)) — a launch/heartbeat ping so active
users can be counted. No session content, file paths, prompts, or tokens; only
that Tablo ran, plus your OS and app version. It's on by default and opts out with
one toggle in **Settings**.

## AeroSpace

Tablo stays put across native macOS **Spaces** (Mission Control desktops) — the
avatar, panel, and toast are pinned to every Space.
[AeroSpace](https://github.com/nikitabobko/AeroSpace), the i3-like tiling window
manager, emulates its *own* workspaces and slides windows off-screen on a switch,
so that native pin doesn't apply. Tablo handles it by **following**: when it
detects AeroSpace running, it moves the cat onto whichever workspace you focus, so
the widget is always with you. It's on by default and cost-free when AeroSpace
isn't running — the **Follow AeroSpace workspace** toggle in Settings appears only
once AeroSpace is detected.

Big shoutout to [@nikitabobko](https://github.com/nikitabobko) for
[AeroSpace](https://github.com/nikitabobko/AeroSpace) — a superb tiling window
manager for macOS.

## Develop

```bash
bun install
bun run tauri dev      # launch the app (the cat appears bottom-right)
bun run tauri build    # produce a bundle
```

Config is stored at the Tauri app-config dir — on macOS,
`~/Library/Application Support/com.projektdreamscape.tablo/config.json` — and
holds the avatar position, thresholds, timers, theme, and toggles.

On Windows, `"softwareRendering": true` in that file makes the webviews render
without GPU acceleration from the next launch — for hybrid-graphics laptops
where the discrete GPU otherwise never drops back to an idle power state.
