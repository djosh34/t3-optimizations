// node scripts/research116/assert-scroll.mjs <recording.json[.gz]>
// The driver releases follow, scrolls into history, then sends no input for 3 s.
// Prepending older rows must preserve the visible row's screen position.
import fs from 'node:fs';
import zlib from 'node:zlib';
import assert from 'node:assert/strict';
const file=process.argv[2];
const raw=fs.readFileSync(file);
const recording=JSON.parse(file.endsWith('.gz')?zlib.gunzipSync(raw):raw);
const hold=recording.frames.filter(f=>f.phase==='cold-hold');
assert.ok(hold.length>0,'The recording must contain the cold-hold phase');
const frames=hold.filter(f=>f.at>=hold[0].at+300);
assert.ok(frames.at(-1).at-frames[0].at>1000,'At least one second without input is required');
const anchor=frames[0].visible[0];
assert.ok(anchor,'A visible row is required');
const positions=frames.map(f=>f.visible.find(v=>v.id===anchor.id)?.top);
const present=positions.filter(p=>p!==undefined);
const drift=Math.max(...present)-Math.min(...present);
console.log(JSON.stringify({file,anchor:anchor.id,samples:frames.length,
  missing:positions.length-present.length,driftPx:drift}));
assert.equal(positions.length-present.length,0,'The reading anchor left the viewport without input');
assert.ok(drift<=4,`The reading anchor moved ${drift}px without input, allowed 4px`);
