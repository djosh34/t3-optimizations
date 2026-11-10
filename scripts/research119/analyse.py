#!/usr/bin/env python3
"""Bytes a cache layer would send for a sequence of responses, per scheme.

python3 analyse.py <rows.json>...   (see README.md)

Each row assumes the phone holds every chunk an earlier row sent.
plain:    content-defined chunks over the bytes (gear hash, 512 bytes to
          8 kB, about 2 kB on average); a held chunk costs a 17-byte ref.
subtrees: a held JSON subtree of at least 512 bytes costs 20 bytes, nested.
aligned:  flat chunks cut at JSON value boundaries (the design's choice).
The deflate columns stand in for Brotli of what is left.
"""
import hashlib
import json
import random
import sys
import zlib

random.seed(119)
GEAR = [random.getrandbits(64) for _ in range(256)]
MASK = (1 << 11) - 1  # about 2 kB between boundaries past the minimum
MIN, MAX = 512, 8192


def chunks(data: bytes):
    out, start, h = [], 0, 0
    i = 0
    n = len(data)
    while i < n:
        h = ((h << 1) + GEAR[data[i]]) & 0xFFFFFFFFFFFFFFFF
        size = i - start + 1
        if (size >= MIN and (h >> 40) & MASK == 0) or size >= MAX:
            out.append(data[start : i + 1])
            start, h = i + 1, 0
        i += 1
    if start < n:
        out.append(data[start:])
    return out


def cdc_cost(data: bytes, have: set):
    if len(data) < 1024:
        return len(data), data
    cost, literal = 0, b""
    for c in chunks(data):
        k = hashlib.sha256(c).digest()[:16]
        if k in have:
            cost += 17
        else:
            have.add(k)
            cost += 17 + len(c)
            literal += c
    return cost, literal


def json_cost(v, have):
    s = json.dumps(v, separators=(",", ":"), ensure_ascii=False).encode()
    if len(s) >= 512:
        k = hashlib.sha256(s).digest()[:16]
        if k in have:
            return 20
        have.add(k)
    if isinstance(v, list):
        return 2 + sum(json_cost(e, have) + 1 for e in v)
    if isinstance(v, dict):
        return 2 + sum(len(k) + 4 + json_cost(c, have) for k, c in v.items())
    return len(s)


def br(b: bytes):
    return len(zlib.compress(b, 6)) if b else 0


rows = []
for path in sys.argv[1:]:
    rows += json.load(open(path))
have_c, have_j = set(), set()
tot = {"full": 0, "cdc": 0, "json": 0}
print("| response | bytes | plain | subtrees | bytes deflated | plain literal deflated |")
print("|---|---:|---:|---:|---:|---:|")
for r in rows:
    data = r["text"].encode()
    c, lit = cdc_cost(data, have_c)
    j = json_cost(json.loads(r["text"]), have_j)
    tot["full"] += len(data)
    tot["cdc"] += c
    tot["json"] += j
    print(f"| {r['label']} | {len(data)} | {c} | {j} | {br(data)} | {br(lit)} |")
print(f"| total | {tot['full']} | {tot['cdc']} | {tot['json']} | | |")


# Scheme aligned: flat chunks cut at JSON value boundaries. A value of at least
# 512 B with no child that large is a chunk of its own; other large values are
# split into their parts;
# smaller ones are grouped, a group ending after a value whose hash matches
# a mask (content-defined) or at 4 kB.
def units(v, out):
    s = json.dumps(v, separators=(",", ":"), ensure_ascii=False).encode()
    kids = list(v.values()) if isinstance(v, dict) else v if isinstance(v, list) else []
    big_kid = any(len(json.dumps(c, separators=(",", ":"), ensure_ascii=False).encode()) >= 512 for c in kids)
    if not isinstance(v, (dict, list)) or (len(s) <= 8192 and not big_kid):
        out.append((s, len(s) >= 512))
        return
    if isinstance(v, dict):
        out.append((b"{", False))
        for i, (k, c) in enumerate(v.items()):
            out.append(((b"," if i else b"") + json.dumps(k).encode() + b":", False))
            units(c, out)
        out.append((b"}", False))
    else:
        out.append((b"[", False))
        for i, c in enumerate(v):
            if i:
                out.append((b",", False))
            units(c, out)
        out.append((b"]", False))


def aligned_chunks(v):
    us = []
    units(v, us)
    chunks_, cur = [], b""
    for s, alone in us:
        if alone:
            if cur:
                chunks_.append(cur)
                cur = b""
            chunks_.append(s)
            continue
        cur += s
        if (len(cur) >= 512 and hashlib.sha256(s).digest()[0] % 4 == 0) or len(cur) >= 4096:
            chunks_.append(cur)
            cur = b""
    if cur:
        chunks_.append(cur)
    return chunks_


def aligned_cost(v, have):
    cost, lit = 0, b""
    for c in aligned_chunks(v):
        if len(c) < 64:
            cost += len(c)
            continue
        k = hashlib.sha256(c).digest()[:16]
        if k in have:
            cost += 17
        else:
            have.add(k)
            cost += 17 + len(c)
            lit += c
    return cost, lit


have_a = set()
tot_a = 0
print()
print("| response | aligned | aligned literal deflated | chunks |")
print("|---|---:|---:|---:|")
for r in rows:
    v = json.loads(r["text"])
    c, lit = aligned_cost(v, have_a)
    tot_a += c
    print(f"| {r['label']} | {c} | {br(lit)} | {len(aligned_chunks(v))} |")
print(f"| total | {tot_a} | | |")
