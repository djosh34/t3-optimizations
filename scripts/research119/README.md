# Cache measurements for #119

These scripts produce the table in `docs/design/plan3.md`, "Measured on stock t3": what three ways of cutting response bodies into chunks would send for a sequence of real stock t3 v0.1.7 responses. They change no code and touch no live service.

1. Start a throwaway stock t3 on a free port with a fresh base dir under `/tmp`, and add a project:

   ```sh
   base=/tmp/research119/t3; mkdir -p "$base"
   ~/.local/share/t3opt/t3-0.1.7/t3 serve --base-dir "$base" --port 13805 &
   ~/.local/share/t3opt/t3-0.1.7/t3 project add --base-dir "$base" --title research119 /tmp/research119
   ```

2. Stop only that t3, seed two 600-message threads with `scripts/research116/seed.py "$base"` from the `research/116-scroll-flash` branch, and start it again.
3. Fetch, build the sequences, count:

   ```sh
   node scripts/research119/fetch.mjs "$base" 13805 /tmp/research119
   python3 scripts/research119/rows.py /tmp/research119
   python3 scripts/research119/analyse.py /tmp/research119/config-rows.json
   python3 scripts/research119/analyse.py /tmp/research119/thread-rows.json
   ```

4. Stop the throwaway t3 and delete `/tmp/research119`.

`fetch.mjs` pairs with `t3 pair`, so each run leaves one login session on the throwaway server, which goes with its base dir. Do not point it at a live t3: the script refuses a base dir outside `/tmp`.

The three schemes in `analyse.py`:

- plain: FastCDC-style chunks over the bytes (512 bytes to 8 kB, about 2 kB on average);
- subtrees: any JSON subtree of at least 512 bytes the phone holds becomes a 20-byte reference, nested;
- aligned: flat chunks cut at JSON value boundaries, the scheme the design picks.

Each scheme assumes the phone holds every chunk an earlier row sent. The config's size depends on what the server has discovered about providers by then: the design's table was taken at 101 kB, and a later run of the same server gave 63 kB. The compressed columns use deflate as a stand-in for Brotli and mean little for the seeded threads, whose text repeats far more than real text.
