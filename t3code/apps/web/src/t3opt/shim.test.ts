import { describe, expect, it } from "vite-plus/test";

import { isBackgroundCall } from "./shim";

describe("isBackgroundCall", () => {
  it.each([
    // What the owner sees or taps: the reply class, ahead of history (#16).
    ["subscribeServerLifecycle", false],
    ["subscribeAuthAccess", false],
    ["subscribeVcsStatus", false],
    ["server.getSettings", false],
    ["attachments.createUploadUrl", false],
    ["git.status", false],
    // Also when the client calls them by itself: the name cannot tell a tap from that.
    ["projects.readFile", false],
    ["preview.list", false],
    ["server.refreshProviders", false],
    // What the client does by itself.
    ["server.reportClientActivity", true],
    ["preview.reportProfiles", true],
    ["subscribeResourceTelemetry", true],
  ])("%s goes at the background class: %s", (tag, background) => {
    expect(isBackgroundCall(tag)).toBe(background);
  });
});
