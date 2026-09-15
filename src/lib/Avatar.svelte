<script lang="ts">
  import { store } from "./state.svelte";
  import { beginDrag, moveAvatar, endDrag, togglePanel } from "./bridge";
  import { prefs } from "./prefs.svelte";
  import type { AvatarState } from "./types";

  let s = $derived(store.snap.state);
  // Pending tool approvals (Phase 4) — intensify the alarmed cat's coral glow
  // (see the shocked sprite CSS), distinct from the gentler context-alarm.
  let needsInput = $derived(store.snap.waiting > 0);

  // Per-state session tallies for the pips around Tablo. A session with a pending
  // permission request counts only as "permission" (matches the panel's grouping:
  // Permission Request / Waiting / Working). Each pip shows only when its count > 0.
  let pendingIds = $derived(new Set(store.snap.pending.map((p) => p.sessionId)));
  // Caution pip: the panel's two coral groups (pending approval, context past warn%), per session.
  let attnCount = $derived(
    new Set([
      ...pendingIds,
      ...store.snap.sessions.filter((x) => x.level !== "ok").map((x) => x.id),
    ]).size,
  );
  let waitCount = $derived(
    store.snap.sessions.filter((x) => !pendingIds.has(x.id) && x.activityKind === "waiting").length,
  );
  let workCount = $derived(
    store.snap.sessions.filter((x) => !pendingIds.has(x.id) && x.activityKind !== "waiting").length,
  );

  // Alarm startles the cat briefly, then it falls back to trot/sleep; a new permission re-startles.
  const SHOCK_MS = 7000;
  let shocked = $state(false);
  let shockTimer: ReturnType<typeof setTimeout> | undefined;
  let wasAlarmed = false;
  let hadInput = false;
  $effect(() => {
    const alarmed = s === "alarmed";
    const startle = alarmed && (!wasAlarmed || (needsInput && !hadInput));
    wasAlarmed = alarmed;
    hadInput = needsInput;
    if (startle || !alarmed) {
      clearTimeout(shockTimer);
      shocked = startle;
      if (startle) shockTimer = setTimeout(() => (shocked = false), SHOCK_MS);
    }
  });
  let display = $derived(s === "alarmed" && !shocked ? (workCount > 0 ? "running" : "idle") : s);

  // Sleep/wake transitions between the sleeping and running loops, both driven
  // off the one curl-down sheet (the wake-up just plays it in reverse). prevState
  // is plain bookkeeping (not reactive) so this effect only re-runs when the
  // avatar state actually changes — or when `animations` flips (read below), which
  // takes the reset branch and clears any transition that was in flight.
  let prevState: AvatarState | null = null;
  let transitioning = $state(false); // running → idle: curl down to sleep
  let waking = $state(false); // idle → running: uncurl and get up
  $effect(() => {
    const cur = display;
    const animate = prefs.animations;
    if (animate && prevState === "running" && cur === "idle") {
      transitioning = true;
      waking = false;
    } else if (animate && prevState === "idle" && cur === "running") {
      waking = true;
      transitioning = false;
    } else {
      // any other change (e.g. → alarmed) cancels an in-flight transition
      transitioning = false;
      waking = false;
    }
    prevState = cur;
  });

  // --- tap vs. drag ---
  const DRAG_THRESHOLD = 6; // px of movement before a press becomes a drag
  let downScreen: { x: number; y: number } | null = null;
  let origin: { x: number; y: number } | null = null;
  let moved = false;
  let last: { x: number; y: number } | null = null;

  let raf = 0;
  let pending: { x: number; y: number } | null = null;
  function scheduleMove(x: number, y: number) {
    pending = { x, y };
    if (!raf) raf = requestAnimationFrame(flushMove);
  }
  function flushMove() {
    raf = 0;
    if (pending) {
      moveAvatar(pending.x, pending.y);
      pending = null;
    }
  }

  async function onDown(e: PointerEvent) {
    if (e.button !== 0) return;
    (e.currentTarget as HTMLElement).setPointerCapture(e.pointerId);
    downScreen = { x: e.screenX, y: e.screenY };
    moved = false;
    origin = null;
    last = null;
    try {
      origin = await beginDrag();
      last = { ...origin };
    } catch {
      /* backend unavailable */
    }
  }

  function onMove(e: PointerEvent) {
    if (!downScreen || !origin) return;
    const dx = e.screenX - downScreen.x;
    const dy = e.screenY - downScreen.y;
    if (!moved && Math.hypot(dx, dy) > DRAG_THRESHOLD) moved = true;
    if (moved) {
      last = { x: origin.x + dx, y: origin.y + dy };
      scheduleMove(last.x, last.y);
    }
  }

  async function onUp(e: PointerEvent) {
    (e.currentTarget as HTMLElement).releasePointerCapture(e.pointerId);
    const wasDrag = moved;
    const end = last ?? origin;
    downScreen = null;
    origin = null;
    moved = false;
    if (wasDrag && end) {
      if (raf) {
        cancelAnimationFrame(raf);
        raf = 0;
      }
      await moveAvatar(end.x, end.y);
      endDrag(end.x, end.y);
    } else {
      // A tap toggles the panel.
      togglePanel();
    }
  }
