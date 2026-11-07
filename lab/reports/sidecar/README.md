# Sidecar reports

The sync protocol through the sidecar on the WebRTC data channel (issue #44): the forked t3 server and `crates/sidecar` in the server namespace, `crates/native-phone` in the phone container in the web app's place. The run is the same as the baseline's: a warm start on the clean link, then the profile, the measured prompt, the scripted reply with its approval. See [lab/README.md](../../README.md#sidecar-runs).

Every run was made with the lab, the sidecar and the native phone built from commit 225ea00fa9b4, two runs at a time on the dev VM. Seeds 1 to 3.

## The two graded cases

| Run | Prompt confirmed | First text | Newest-text lag max / median / p95 | Longest silence | Whole reply | Wire down / up (B) |
|---|---|---|---|---|---|---|
| [case70 s1](case70-sidecar-native-warm-s1/report.md) | 2.2 s | 5.4 s | 12.3 / 2.2 / 9.3 s | 21.6 s | done | 55163 / 28859 |
| [case70 s2](case70-sidecar-native-warm-s2/report.md) | 2.5 s | 4.4 s | 9.9 / 2.2 / 6.9 s | 18.3 s | done | 56481 / 28840 |
| [case70 s3](case70-sidecar-native-warm-s3/report.md) | 4.2 s | 11.3 s | 13.8 / 4.3 / 10.8 s | 25.3 s | done | 52974 / 28191 |
| [case50 s1](case50-sidecar-native-warm-s1/report.md) | 1.2 s | 3.5 s | 5.3 / 1.1 / 2.7 s | 7.3 s | done | 130486 / 63452 |
| [case50 s2](case50-sidecar-native-warm-s2/report.md) | 1.1 s | 2.8 s | 3.2 / 1.1 / 2.2 s | 4.9 s | done | 176637 / 83894 |
| [case50 s3](case50-sidecar-native-warm-s3/report.md) | 1.3 s | 3.2 s | 3.1 / 0.8 / 1.9 s | 6.2 s | done | 209242 / 99823 |

The targets of [issue #43](https://github.com/djosh34/t3-optimizations/issues/43):

- case 70, prompt confirmed within 10 s: met on all three seeds (2.2 to 4.2 s).
- case 50, newest text never more than 5 s behind: met on seeds 2 and 3 (3.2 and 3.1 s), missed by 0.3 s on seed 1 (5.3 s).

The [WebSocket baseline](../baseline/README.md) on the same cases, seed 1: case 70 never confirmed the prompt (with compression) or showed no text in 15 minutes (without it); case 50 confirmed the prompt after 3.5 s and 32.4 s and showed at most 397 of 1596 characters. The baseline ran before the lab's ARP fix of issue #49.

No run needed an ICE restart or a new connection, and the native phone could read every packet.

## Not like production

- The phone is the native test phone on str0m, not WebKit. WebKit's own SCTP on the phone-to-server direction stalled up to 7.6 s at 70% loss in issue #49; prompts on WebKit may confirm later.
- The lab's UDP crosses a veth pair. In production it goes through Tailscale.
- The send rate starts low after the switch from the clean link: the base RTT of the clean link (about 1 ms) stays for 60 s, and the rate drains until the path change is found, about 20 s in (issue #44 has the trace). The lag above includes that.
