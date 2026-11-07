import { it } from "@effect/vitest";
import { OrchestrationV2TurnItem } from "@t3tools/contracts";
import * as Effect from "effect/Effect";
import * as Fiber from "effect/Fiber";
import * as Schema from "effect/Schema";
import * as Stream from "effect/Stream";
import * as TestClock from "effect/testing/TestClock";
import { describe, expect } from "vite-plus/test";

import { splitBufferedAssistantText } from "./assistantStreaming.ts";
import {
  isLiveTextItem,
  LIVE_TEXT_FRAME_INTERVAL_MS,
  type LiveTextItem,
  makeLiveTextHub,
} from "./LiveTextHub.ts";

const decodeItem = Schema.decodeUnknownSync(Schema.toCodecJson(OrchestrationV2TurnItem));

function assistantItem(id: string, text: string, streaming: boolean): LiveTextItem {
  const item = decodeItem({
    id,
    threadId: "thread-1",
    runId: "run-1",
    nodeId: "node-1",
    providerThreadId: "provider-thread-1",
    providerTurnId: null,
    nativeItemRef: null,
    parentItemId: null,
    ordinal: 2,
    status: streaming ? "running" : "completed",
    title: null,
    startedAt: "2026-11-07T13:22:01.702Z",
    completedAt: null,
    updatedAt: "2026-11-07T13:22:11.652Z",
    type: "assistant_message",
    messageId: `message-${id}`,
    text,
    streaming,
  });
  if (!isLiveTextItem(item)) throw new Error("fixture is not an assistant message");
  return item;
}

const texts = (frames: ReadonlyArray<{ readonly item: LiveTextItem }>) =>
  frames.map((frame) => [frame.item.id, frame.item.text]);

describe("LiveTextHub", () => {
  it.effect("sends partial text the paragraph buffer holds back, latest per frame", () =>
    Effect.scoped(
      Effect.gen(function* () {
        const hub = makeLiveTextHub();
        const open = "A paragraph that has not ended yet";
        // Normal subscribers would see none of it.
        expect(splitBufferedAssistantText(open).ready).toBe("");
        yield* hub.publish(assistantItem("a", open, true));
        const pull = yield* Stream.toPull(hub.subscribe);
        expect(texts(yield* pull)).toEqual([["a", open]]);

        yield* hub.publish(assistantItem("a", `${open}, still`, true));
        yield* hub.publish(assistantItem("a", `${open}, still going\n\`\`\`ts\nconst x`, true));
        expect(texts(yield* pull)).toEqual([["a", `${open}, still going\n\`\`\`ts\nconst x`]]);

        // The next change waits for the frame interval.
        yield* hub.publish(assistantItem("a", `${open}, still going.`, false));
        const next = yield* Effect.forkChild(pull);
        yield* TestClock.adjust(LIVE_TEXT_FRAME_INTERVAL_MS);
        expect(texts(yield* Fiber.join(next))).toEqual([["a", `${open}, still going.`]]);
      }),
    ),
  );

  it.effect("starts a new subscriber with the items still streaming", () =>
    Effect.scoped(
      Effect.gen(function* () {
        const hub = makeLiveTextHub();
        yield* hub.publish(assistantItem("done", "Finished text.", false));
        yield* hub.publish(assistantItem("open", "Still writing", true));
        const pull = yield* Stream.toPull(hub.subscribe);
        expect(texts(yield* pull)).toEqual([["open", "Still writing"]]);
      }),
    ),
  );
});
