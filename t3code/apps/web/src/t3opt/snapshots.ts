// The forked server's state as the web client reads it, rebuilt from synced
// objects (crates/sidecar/README.md, "What phones get"). A thread shows as
// soon as its thread body is here; turn items and texts fill in
// as they arrive, with an ellipsis where bytes are still missing.

import { createOrchestrationV2TurnItemVisibility } from "@t3tools/shared/orchestrationV2Timeline";

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

/** Text the client shows as it streams; other text is part of a row's structure. */
function isStreamedText(c: string, entity: Json): boolean {
  return c === "turnItems"
    ? entity.type === "assistant_message" || entity.type === "reasoning"
    : entity.role === "assistant";
}

/**
 * A body with its text filled in from its text object; null while the text
 * of a row the client does not stream (a user message) is still on its way:
 * the client would keep the row it first showed.
 */
function withText(
  c: string,
  entity: Json,
  key: string | undefined,
  texts: Map<string, Obj>,
): Json | null {
  if (key === undefined || typeof entity.text !== "string") return entity;
  const t = texts.get(key);
  if (!isStreamedText(c, entity) && !t?.complete) return null;
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

/** A thread as a `subscribeThread` snapshot item without its kind; null until its thread body is here. */
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
      const filledIn = withText(c, v, key, texts);
      if (filledIn !== null) lists.get(c)?.push(filledIn);
    }
  }
  if (thread === null) return null;
  const turnItems = lists.get("turnItems") ?? [];
  turnItems.push(...standIns(objs, turnItems, lists.get("runs") ?? []));
  turnItems.sort((a, b) => Number(a.ordinal ?? 0) - Number(b.ordinal ?? 0));
  const projection: Json = { thread, ...Object.fromEntries(lists) };
  projection.visibleTurnItems = visibleRows(threadId, frame ?? {}, projection);
  projection.updatedAt = frame?.updatedAt ?? thread.updatedAt;
  return {
    snapshotSequence: Number(frame?.snapshotSequence ?? 0),
    projection,
    historyCursor: null,
    hasMoreHistory: false,
    latestLocalTurnOrdinal:
      typeof frame?.latestLocalTurnOrdinal === "number" ? frame.latestLocalTurnOrdinal : null,
  };
}

const standInCache = new WeakMap<Obj, Json>();

/**
 * Rows for assistant texts whose turn item (a kilobyte of JSON queued behind the
 * thread's status) has not arrived: copies of the newest complete assistant item.
 * The real item has the same id and takes the stand-in's place (README.md).
 */
function standIns(
  objs: ReadonlyArray<Obj>,
  items: ReadonlyArray<Json>,
  runs: ReadonlyArray<Json>,
): Json[] {
  const shown = new Set(items.map((i) => i.messageId ?? i.id));
  const assistant = items.filter((i) => i.type === "assistant_message");
  const template = assistant.reduce<Json | null>(
    (best, i) => (best === null || Number(i.ordinal) > Number(best.ordinal) ? i : best),
    null,
  );
  if (template === null) return [];
  const run = runs.reduce<Json | null>(
    (best, r) => (best === null || Number(r.ordinal) > Number(best.ordinal) ? r : best),
    null,
  );
  let ordinal = Math.max(0, ...items.map((i) => Number(i.ordinal ?? 0)));
  const out: Json[] = [];
  for (const o of objs) {
    const m = o.metaJson.m;
    if (o.metaJson.k !== "assistant" || m === undefined || shown.has(m)) continue;
    let item = standInCache.get(o);
    if (!item) {
      ordinal += 1;
      item = {
        ...template,
        id: `turn-item:message:${m}`,
        messageId: m,
        ordinal,
        text: textOf(o),
        streaming: true,
        status: "running",
        completedAt: null,
        ...(run ? { runId: run.id } : {}),
      };
      standInCache.set(o, item);
    }
    out.push(item);
  }
  return out;
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
 * The visible rows. Rows that come from other threads (inherited, synthetic)
 * are the frame's; local rows are made here from every turn item the phone
 * has, with t3's own visibility rule: the frame of the open thread comes from
 * its last fetch, while its turn items arrive as they change.
 */
function visibleRows(threadId: string, frame: Json, projection: Json): Json[] {
  const turnItems = (projection.turnItems ?? []) as Json[];
  const isVisible = createOrchestrationV2TurnItemVisibility({
    runs: (projection.runs ?? []) as never,
    attempts: (projection.attempts ?? []) as never,
    items: turnItems as never,
  });
  const frameRows = Array.isArray(frame.visibleTurnItems)
    ? frame.visibleTurnItems.filter(isJson)
    : [];
  const out: Json[] = [];
  for (const row of frameRows) {
    if (row.visibility !== "local" && isJson(row.item))
      out.push(rowWithItem(row, row.item, out.length));
  }
  for (const item of turnItems) {
    if (!isVisible(item as never)) continue;
    out.push(rowWithItem(localRow(threadId, item), item, out.length));
  }
  return out;
}

const localRows = new WeakMap<object, Json>();

function localRow(threadId: string, item: Json): Json {
  let row = localRows.get(item);
  if (!row) {
    row = { visibility: "local", sourceThreadId: threadId, sourceItemId: item.id };
    localRows.set(item, row);
  }
  return row;
}

/** An image object's bytes as a blob, by attachment id. */
/**
 * Whether the image comes over the sync link: the sidecar fetches every image
 * a turn item it knows refers to, so the phone gets it, perhaps later.
 */
export function imageComing(objects: Objects, attachmentId: string): boolean {
  if (objects.ofKind("image").some((i) => i.metaJson.i === attachmentId)) return true;
  for (const id of objects.ids()) {
    const o = objects.get(id);
    if (o?.metaJson.c !== "turnItems") continue;
    const attachments = (jsonOf(o) as Json | null)?.attachments;
    if (!Array.isArray(attachments)) continue;
    if (attachments.some((a: Json) => a?.type === "image" && a.id === attachmentId)) return true;
  }
  return false;
}

export function imageBlob(objects: Objects, attachmentId: string): Blob | null {
  const o = objects.ofKind("image").find((i) => i.metaJson.i === attachmentId);
  if (!o?.complete) return null;
  return new Blob([o.data.slice()], { type: o.metaJson.y ?? "application/octet-stream" });
}
