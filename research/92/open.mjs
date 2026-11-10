import {devices,webkit} from '/opt/driver/node_modules/playwright/index.mjs';
import fs from 'node:fs';import {execSync} from 'node:child_process';
const cfg=JSON.parse(fs.readFileSync('/private/config.json'));
const [route,size,code,run]=process.argv.slice(2);const origin=cfg.origins[route];
const browser=await webkit.launch();const context=await browser.newContext({...devices['iPhone 17 Pro'],viewport:{width:402,height:size==='safari'?681:874}});
const pair=await context.request.post(origin+(route==='opt'?'/api/pair':'/api/auth/browser-session'),{data:route==='opt'?{code}:{credential:code},headers:{Origin:origin}});
if(!pair.ok())throw new Error('pair status '+pair.status());const pairing=route==='opt'?await pair.json():null;
await context.addInitScript(({allowed})=>{
 window.measure={events:[],blocked:0};const log=(kind,detail={})=>window.measure.events.push({t:performance.now(),kind,...detail});
 const valid=id=>!id||allowed.includes(id);
 // Wrap the native socket too. no-visits.mjs only wraps later assignments.
 const wrap=Base=>new Proxy(Base,{construct(target,args){const ws=Reflect.construct(target,args);const send=ws.send.bind(ws);const requests=new Map();
 ws.addEventListener('message',e=>{try{const parsed=JSON.parse(e.data);for(const f of Array.isArray(parsed)?parsed:[parsed]){const req=requests.get(f.requestId);if(req)log('ws-in',{tag:req.tag,thread:req.thread,frame:f._tag,bytes:JSON.stringify(f).length});}}catch{}});
 ws.send=data=>{if(typeof data!=='string')return send(data);let parsed;try{parsed=JSON.parse(data);}catch{return send(data);}
 const frames=Array.isArray(parsed)?parsed:[parsed];const kept=[];
 for(const f of frames){if(f._tag!=='Request'){kept.push(f);continue;}const thread=f.payload?.threadId;const visit=f.tag==='orchestration.dispatchCommand'&&f.payload?.type==='thread.visit';
 if(visit){const exit={_tag:'Exit',requestId:f.id,exit:{_tag:'Success',value:{sequence:0}}};setTimeout(()=>{const e=new MessageEvent('message',{data:JSON.stringify(exit)});ws.dispatchEvent(e);if(ws.onmessage)ws.onmessage(e);},0);continue;}
 if(f.tag==='orchestration.dispatchCommand'||(f.tag==='orchestration.subscribeThread'&&!valid(thread))){window.measure.blocked++;continue;}
 const tag=f.tag;const test=thread?allowed.indexOf(thread):undefined;log('ws-out',{tag,thread:test});requests.set(f.id,{tag,thread:test});kept.push(f);}
 if(kept.length)send(JSON.stringify(Array.isArray(parsed)?kept:kept[0]));};return ws;}});
 let socket=wrap(window.WebSocket);Object.defineProperty(window,'WebSocket',{configurable:true,get:()=>socket,set:Base=>socket=wrap(Base)});
 let real=window.fetch;const wrapFetch=fn=>async(input,init)=>{const url=new URL(input instanceof Request?input.url:input,location.href);const m=url.pathname.match(/^\/api\/orchestration\/threads\/([^/]+)/);const id=m?decodeURIComponent(m[1]):null;if(id&&!valid(id)){window.measure.blocked++;throw new Error('non-test thread blocked');}
 const kind=m?'thread':url.pathname==='/api/orchestration/shell'?'shell':url.pathname.includes('auth/session')?'session':url.pathname.includes('.well-known')?'descriptor':url.pathname.includes('websocket-ticket')?'ticket':null;
 const t=performance.now();if(kind)log('fetch-start',{what:kind,thread:id?allowed.indexOf(id):undefined});const r=await fn(input,init);if(kind)log('fetch-end',{what:kind,thread:id?allowed.indexOf(id):undefined,duration:performance.now()-t,status:r.status});return r;};
 let fetch=wrapFetch(real);Object.defineProperty(window,'fetch',{configurable:true,get:()=>fetch,set:fn=>fetch=wrapFetch(fn)});
 document.addEventListener('pointerdown',()=>log('pointer'),true);document.addEventListener('click',()=>log('click'),true);
 new PerformanceObserver(list=>{for(const e of list.getEntries())if(e.duration>50)log('longtask',{start:e.startTime,duration:e.duration});}).observe({entryTypes:['longtask']});
 const seen=new Set();const check=()=>{const mark=k=>{if(!seen.has(k)){seen.add(k);log(k);}};if(document.querySelector('button[aria-label="Toggle main sidebar"]'))mark('app-controls');const search=document.querySelector('[aria-label="Search threads"]');const r=search?.getBoundingClientRect();let opaque=true;for(let n=search;n;n=n.parentElement)if(getComputedStyle(n).opacity!=='1')opaque=false;if(r&&r.width>0&&r.right>0&&r.left<innerWidth&&opaque&&[...document.querySelectorAll('button')].some(e=>/^Settled\s*\d+/.test(e.textContent)))mark('list-painted');requestAnimationFrame(check);};requestAnimationFrame(check);
},{allowed:cfg.tests.map(t=>t.id)});
let page=await context.newPage();
if(pairing){await page.route(origin+'/seed',r=>r.fulfill({contentType:'text/html',body:'<!doctype html><html></html>'}));await page.goto(origin+'/seed');await page.evaluate(async pairing=>{await new Promise((resolve,reject)=>{const r=indexedDB.open('t3opt',1);r.onupgradeneeded=()=>{r.result.createObjectStore('kv');r.result.createObjectStore('objects');};r.onsuccess=()=>{const t=r.result.transaction('kv','readwrite');t.objectStore('kv').put(pairing,'pairing');t.oncomplete=()=>{r.result.close();resolve();};t.onerror=()=>reject(t.error);};r.onerror=()=>reject(r.error);});},pairing);await page.unroute(origin+'/seed');}
const uptime=()=>execSync('uptime').toString().trim();
const row=()=>page.getByText(cfg.tests[0].title,{exact:true}).and(page.locator(':not(.sr-only)')).filter({visible:true}).first();
async function sidebar(){await page.waitForFunction(()=>{const e=document.querySelector('button[aria-label="Toggle main sidebar"]');return e&&e.getBoundingClientRect().width>0;},null,{timeout:120000,polling:16});const search=page.getByRole('combobox',{name:'Search threads'});if(!await search.isVisible())await page.getByRole('button',{name:'Toggle main sidebar'}).first().click({force:true});await page.waitForFunction(()=>window.measure.events.some(e=>e.kind==='list-painted'),null,{timeout:120000,polling:16});}
async function findTest(){await page.getByRole('combobox',{name:'Search threads'}).fill(cfg.tests[0].title);await row().waitFor({timeout:30000});}
async function waitReply(){await page.waitForFunction(word=>{const shown=e=>{for(let n=e;n;n=n.parentElement){const s=getComputedStyle(n);if(s.opacity!=='1'||s.visibility==='hidden')return false;}const r=e.getBoundingClientRect();return r.width>0&&r.height>0&&r.bottom>0&&r.top<innerHeight&&r.right>0&&r.left<innerWidth;};return [...document.querySelectorAll('p,span,div')].some(e=>e.children.length===0&&(e.textContent??'').trim().replace(/\.$/,'').toLowerCase()===word.toLowerCase()&&shown(e));},cfg.tests[0].reply,{timeout:120000,polling:16});}
async function dump(phase,started,load,extra={}){const data=await page.evaluate(()=>({end:performance.now(),events:window.measure.events,blocked:window.measure.blocked,nav:performance.getEntriesByType('navigation').map(x=>({ttfb:x.responseStart,dom:x.domContentLoadedEventEnd,load:x.loadEventEnd})),resources:performance.getEntriesByType('resource').filter(x=>/\.js($|\?)/.test(x.name)).map(x=>({start:x.startTime,end:x.responseEnd,bytes:x.transferSize})),status:window.t3opt?.engine?.status()}));if(data.status){const {link,stats,receivedContentBytes,undecodable}=data.status;data.status={link:link.transport,handshake:link.handshakeMs,srtt:stats.srttMs,loss:stats.loss,bytes:receivedContentBytes,undecodable};}console.log(JSON.stringify({route,size,run,phase,total:Date.now()-started,load,...extra,...data}));}
try{
 const load=uptime(),t=Date.now();await page.goto(origin,{waitUntil:'commit',timeout:120000});await sidebar();await dump('cold-list',t,load);await findTest();
 for(const phase of ['missing','has']){
 if(phase==='has'){await page.goto(origin,{waitUntil:'commit',timeout:120000});await sidebar();await findTest();}
 await page.waitForTimeout(1600);
 const have=await page.evaluate(id=>window.t3opt?.engine?.objects.ofThread(id).filter(x=>x.metaJson.c==='frame'&&x.complete).length??null,cfg.tests[0].id);
 await page.evaluate(()=>{window.measure.events=[];});const load=uptime(),t=Date.now();await row().click({timeout:30000});await waitReply();await dump(phase,t,load,{have});
 }
}catch(e){fs.writeFileSync('/private/debug.json',JSON.stringify(await page.evaluate(()=>({url:location.href,text:document.body.innerText,events:window.measure?.events})),null,2));console.log(JSON.stringify({route,size,run,error:e.name,detail:e.message.split('\n')[0].replaceAll(cfg.tests[0].title,'[test]')}));process.exitCode=1;}
await browser.close();
