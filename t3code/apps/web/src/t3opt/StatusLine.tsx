// The link status (issue #33): one line above the app with live or last
// heard, sync progress, the prompt's state, up and down kB/s and the
// transport; a tap opens the details.

import { useEffect, useState, useSyncExternalStore } from "react";

import type { Engine, EngineStatus } from "./engine";
import { type PromptState, shimEvents } from "./shim";
import { updateEvents } from "./serviceWorker";

const PROMPT_LABELS: Record<PromptState, string> = {
  queued: "prompt queued",
  sent: "prompt sent",
  "server-has": "server has prompt",
  confirmed: "prompt confirmed",
  failed: "prompt failed",
};

function kb(bytesPerSecond: number) {
  return `${(bytesPerSecond / 1000).toFixed(bytesPerSecond < 10_000 ? 1 : 0)} kB/s`;
}

function ago(ms: number) {
  const s = Math.round(ms / 1000);
  return s < 120 ? `${s} s` : `${Math.round(s / 60)} min`;
}

/** Bytes the phone has of every object it knows, and their length. */
function progress(engine: Engine) {
  let have = 0;
  let total = 0;
  for (const id of engine.objects.ids()) {
    const o = engine.objects.get(id);
    if (!o) continue;
    total += o.len;
    have += o.ranges.reduce((n, [s, e]) => n + (e - s), 0);
  }
  return { have, total };
}

interface Rates {
  readonly up: number;
  readonly down: number;
}

function useTicker(ms: number) {
  const [, setTick] = useState(0);
  // oxlint-disable-next-line t3code/no-unreasoned-state-sync -- a clock: "last heard" and rates change with time, not with React state
  useEffect(() => {
    const t = setInterval(() => setTick((n) => n + 1), ms);
    return () => clearInterval(t);
  }, [ms]);
}

function useRates(status: () => EngineStatus): Rates {
  const [rates, setRates] = useState<Rates>({ up: 0, down: 0 });
  // oxlint-disable-next-line t3code/no-unreasoned-state-sync -- samples the engine's byte counters once a second
  useEffect(() => {
    let last = status().stats;
    let lastAt = performance.now();
    const t = setInterval(() => {
      const now = performance.now();
      const s = status().stats;
      const dt = (now - lastAt) / 1000;
      setRates({
        up: (s.sentWireBytes - last.sentWireBytes) / dt,
        down: (s.receivedWireBytes - last.receivedWireBytes) / dt,
      });
      last = s;
      lastAt = now;
    }, 2_000);
    return () => clearInterval(t);
  }, [status]);
  return rates;
}

function subscribeShim(listener: () => void) {
  shimEvents.listeners.add(listener);
  return () => shimEvents.listeners.delete(listener);
}

function subscribeUpdate(listener: () => void) {
  updateEvents.listeners.add(listener);
  return () => updateEvents.listeners.delete(listener);
}

export function StatusLine({ engine }: { engine: Engine | null }) {
  const [open, setOpen] = useState(false);
  const prompt = useSyncExternalStore(subscribeShim, () => shimEvents.prompt);
  const updateWaiting = useSyncExternalStore(subscribeUpdate, () => updateEvents.waiting);
  useTicker(1_000);
  const status = engine === null ? null : engine.status();
  if (engine === null || status === null) {
    return (
      <div className="t3opt-status" data-testid="t3opt-status">
        <span>not paired</span>
        {updateWaiting ? <UpdateButton /> : null}
      </div>
    );
  }
  return <Connected engine={engine} open={open} setOpen={setOpen} prompt={prompt} update={updateWaiting} />;
}

function UpdateButton() {
  return (
    <button
      type="button"
      className="t3opt-update"
      data-testid="t3opt-update"
      onClick={(e) => {
        e.stopPropagation();
        updateEvents.apply();
      }}
    >
      Update ready, reload
    </button>
  );
}

function Connected(props: {
  engine: Engine;
  open: boolean;
  setOpen: (open: boolean) => void;
  prompt: typeof shimEvents.prompt;
  update: boolean;
}) {
  const { engine } = props;
  const status = engine.status();
  const rates = useRates(() => engine.status());
  const { link, stats } = status;
  const silent = link.lastDataAt === null ? null : Date.now() - link.lastDataAt;
  const live = silent !== null && silent < 3_000;
  const { have, total } = progress(engine);
  const synced = total === 0 ? 0 : Math.floor((have / total) * 100);
  const heard = live ? "live" : silent === null ? "not heard yet" : `last heard ${ago(silent)} ago`;
  return (
    <>
      <button
        type="button"
        className="t3opt-status"
        data-testid="t3opt-status"
        data-live={live}
        onClick={() => props.setOpen(!props.open)}
      >
        <span className="t3opt-dot" />
        <span>{heard}</span>
        <span>synced {synced}%</span>
        {props.prompt ? <span data-testid="t3opt-prompt">{PROMPT_LABELS[props.prompt.state]}</span> : null}
        <span>
          ↓{kb(rates.down)} ↑{kb(rates.up)}
        </span>
        <span>{link.trying}</span>
        {props.update ? <UpdateButton /> : null}
      </button>
      {props.open ? (
        <div className="t3opt-details" data-testid="t3opt-details" onClick={() => props.setOpen(false)}>
          <dl>
            <dt>Transport</dt>
            <dd>
              {link.transport}, generation {link.generation}, {link.trying}
            </dd>
            <dt>Handshake</dt>
            <dd>{link.handshakeMs === null ? "none yet" : `${link.handshakeMs} ms`}</dd>
            <dt>Reconnects</dt>
            <dd>
              {link.iceRestarts} ICE restarts, {link.newConnections} new connections
            </dd>
            <dt>Rate</dt>
            <dd>
              sending {kb(stats.sendRate)}, server receives {kb(stats.deliveredRate)}
            </dd>
            <dt>Loss</dt>
            <dd>{Math.round(stats.loss * 100)}% of packets to the server</dd>
            <dt>RTT</dt>
            <dd>
              {stats.srttMs === null ? "–" : `${Math.round(stats.srttMs)} ms`}
              {stats.minRttMs === null ? "" : `, lowest ${Math.round(stats.minRttMs)} ms`}
            </dd>
            <dt>Silences</dt>
            <dd>
              {link.silences} over 2 s, longest {ago(link.longestSilenceMs)}; longest recovered on
              the same connection {ago(link.longestRecoveredGapMs)}
            </dd>
            <dt>Synced</dt>
            <dd>
              {(have / 1000).toFixed(0)} of {(total / 1000).toFixed(0)} kB known so far
            </dd>
            <dt>Packets</dt>
            <dd>
              {stats.sentPackets} sent, {stats.receivedPackets} received, {status.undecodable}{" "}
              unreadable
            </dd>
          </dl>
        </div>
      ) : null}
    </>
  );
}