</script>

<div
  class="avatar-hit"
  role="button"
  tabindex="0"
  aria-label="Tablo — {s}"
  onpointerdown={onDown}
  onpointermove={onMove}
  onpointerup={onUp}
>
  <div class="tablo-wrap" class:needs-input={needsInput} class:still={!prefs.animations}>
    {#if display === "idle" && transitioning}
      <!-- running → sleeping: one-shot curl-down, then the sleeping loop -->
      <div
        class="sprite transition"
        aria-hidden="true"
        onanimationend={() => (transitioning = false)}
      ></div>
    {:else if display === "running" && waking}
      <!-- sleeping → running: the curl sheet in reverse, then the trotting loop -->
      <div
        class="sprite waking"
        aria-hidden="true"
        onanimationend={() => (waking = false)}
      ></div>
    {:else if display === "idle"}
      <!-- idle → sleeping cat -->
      <div class="sprite sleeping" aria-hidden="true"></div>
    {:else if display === "running"}
      <!-- running → trotting cat -->
      <div class="sprite running" aria-hidden="true"></div>
    {:else}
      <!-- alarmed → shocked cat (portrait box; sits bolt upright) -->
      <div class="sprite shocked" aria-hidden="true"></div>
    {/if}
    <div class="badges">
      {#if waitCount > 0}<span class="badge wait">{waitCount}</span>{/if}
      {#if workCount > 0}<span class="badge work">{workCount}</span>{/if}
    </div>
  </div>
  {#if attnCount > 0}<span class="badge attn">{attnCount}</span>{/if}
</div>

<style>
  .avatar-hit {
    position: fixed;
    inset: 0;
    display: grid;
    place-items: center;
    background: transparent;
  }

  .tablo-wrap {
    position: relative;
    display: inline-grid;
    place-items: center;
  }

  /* real cat sprite. The sheet is a single row of `--frames` cells played with
     steps(); the state glow is a drop-shadow that follows the current frame's
     alpha, the same signal the placeholder hex used. Each state is just a
     background-image + timing swap, so the remaining sheets drop in the same way.
     `steps()` must match `--frames`. */
  .sprite {
    --frames: 4;
    --size: 88px; /* display WIDTH; height derives from the frame aspect below */
    /* Native frame pixel dims — MUST match the referenced sheet, or the art
       distorts. Height is computed from them so a landscape frame (wider than
       tall) is NOT crushed into a square box. */
    --fw: 1;
    --fh: 1;
    width: var(--size);
    height: calc(var(--size) * var(--fh) / var(--fw));
    background-repeat: no-repeat;
    background-position: 0 0;
    transition: transform 0.25s var(--ease);
  }
  .avatar-hit:hover .sprite {
    transform: translateY(-3px);
  }

  /* Every state glow steps rather than tweens: a smooth filter animation
     repaints on every vsync forever, which keeps laptop GPUs from idling. */

  /* idle → sleeping: curled cat, slow breathing sage glow, drifting Zs */
  .sprite.sleeping {
    --fw: 256; /* sleeping-sprite-sheet.png: 1024×196, 4 frames → 256×196 */
    --fh: 196;
    background-image: url(/sprites/sleeping-sprite-sheet.png);
    background-size: calc(var(--size) * var(--frames)) 100%;
    animation:
      sleep-frames 2.8s steps(4) infinite,
      sleep-glow 4.5s steps(18) infinite;
  }
  @keyframes sleep-frames {
    to {
      background-position-x: calc(var(--size) * var(--frames) * -1);
    }
  }
  @keyframes sleep-glow {
    0%,
    100% {
      filter: drop-shadow(0 0 5px color-mix(in srgb, var(--sage) 28%, transparent));
    }
    50% {
      filter: drop-shadow(0 0 9px color-mix(in srgb, var(--sage) 48%, transparent));
    }
  }

  /* running → trotting cat: quick trot, energetic amber "working" glow */
  .sprite.running {
    --fw: 600; /* running-sprite-sheet.png: 2400×477, 4 frames → 600×477 */
    --fh: 477;
    background-image: url(/sprites/running-sprite-sheet.png);
    background-size: calc(var(--size) * var(--frames)) 100%;
    animation:
      run-frames 0.50s steps(4) infinite,
      run-glow 1.6s steps(12) infinite;
  }
  @keyframes run-frames {
    to {
      background-position-x: calc(var(--size) * var(--frames) * -1);
    }
  }
  @keyframes run-glow {
    0%,
    100% {
      filter: drop-shadow(0 0 5px color-mix(in srgb, var(--amber) 30%, transparent));
    }
    50% {
      filter: drop-shadow(0 0 10px color-mix(in srgb, var(--amber) 55%, transparent));
    }
  }

  /* alarmed → shocked cat: a PORTRAIT box (the cat sits bolt upright), so it
     overrides the shared landscape `--size` with a narrower width; height still
     derives from the frame aspect. Fast startled frame cycle + urgent coral glow. */
  .sprite.shocked {
    --frames: 5;
    --fw: 382; /* shocked-sprite-sheet.png: 1908×544, 5 frames → ~382×544 */
    --fh: 544;
    --size: 80px; /* narrower so the taller pose still clears the count pips */
    background-image: url(/sprites/shocked-sprite-sheet.png);
    background-size: calc(var(--size) * var(--frames)) 100%;
    animation:
      shock-frames 0.7s steps(5) infinite,
      shock-glow 0.9s steps(10) infinite;
  }
  @keyframes shock-frames {
    to {
      background-position-x: calc(var(--size) * var(--frames) * -1);
    }
  }
  @keyframes shock-glow {
    0%,
    100% {
      filter: drop-shadow(0 0 6px color-mix(in srgb, var(--coral) 45%, transparent));
    }
    50% {
      filter: drop-shadow(0 0 15px color-mix(in srgb, var(--coral) 80%, transparent));
    }
  }
  /* needs-input (a pending approval) is the loudest alarm: intensify the shocked
     cat's coral glow and quicken its startled frame cycle. */
  .tablo-wrap.needs-input .sprite.shocked {
    animation:
      shock-frames 0.5s steps(5) infinite,
      shock-glow-urgent 0.85s steps(10) infinite;
  }
  @keyframes shock-glow-urgent {
    0%,
    100% {
      filter: drop-shadow(0 0 10px color-mix(in srgb, var(--coral) 60%, transparent));
    }
    50% {
      filter: drop-shadow(0 0 22px color-mix(in srgb, var(--coral) 95%, transparent));
    }
  }

  /* running → sleeping: one-shot curl-down (5-frame v2 sheet) at normal pace;
     `both` holds the final curled frame until the sleeping loop swaps in. */
  .sprite.transition {
    --frames: 5;
    --fw: 206; /* running-to-sleeping-sprite-sheet.png: 1030×180, 5 frames → 206×180 */
    --fh: 180;
    background-image: url(/sprites/running-to-sleeping-sprite-sheet.png);
    background-size: calc(var(--size) * var(--frames)) 100%;
    animation: trans-frames 0.6s steps(5) 1 both;
  }
  @keyframes trans-frames {
    to {
      background-position-x: calc(var(--size) * var(--frames) * -1);
    }
  }

  /* sleeping → working: the SAME curl sheet run in reverse (uncurl → sit → up),
     then the trotting loop takes over. Reverse without the steps() blank-frame
     quirk: step the position from the last frame (-4·size = -(frames-1)) up past
     the first (+size), so the 5 held stops are -4,-3,-2,-1,0. */
  .sprite.waking {
    --frames: 5;
    --fw: 206; /* running-to-sleeping-sprite-sheet.png: 1030×180, 5 frames → 206×180 */
    --fh: 180;
    background-image: url(/sprites/running-to-sleeping-sprite-sheet.png);
    background-size: calc(var(--size) * var(--frames)) 100%;
    animation: wake-frames 0.6s steps(5) 1 both;
  }
  @keyframes wake-frames {
    from {
      background-position-x: calc(var(--size) * -4);
    }
    to {
      background-position-x: var(--size);
    }
  }

  /* Animations off (Settings → Cat animations): every state holds its first
     frame with a steady glow instead of a breathing one, so the cat still reads
     as sage/amber/coral at a glance — only the motion goes. Each static glow is
     the midpoint of the pulse it replaces. The sleep/wake curl sheets never
     render in this mode — the effect above doesn't start a transition — so the
     cat cuts straight between the sleeping and trotting poses.

     The second selector re-states the `.needs-input .sprite.shocked` override
     above, which is otherwise more specific and would keep animating. */
  .tablo-wrap.still .sprite,
  .tablo-wrap.still.needs-input .sprite.shocked {
    animation: none;
  }
  .tablo-wrap.still .sprite.sleeping {
    filter: drop-shadow(0 0 7px color-mix(in srgb, var(--sage) 38%, transparent));
  }
  .tablo-wrap.still .sprite.running {
    filter: drop-shadow(0 0 8px color-mix(in srgb, var(--amber) 42%, transparent));
  }
  .tablo-wrap.still .sprite.shocked {
    filter: drop-shadow(0 0 10px color-mix(in srgb, var(--coral) 62%, transparent));
  }
  .tablo-wrap.still.needs-input .sprite.shocked {
    filter: drop-shadow(0 0 16px color-mix(in srgb, var(--coral) 78%, transparent));
  }

  /* count pips — float off the top-right edge, stacked: caution (red),
     waiting (green), working (amber), each shown only when its count > 0 */
  .badges {
    position: absolute;
    top: -7px;
    right: -10px;
    display: flex;
    flex-direction: column;
    gap: 3px;
    z-index: 3;
  }
  .badge {
    min-width: 19px;
    height: 19px;
    padding: 0 4px;
    border-radius: 999px;
    font-family: var(--font-mono);
    font-size: 11px;
    font-weight: 700;
    display: grid;
    place-items: center;
    border: 1px solid var(--bg-inset);
  }
  /* window-anchored so pose swaps don't move it; padding drops the count into the wide half */
  .badge.attn {
    position: absolute;
    bottom: 26px;
    right: 9px;
    z-index: 3;
    width: 24px;
    height: 21px;
    padding: 8px 0 0;
    border: 0;
    border-radius: 0;
    clip-path: polygon(50% 0, 100% 100%, 0 100%);
    background: var(--coral);
    color: #fff;
    font-size: 10px;
  }
  .badge.wait {
    background: var(--sage);
    color: #17220f;
  }
  .badge.work {
    background: var(--amber);
    color: #1c1409;
  }
  :global([data-theme="light"]) .badge.wait,
  :global([data-theme="light"]) .badge.work {
    color: #fff;
  }

</style>
