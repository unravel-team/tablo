//! Persistent user configuration — the single source of truth for every tunable
//! value (thresholds, context-window sizes, active window, timings).
//! No domain value is hardcoded elsewhere; the scanner and UI read from here.
//!
//! Stored as JSON in the Tauri app-config dir. All fields have defaults so an
//! older/partial config file still loads cleanly (Open Question #10).

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", default)]
pub struct Config {
    // ---- avatar ----
    /// Saved avatar window position (logical pixels). `None` until first placed.
    pub avatar_x: Option<i32>,
    pub avatar_y: Option<i32>,

    // ---- session activity ----
    /// A transcript modified within this many seconds counts as an active
    /// session (Open Question #3).
    pub active_window_secs: u64,
    /// A typed prompt optimistically shows "thinking…", but a Ctrl+C *before*
    /// the assistant responds writes no marker — the transcript just goes quiet.
    /// If we're still awaiting the first assistant line after this many minutes
    /// of silence, the prompt is treated as cancelled and Tablo hands back to the
    /// user. Kept long by default because a genuinely long-thinking turn is
    /// indistinguishable on disk (it also writes nothing until it responds).
    /// Minimum 1 minute (enforced at use).
    pub cancel_grace_mins: u64,
    /// How long a *finished* (waiting-on-you) session lingers in the panel before
    /// it clears, in minutes. Independent of (and shorter than) `active_window_secs`
    /// so finished sessions declutter quickly while a genuinely long-thinking one
    /// still gets the full active window and never vanishes mid-think. The session
    /// isn't lost — it reappears the instant the user submits again. Min 1 minute.
    pub clear_waiting_mins: u64,

    // ---- Codex support ----
    /// Also watch OpenAI Codex CLI sessions (`~/.codex/sessions/**/*.jsonl`)
    /// alongside Claude Code. Default on; a user who only wants Claude Code can
    /// turn it off in Settings.
    pub watch_codex: bool,
    /// Fallback context window for a Codex session before its first `token_count`
    /// / `task_started` reports the real `model_context_window`. The live value
    /// almost always arrives first, so this rarely applies.
    pub codex_context_limit: u64,

    // ---- OpenCode support ----
    /// Also watch OpenCode sessions (`~/.local/share/opencode/opencode.db`).
    /// Default on; turn it off in Settings to watch Claude Code only.
    pub watch_opencode: bool,
    /// Fallback context window for an OpenCode session whose model isn't in the
    /// cached models.dev catalog. Only a display floor — a session using it is
    /// marked unresolved, so it never shows a percentage or raises an alarm.
    pub opencode_context_limit: u64,

    // ---- AeroSpace (macOS tiling WM) ----
    /// Follow the focused AeroSpace workspace so the widget survives workspace
    /// switches (AeroSpace emulates its own workspaces and ignores the native
    /// "on all Spaces" pin). Default on; a no-op — and cost-free — when AeroSpace
    /// isn't running, so it only ever does anything for AeroSpace users. The
    /// Settings toggle appears only once AeroSpace is detected.
    pub aerospace_follow: bool,

    // ---- context window sizing (Open Question #4) ----
    /// Denominator used when a session's window can't be determined and its
    /// usage is below `standard_context_limit`. Set to `extended_context_limit`
    /// if you primarily run the 1M beta.
    pub default_context_limit: u64,
    /// The standard window. Also the threshold: usage above this implies the
    /// extended window (a standard session compacts before it could get there).
    pub standard_context_limit: u64,
    /// The extended (beta) window.
    pub extended_context_limit: u64,
    /// Model-string markers (lowercased, substring match) that force the
    /// extended window when present.
    pub extended_window_markers: Vec<String>,

    // ---- alarm thresholds (percent) ----
    /// Context-fill warning line. 60 is the required alarm line.
    pub warn_pct: f64,
    /// Context-fill critical line.
    pub crit_pct: f64,

    // ---- tuning ----
    /// On first sight of a transcript larger than this, skip to a bounded tail
    /// (we only need recent messages + the file's end).
    pub initial_tail_cap_bytes: u64,

