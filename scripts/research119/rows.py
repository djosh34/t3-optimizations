#!/usr/bin/env python3
"""Turns fetch.mjs output into the response sequences analyse.py counts.

python3 rows.py <out dir>   writes <out dir>/config-rows.json and thread-rows.json

Two changes are made by hand, because an idle throwaway server makes neither:
every `checkedAt` in the config gets a new value, then only the first does
(#90 saw provider health refreshes change them); and the newest reply of the
thread grows by 620 bytes, as a streamed reply does.
"""
import json
import re
import sys

out = sys.argv[1]
config = json.load(open(f"{out}/config.json"))
every = re.sub(r'"checkedAt":"[^"]*"', '"checkedAt":"2026-11-10T16:10:00.000Z"', config["get"])
one = re.sub(r'"checkedAt":"[^"]*"', '"checkedAt":"2026-11-10T16:15:00.000Z"', every, count=1)
json.dump(
    [
        {"label": "config, first time (subscription snapshot)", "text": config["stream"]},
        {"label": "config again, same content (server.getConfig)", "text": config["get"]},
        {"label": "config again, every checkedAt changed", "text": every},
        {"label": "config again, one checkedAt changed", "text": one},
    ],
    open(f"{out}/config-rows.json", "w"),
)

rows = json.load(open(f"{out}/thread.json"))
again = rows[-1]["text"]
at = again.rfind("Synthetic history for the scrolling test.")
grown = again[:at] + "A new line the agent streamed. " * 20 + again[at:]
rows.append({"label": "bounded, newest reply grew by 620 bytes", "text": grown})
json.dump(rows, open(f"{out}/thread-rows.json", "w"))
