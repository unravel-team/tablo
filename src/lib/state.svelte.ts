// Shared reactive store (Svelte 5 runes). Every window fetches the current
// snapshot on mount, then subscribes to live `state-update` events.

import { EMPTY_SNAPSHOT, LOADING_CONFIG, type Snapshot } from "./types";
import { getConfig, getSnapshot, onState, onTheme, setTheme } from "./bridge";

export const store = $state({
  snap: EMPTY_SNAPSHOT as Snapshot,
  config: LOADING_CONFIG,
  ready: false,
});

export function applyTheme(theme: string) {
  document.documentElement.setAttribute("data-theme", theme);
}

// Set the app-wide theme: apply locally for instant feedback, persist + rebroadcast
// via Rust so the other windows follow. No-op if already on that theme.
export function setThemeMode(theme: "dark" | "light") {
  if (store.config.theme === theme) return;
  store.config.theme = theme;
  applyTheme(theme);
  setTheme(theme);
}

export function toggleTheme() {
  setThemeMode(store.config.theme === "dark" ? "light" : "dark");
}

// Ticks only while `needed()` holds, so an idle surface has no interval at all.
export function liveClock(needed: () => boolean) {
  const clock = $state({ now: Date.now() });
  $effect(() => {
    if (!needed()) return;
    clock.now = Date.now();
    const timer = setInterval(() => (clock.now = Date.now()), 1000);
    return () => clearInterval(timer);
  });
  return clock;
}

let started = false;

export async function initStore() {
  if (started) return;
  started = true;
  try {
    store.config = await getConfig();
  } catch {
    /* keep defaults */
  }
  applyTheme(store.config.theme);
  try {
    store.snap = await getSnapshot();
  } catch {
    /* keep empty */
  }
  store.ready = true;
  onState((s) => {
    store.snap = s;
  }).catch(() => {});
  // A hidden webview is suspended, so it can miss updates while away — re-read
  // the snapshot whenever a surface comes back into view.
  document.addEventListener("visibilitychange", () => {
    if (document.visibilityState !== "visible") return;
    getSnapshot()
      .then((s) => (store.snap = s))
      .catch(() => {});
  });
  // Follow theme flips made in any other window.
  onTheme((theme) => {
    store.config.theme = theme as "dark" | "light";
    applyTheme(theme);
  }).catch(() => {});
}