    // ---- permissions (Phase 4) ----
    /// Loopback port the PreToolUse hook posts to for approve/deny decisions.
    pub permission_port: u16,
    /// Tools the hook intercepts for human approval — used to build the
    /// settings.json matcher. Read-only tools are intentionally left out so
    /// they never pay the round-trip. `["*"]` would intercept everything.
    pub intercept_tools: Vec<String>,
    /// Per-hook timeout written into settings.json (seconds), and the window the
    /// server waits for a decision. A long window (approvals fail **closed** —
    /// deny — if it elapses). Capped by Claude Code's own max hook timeout.
    pub hook_timeout_secs: u64,

    // ---- misc ----
    /// Fire a one-time OS notification when a session first crosses `warn_pct`
    /// (Open Question #5, default off for Phase 1).
    pub notify_on_warn: bool,
    /// Fire an OS notification when a new tool approval is requested (Phase 4).
    pub notify_on_permission: bool,
    /// Send anonymous usage pings (Aptabase) so active users can be counted. Only
    pub telemetry_enabled: bool,
    /// Install new releases automatically in the background. On by default.
    /// Turning it off doesn't stop Tablo *looking* for releases — it stops it
    /// applying them: a found update is surfaced for a one-tap install instead
    /// of downloaded, installed, and restarted behind the user's back.
    pub auto_update: bool,
    /// "dark" (hero) or "light".
    pub theme: String,
    /// Global hotkey (Tauri accelerator syntax, e.g. "Control+Command+P") that
    /// toggles the panel from anywhere. Empty string disables it.
    pub panel_shortcut: String,
    /// Whether the global panel hotkey is registered. On by default; the Settings
    /// toggle flips it live (register / unregister) without losing the keystroke.
    pub panel_shortcut_enabled: bool,
    /// App version whose release notes the user has seen. Empty on a fresh
    /// install — recorded silently, so a new user isn't shown notes for a build
    /// they never ran.
    pub last_seen_version: String,
    /// One-time guard: whether "jump to session" has been auto-enabled once. Set
    /// true after the first launch wires it in, so a later user-disable sticks
    /// and we never silently re-enable it behind their back.
    pub locate_default_applied: bool,
    /// Windows only: render the webviews without GPU acceleration. Off by
    /// default; for hybrid-graphics laptops whose discrete GPU never idles.
    pub software_rendering: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            avatar_x: None,
            avatar_y: None,
            active_window_secs: 900,
            cancel_grace_mins: 3,
            clear_waiting_mins: 10,
            watch_codex: true,
            codex_context_limit: 256_000,
            watch_opencode: true,
            opencode_context_limit: 200_000,
            aerospace_follow: true,
            default_context_limit: 200_000,
            standard_context_limit: 200_000,
            extended_context_limit: 1_000_000,
            extended_window_markers: vec!["[1m]".into(), "-1m".into()],
            warn_pct: 60.0,
            crit_pct: 85.0,
            initial_tail_cap_bytes: 2_000_000,
            permission_port: 8577,
            intercept_tools: vec![
                "Bash".into(),
                "Write".into(),
                "Edit".into(),
                "MultiEdit".into(),
                "NotebookEdit".into(),
            ],
            hook_timeout_secs: 600,
            notify_on_warn: false,
            notify_on_permission: true,
            telemetry_enabled: true,
            auto_update: true,
            theme: "dark".into(),
            panel_shortcut: "Control+Command+P".into(),
            panel_shortcut_enabled: true,
            last_seen_version: String::new(),
            locate_default_applied: false,
            software_rendering: false,
        }
    }
}

impl Config {
    pub fn path(config_dir: &Path) -> PathBuf {
        config_dir.join("config.json")
    }

    pub fn load(config_dir: &Path) -> Self {
        let p = Self::path(config_dir);
        match std::fs::read_to_string(&p) {
            Ok(s) => serde_json::from_str(&s).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    pub fn save(&self, config_dir: &Path) {
        let _ = std::fs::create_dir_all(config_dir);
        if let Ok(s) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(Self::path(config_dir), s);
        }
    }
}
