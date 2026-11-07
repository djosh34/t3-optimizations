// The forked server's state as the web client reads it, rebuilt from synced
// objects (crates/sidecar/README.md, "What phones get"). A thread shows as
// soon as its thread body and frame are here; texts fill in as they arrive,
// with an ellipsis where bytes are still missing.

import { jsonOf, type Obj, type Objects, textOf } from "./objects";

type Json = Record<string, unknown>;

const COLLECTIONS = [
  "runs",
  "attempts",
  "nodes",
  "subagents",
  "providerSessions",
  "providerThreads",
  "providerTurns",
  "runtimeRequests",
  "messages",
  "plans",
  "turnItems",
  "checkpointScopes",
  "checkpoints",
  "contextHandoffs",
  "contextTransfers",
] as const;

function isJson(v: unknown): v is Json {
  return typeof v === "object" && v !== null && !Array.isArray(v);
}

/** A config object's JSON: "config", "descriptor" or "session". */
export function configOf(objects: Objects, name: string): Json | null {
  const v = jsonOf(objects.ofKind(name)[0]);
  return isJson(v) ? v : null;
}

/** The thread list, as `OrchestrationV2ShellSnapshot`; null until its header is here. */
export function shellSnapshot(objects: Objects): Json | null {
  const header = jsonOf(objects.ofKind("shell")[0]);
  if (!isJson(header)) return null;
  const projects = objects.ofKind("project").flatMap((o) => {
    const v = jsonOf(o);
    return isJson(v) ? [v] : [];
  });
  const threads: Json[] = [];
  const archivedThreads: Json[] = [];
  for (const o of objects.ofKind("thread")) {
    const v = jsonOf(o);
    if (!isJson(v)) continue;
    (o.metaJson.l === "archive" ? archivedThreads : threads).push(v);
  }
  const snapshot: Json = {
    schemaVersion: header.schemaVersion,
    snapshotSequence: header.snapshotSequence,
    projects,
    threads,
    archivedThreads,
  };
  if (isJson(header.olderSettled)) snapshot.olderSettled = header.olderSettled;
  return snapshot;
}

/** Text objects of a thread by their key (message or item id). */
function textsByKey(objs: ReadonlyArray<Obj>): Map<string, Obj> {
  const out = new Map<string, Obj>();
  for (const o of objs) if (o.metaJson.m !== undefined) out.set(o.metaJson.m, o);
  return out;
}

/** Parsed bodies, kept while their bytes do not change, so unchanged rows keep their identity. */
const bodyCache = new WeakMap<Uint8Array, unknown>();

function body(o: Obj): unknown {
  if (bodyCache.has(o.data)) return bodyCache.get(o.data);
  const v = jsonOf(o);
  bodyCache.set(o.data, v);
  return v;
}

const filled = new WeakMap<object, { text: string; value: Json }>();

/** A body with its text filled in from its text object. */
function withText(entity: Json, key: string | undefined, texts: Map<string, Obj>): Json {
  if (key === undefined || typeof entity.text !== "string") return entity;
  const t = texts.get(key);
  const text = t ? textOf(t) : "";
  const cached = filled.get(entity);
  if (cached?.text === text) return cached.value;
  const value = { ...entity, text };
  filled.set(entity, { text, value });
  return value;
}

export interface ThreadSnapshot {
  readonly snapshotSequence: number;
  readonly projection: Json;
  readonly historyCursor: null;
  readonly hasMoreHistory: false;
  readonly latestLocalTurnOrdinal: number | null;
}

/**
 * A thread as a `subscribeThread` snapshot item without its kind; null until
 * its thread body and frame are here.
 */
export function threadSnapshot(objects: Objects, threadId: string): ThreadSnapshot | null {
  const objs = objects.ofThread(threadId);
  const texts = textsByKey(objs);
  const lists = new Map<string, Json[]>(COLLECTIONS.map((c) => [c, []]));
  let thread: Json | null = null;
  let frame: Json | null = null;
  for (const o of objs) {
    const c = o.metaJson.c;
    if (c === undefined) continue;
    const v = body(o);
    if (!isJson(v)) continue;
    if (c === "frame") {
      frame = v;
    } else if (c === "thread") {
      thread = v;
    } else {
      const key =
        c === "messages"
          ? (v.id as string | undefined)
          : c === "turnItems"
            ? ((v.messageId as string | undefined) ?? (v.id as string | undefined))
            : undefined;
      lists.get(c)?.push(withText(v, key, texts));
    }
  }
  if (thread === null || frame === null) return null;
  const turnItems = lists.get("turnItems") ?? [];
  turnItems.sort((a, b) => Number(a.ordinal ?? 0) - Number(b.ordinal ?? 0));
  const projection: Json = { thread, ...Object.fromEntries(lists) };
  projection.visibleTurnItems = visibleRows(threadId, frame, turnItems);
  projection.updatedAt = frame.updatedAt;
  return {
    snapshotSequence: Number(frame.snapshotSequence ?? 0),
    projection,
    historyCursor: null,
    hasMoreHistory: false,
    latestLocalTurnOrdinal:
      typeof frame.latestLocalTurnOrdinal === "number" ? frame.latestLocalTurnOrdinal : null,
  };
}

const rowCache = new WeakMap<object, WeakMap<object, Json>>();

/** A frame row with its turn item, keeping its identity while neither changes. */
function rowWithItem(row: Json, item: Json, position: number): Json {
  let byItem = rowCache.get(row);
  if (!byItem) rowCache.set(row, (byItem = new WeakMap()));
  const cached = byItem.get(item);
  if (cached?.position === position) return cached;
  const value = { ...row, item, position };
  byItem.set(item, value);
  return value;
}

/**
 * The frame's rows with their items. The sidecar leaves the item out of a
 * local row (it is a turn item of its own); a row whose item has not arrived
 * yet is left out until it has. Local items older than the frame's first
 * local row get rows too: a frame from a live subscription covers only the
 * newest turns, and the phone has the older ones, so it shows all of them.
 */
function visibleRows(threadId: string, frame: Json, turnItems: ReadonlyArray<Json>): Json[] {
  const rows = Array.isArray(frame.visibleTurnItems) ? frame.visibleTurnItems.filter(isJson) : [];
  const byId = new Map(turnItems.map((t) => [t.id, t]));
  const shown = new Set(rows.map((r) => r.sourceItemId));
  const firstLocal = rows.find((r) => r.visibility === "local");
  const firstOrdinal = byId.get(firstLocal?.sourceItemId)?.ordinal;
  const older =
    frame.hasMoreHistory === true && typeof firstOrdinal === "number"
      ? turnItems
          .filter(
            (t) => typeof t.ordinal === "number" && t.ordinal < firstOrdinal && !shown.has(t.id),
          )
          .map((t) => ({ visibility: "local", sourceThreadId: threadId, sourceItemId: t.id }))
      : [];
  const out: Json[] = [];
  for (const row of [...older, ...rows]) {
    const item = isJson(row.item) ? row.item : byId.get(row.sourceItemId);
    if (item !== undefined) out.push(rowWithItem(row, item, out.length));
  }
  return out;
}

/** An image object's bytes as a blob, by attachment id. */
export function imageBlob(objects: Objects, attachmentId: string): Blob | null {
  const o = objects.ofKind("image").find((i) => i.metaJson.i === attachmentId);
  if (!o?.complete) return null;
  return new Blob([o.data.slice()], { type: o.metaJson.y ?? "application/octet-stream" });
}
