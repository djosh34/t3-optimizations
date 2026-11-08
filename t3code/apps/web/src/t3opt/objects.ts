// The synced objects as the shim reads them, indexed by what their meta says
// (crates/sidecar/README.md, "What phones get").

import type { ObjectView } from "./phoneWasm";

export interface Meta {
  /** Thread id, for a thread's objects. */
  readonly t?: string;
  /** Collection or kind: "shell", "project", "thread", "frame", "image", "config", ... */
  readonly c?: string;
  /** Id within the collection. */
  readonly i?: string;
  /** Text key (message or item id) of a text object. */
  readonly m?: string;
  /** Text kind: "assistant", "user", "reasoning", ... */
  readonly k?: string;
  /** Listed thread location. */
  readonly l?: string;
  /** Image MIME type. */
  readonly y?: string;
}

export interface Obj extends ObjectView {
  readonly id: bigint;
  readonly metaJson: Meta;
  /** Every byte is here. A complete empty object is deleted. */
  readonly complete: boolean;
  /**
   * The content of the last complete revision while a newer one arrives. The
   * object stays on screen as it was: a thread's row missing from the list
   * for seconds made the composer ignore Enter (issue #46).
   */
  readonly lastComplete?: Uint8Array;
}

/** The content to show: this revision's when complete, else the last complete one. */
function shownData(o: Obj): Uint8Array | null {
  if (o.complete) return o.data;
  return o.lastComplete ?? null;
}

const decoder = new TextDecoder();

function parseMeta(bytes: Uint8Array): Meta {
  try {
    const v: unknown = JSON.parse(decoder.decode(bytes));
    return typeof v === "object" && v !== null ? (v as Meta) : {};
  } catch {
    return {};
  }
}

/** The text of an object, with what has not arrived yet shown as an ellipsis. */
export function textOf(o: Obj): string {
  const shown = shownData(o);
  if (shown !== null) return decoder.decode(shown);
  let out = "";
  let at = 0;
  for (const [start, end] of o.ranges) {
    if (start > at) out += "…";
    // A range starts and ends on a character boundary: the sidecar cuts chunks there.
    out += decoder.decode(o.data.subarray(start, end));
    at = end;
  }
  if (at < o.len) out += "…";
  return out;
}

/** A complete object's JSON content, or the last complete revision's, or null. */
export function jsonOf(o: Obj | undefined): unknown {
  const shown = o ? shownData(o) : null;
  if (shown === null || shown.length === 0) return null;
  try {
    return JSON.parse(decoder.decode(shown));
  } catch {
    return null;
  }
}

export class Objects {
  private readonly byId = new Map<bigint, Obj>();
  private readonly byThread = new Map<string, Set<bigint>>();
  private readonly byKind = new Map<string, Set<bigint>>();
  private readonly listeners = new Set<(changed: ReadonlySet<bigint>) => void>();

  ids(): IterableIterator<bigint> {
    return this.byId.keys();
  }

  get(id: bigint): Obj | undefined {
    return this.byId.get(id);
  }

  /** Objects of a thread that are not deleted. */
  ofThread(thread: string): Obj[] {
    return this.collect(this.byThread.get(thread));
  }

  /** Objects whose meta `c` is `kind`, outside threads. */
  ofKind(kind: string): Obj[] {
    return this.collect(this.byKind.get(kind));
  }

  private collect(ids: Set<bigint> | undefined): Obj[] {
    const out: Obj[] = [];
    for (const id of ids ?? []) {
      const o = this.byId.get(id);
      if (o && !(o.complete && o.len === 0)) out.push(o);
    }
    return out;
  }

  set(id: bigint, view: ObjectView | null): void {
    if (view === null) {
      this.byId.delete(id);
      return;
    }
    const metaJson = parseMeta(view.meta);
    const have = view.ranges.reduce((n, [s, e]) => n + (e - s), 0);
    const complete = view.rev > 0n && have >= view.len;
    const prev = this.byId.get(id);
    // Kept only across a new revision: a text growing within one shows its
    // newest bytes as they come.
    const lastComplete =
      complete || prev === undefined
        ? undefined
        : prev.rev !== view.rev && prev.complete
          ? prev.data
          : prev.lastComplete;
    const o: Obj = { ...view, id, metaJson, complete, ...(lastComplete ? { lastComplete } : {}) };
    this.byId.set(id, o);
    const index = metaJson.t !== undefined ? this.byThread : this.byKind;
    const name = metaJson.t ?? metaJson.c ?? "";
    let ids = index.get(name);
    if (!ids) index.set(name, (ids = new Set()));
    ids.add(id);
  }

  subscribe(listener: (changed: ReadonlySet<bigint>) => void): () => void {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }

  notify(changed: ReadonlySet<bigint>): void {
    if (changed.size === 0) return;
    for (const l of this.listeners) l(changed);
  }
}
