import type { OrchestrationV2LiveTextFrame, OrchestrationV2TurnItem } from "@t3tools/contracts";
import * as Clock from "effect/Clock";
import * as Context from "effect/Context";
import * as Effect from "effect/Effect";
import * as Queue from "effect/Queue";
import * as Stream from "effect/Stream";

/**
 * Raw partial text of assistant and reasoning items for the t3-optimizations sidecar, which
 * syncs it to a phone over a lossy link. Normal clients get the paragraph and code-fence
 * buffering of assistantStreaming.ts; this hub sees each provider update before that filter.
 * Nothing here is persisted.
 */

/** Fastest a subscriber receives frames while text streams. */
export const LIVE_TEXT_FRAME_INTERVAL_MS = 100;
/** Finished items kept for subscribers that have not seen their last frame yet. */
const MAX_ITEMS = 64;

export type LiveTextItem = Extract<
  OrchestrationV2TurnItem,
  { readonly type: "assistant_message" | "reasoning" }
>;

export function isLiveTextItem(item: OrchestrationV2TurnItem): item is LiveTextItem {
  return item.type === "assistant_message" || item.type === "reasoning";
}

export interface LiveTextHubShape {
  /** Records the item's latest text. A finished item goes out once more, then is dropped. */
  readonly publish: (item: LiveTextItem) => Effect.Effect<void>;
  /** Items still streaming, then every change, at most one batch per frame interval. */
  readonly subscribe: Stream.Stream<OrchestrationV2LiveTextFrame>;
}

interface Entry {
  readonly item: LiveTextItem;
  readonly version: number;
}

export function makeLiveTextHub(): LiveTextHubShape {
  const items = new Map<string, Entry>();
  const wakes = new Set<Queue.Queue<void>>();
  let version = 0;

  const publish: LiveTextHubShape["publish"] = (item) =>
    Effect.suspend(() => {
      version += 1;
      items.delete(item.id);
      items.set(item.id, { item, version });
      // Map order is insertion order, so the first entries are the least recently written.
      for (const [id, entry] of items) {
        if (items.size <= MAX_ITEMS) break;
        if (!entry.item.streaming) items.delete(id);
      }
      return Effect.forEach(wakes, (wake) => Queue.offer(wake, undefined), { discard: true });
    });

  const subscribe: LiveTextHubShape["subscribe"] = Stream.unwrap(
    Effect.gen(function* () {
      const wake = yield* Queue.sliding<void>(1);
      wakes.add(wake);
      yield* Effect.addFinalizer(() => Effect.sync(() => wakes.delete(wake)));
      const streaming = [...items.values()].filter((entry) => entry.item.streaming);
      let seen = version;
      let lastBatchAt: number | undefined;
      const nextBatch = Effect.gen(function* () {
        yield* Queue.take(wake);
        const now = yield* Clock.currentTimeMillis;
        if (lastBatchAt !== undefined && now - lastBatchAt < LIVE_TEXT_FRAME_INTERVAL_MS) {
          yield* Effect.sleep(LIVE_TEXT_FRAME_INTERVAL_MS - (now - lastBatchAt));
        }
        lastBatchAt = yield* Clock.currentTimeMillis;
        const changed = [...items.values()].filter((entry) => entry.version > seen);
        seen = version;
        return changed.map((entry) => ({ item: entry.item }));
      });
      return Stream.concat(
        Stream.fromIterable(streaming.map((entry) => ({ item: entry.item }))),
        Stream.fromEffectRepeat(nextBatch).pipe(Stream.flatMap(Stream.fromIterable)),
      );
    }),
  );

  return { publish, subscribe };
}

/** One hub per server process; tests provide their own. */
export class LiveTextHub extends Context.Reference<LiveTextHubShape>(
  "t3/orchestration-v2/LiveTextHub",
  { defaultValue: makeLiveTextHub },
) {}
