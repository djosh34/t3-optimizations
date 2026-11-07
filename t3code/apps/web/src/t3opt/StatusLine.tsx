// The link status (issue #33): one line above the app; a tap opens the details.

import { useEffect, useState, useSyncExternalStore } from "react";

import type { Engine } from "./engine";
import { updateEvents } from "./serviceWorker";
import { type PromptInfo, type PromptState, shimEvents } from "./shim";

const PROMPT_LABELS: Record<PromptState, string> = {
  queued: "prompt queued",
  sent: "prompt sent",
  "server-has": "server has prompt",
  confirmed: "prompt confirmed",
  failed: "prompt failed",
};

const BAR =
  "fixed inset-x-0 top-0 z-(--t3opt-z) box-border flex h-(--t3opt-bar) w-full items-center gap-2.5 overflow-hidden whitespace-nowrap border-0 border-b border-border bg-popover px-2.5 pt-(--t3opt-safe-top) text-left font-medium text-popover-foreground text-2xs leading-none";

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

interface Sample {
  readonly at: number;
  readonly up: number;
  readonly down: number;
  /** Wire bytes per second since the sample before. */
  readonly upRate: number;
  readonly downRate: number;
}

/** Once a second: the time and the byte rates. */
function useSample(engine: Engine | null): Sample {
  const [sample, setSample] = useState<Sample>(() => ({
    at: Date.now(),
    up: 0,
    down: 0,
    upRate: 0,
    downRate: 0,
  }));
  // oxlint-disable-next-line t3code/no-unreasoned-state-sync -- a clock: "last heard" and the rates change with time, not with React state
  useEffect(() => {
    const t = setInterval(() => {
      setSample((last) => {
        const at = Date.now();
        const s = engine?.status().stats;
        const up = s?.sentWireBytes ?? 0;
        const down = s?.receivedWireBytes ?? 0;
        const dt = Math.max(0.001, (at - last.at) / 1000);
        return { at, up, down, upRate: (up - last.up) / dt, downRate: (down - last.down) / dt };
      });
    }, 1_000);
    return () => clearInterval(t);
  }, [engine]);
  return sample;
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
  const update = useSyncExternalStore(subscribeUpdate, () => updateEvents.waiting);
  const sample = useSample(engine);
  if (engine === null) {
    return (
      <div className={BAR} data-testid="t3opt-status">
        <span>not paired</span>
        {update ? <UpdateButton /> : null}
      </div>
    );
  }
  return (
    <Connected
      engine={engine}
      sample={sample}
      open={open}
      setOpen={setOpen}
      prompt={prompt}
      update={update}
    />
  );
}

function UpdateButton() {
  return (
    <button
      type="button"
      className="ml-auto rounded-lg border-0 bg-update px-2 py-0.75 text-update-foreground"
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
  sample: Sample;
  open: boolean;
  setOpen: (open: boolean) => void;
  prompt: PromptInfo | null;
  update: boolean;
}) {
  const { engine, sample } = props;
  const status = engine.status();
  const { link, stats } = status;
  const silent = link.lastDataAt === null ? null : Math.max(0, sample.at - link.lastDataAt);
  const live = silent !== null && silent < 3_000;
  const { have, total } = progress(engine);
  const synced = total === 0 ? 0 : Math.floor((have / total) * 100);
  const heard = live ? "live" : silent === null ? "not heard yet" : `last heard ${ago(silent)} ago`;
  return (
    <>
      <button
        type="button"
        className={BAR}
        data-testid="t3opt-status"
        data-live={live}
        onClick={() => props.setOpen(!props.open)}
      >
        <span className={`size-2 flex-none rounded-full ${live ? "bg-success" : "bg-warning"}`} />
        <span>{heard}</span>
        <span>synced {synced}%</span>
        {props.prompt ? (
          <span data-testid="t3opt-prompt">{PROMPT_LABELS[props.prompt.state]}</span>
        ) : null}
        <span>
          ↓{kb(sample.downRate)} ↑{kb(sample.upRate)}
        </span>
        <span>{link.trying}</span>
        {props.update ? <UpdateButton /> : null}
      </button>
      {props.open ? (
        <div
          className="fixed inset-x-0 top-(--t3opt-bar) z-(--t3opt-z) max-h-(--t3opt-details-max) overflow-auto bg-popover px-3.5 pt-3 pb-4 text-popover-foreground text-sm leading-snug shadow-2xl"
          data-testid="t3opt-details"
          onClick={() => props.setOpen(false)}
        >
          <dl className="m-0 grid grid-cols-[max-content_1fr] gap-x-3 gap-y-1.5 [&_dd]:m-0 [&_dt]:text-muted-foreground">
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
            <dt>Build</dt>
            <dd>{import.meta.env.VITE_T3OPT_BUILD_LABEL}</dd>
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
