// Run with the 1.64.1 lab driver image. Only throwaway loopback ports.
import fs from 'node:fs';
import {execFileSync} from 'node:child_process';
import {webkit, devices} from '/opt/driver/node_modules/playwright/index.mjs';
const size=process.argv[2]||'safari';
const diagnostic=process.argv[3]||'';
const out=`/data/out/${size}${diagnostic?'-'+diagnostic:''}`;fs.mkdirSync(out,{recursive:true});
console.log(execFileSync('uptime',{encoding:'utf8'}).trim());
const codes=Object.fromEntries(['native','sidecar'].map(mode=>[mode,fs.readFileSync(`/data/code-${mode}`,'utf8').match(/^Token: (.*)$/m)[1]]));
const browser=await webkit.launch();
const contexts=[];
async function open(mode){
 const ctx=await browser.newContext({...devices['iPhone 17 Pro'],viewport:{width:402,height:size==='safari'?681:874},recordVideo:{dir:out,size:{width:402,height:size==='safari'?681:874}}});contexts.push(ctx);
 await ctx.addInitScript(({diagnostic})=>{
  window.r116={frames:[],answers:[],phase:'open',mutations:[]};
  let firstSnapshot=null;
  let completeReleased=false;
  const dispatch=EventTarget.prototype.dispatchEvent;
  EventTarget.prototype.dispatchEvent=function(e){
   if(e.type==='message'&&typeof e.data==='string'){
    try{const f=JSON.parse(e.data);if(f._tag==='Chunk'&&f.values?.some(v=>v.projection?.thread?.id==='research116-long')){
     if(diagnostic==='atomic-history')for(const v of f.values){if(v.projection){
      firstSnapshot??=structuredClone(v);
      const historical=v.projection.turnItems.filter(i=>i.id.startsWith('research116-long-item-'));
      completeReleased ||= historical.length===600 && historical.every(i=>i.text?.startsWith('MARKER ')) && v.projection.messages.filter(m=>m.id.startsWith('research116-long-message-')&&m.text?.startsWith('MARKER ')).length===600;
      if(!completeReleased)Object.assign(v,structuredClone(firstSnapshot));
     }}
     if(diagnostic==='no-standins')for(const v of f.values){if(v.projection){v.projection.turnItems=v.projection.turnItems.filter(i=>!i.id.startsWith('turn-item:message:'));v.projection.visibleTurnItems=v.projection.visibleTurnItems.filter(r=>!r.sourceItemId.startsWith('turn-item:message:')).map((r,position)=>({...r,position}));}}
     window.r116.answers.push({at:performance.now(),phase:window.r116.phase,frame:f});
     if(diagnostic)e=new MessageEvent('message',{data:JSON.stringify(f)});
    }}catch{}
   }return dispatch.call(this,e);
  };
  window.start116=()=>{
   let prev=performance.now();
   const tick=()=>{
    const t=performance.now(),e=document.querySelector('.messages-timeline-scroll');
    if(e){const r=e.getBoundingClientRect();
     const elements=[...e.querySelectorAll('[data-message-id]')];
     const visible=elements.map(x=>({id:x.dataset.messageId,top:x.getBoundingClientRect().top,height:x.getBoundingClientRect().height,text:x.textContent.slice(0,70)})).filter(x=>x.top+x.height>r.top&&x.top<r.bottom);
     window.r116.frames.push({at:t,gap:t-prev,phase:window.r116.phase,top:e.scrollTop,height:e.scrollHeight,viewport:e.clientHeight,visible});
    }prev=t;requestAnimationFrame(tick);
   };requestAnimationFrame(tick);
  };
 },{diagnostic});
 const p=await ctx.newPage();p.on('pageerror',e=>console.log(mode,'error',e.message.slice(0,200)));
 await p.goto(`http://127.0.0.1:${mode==='native'?13804:12504}`);
 await p.locator('#pairing-token').waitFor({timeout:60000});
 console.log(mode,'pairing page ready');
 if(await p.locator('#pairing-token').count()){await p.locator('#pairing-token').fill(codes[mode]);await p.getByRole('button',{name:'Continue',exact:true}).click();}
 await p.getByRole('button',{name:'Toggle main sidebar'}).first().waitFor({timeout:60000}).catch(async e=>{console.log(mode,'login failed',await p.locator('body').innerText());await p.screenshot({path:`${out}/${mode}-failed.png`});throw e;});
 await p.waitForTimeout(1500);
 const close=p.getByRole('button',{name:'Close',exact:true});if(await p.getByText('Connect your computers',{exact:true}).isVisible().catch(()=>false))await close.first().click();
 await p.getByRole('button',{name:'Toggle main sidebar'}).first().click();await p.getByText('research116-long',{exact:true}).last().click();
 await p.locator('.messages-timeline-scroll').waitFor({timeout:60000});
 await p.evaluate(()=>window.start116());
 console.log(mode,'opened',await p.locator('.messages-timeline-scroll').evaluate(e=>({top:e.scrollTop,height:e.scrollHeight})));
 return {p,ctx,mode};
}
try{
 const peers=await Promise.all(['native','sidecar'].map(open));
 const phase=async(name)=>{console.log(size,name,execFileSync('uptime',{encoding:'utf8'}).trim());await Promise.all(peers.map(({p})=>p.evaluate(n=>window.r116.phase=n,name)));};
 // Mobile WebKit exposes no wheel or touch-swipe driver API. Dispatch the
 // navigation signal, then scroll the real viewport, without altering app data.
 const scroll=async(delta,n,delay)=>{for(let i=0;i<n;i++){await Promise.all(peers.map(({p})=>p.locator('.messages-timeline-scroll').evaluate((e,d)=>{e.dispatchEvent(new WheelEvent('wheel',{deltaY:d,bubbles:true}));e.scrollTop+=d;},delta)));await new Promise(r=>setTimeout(r,delay));}};
 await phase('cold-history');await scroll(-220,12,70);
 await phase('cold-hold');await new Promise(r=>setTimeout(r,3000));
 await phase('cold-fling');await scroll(-1800,15,30);await new Promise(r=>setTimeout(r,5000));
 await phase('settle');await new Promise(r=>setTimeout(r,20000));
 await phase('native-load-history');
 const native=peers.find(x=>x.mode==='native').p;
 for(let i=0;i<35;i++){const b=native.getByRole('button',{name:'Load earlier turns',exact:true});if(!await b.count())break;await b.click({force:true});await native.waitForTimeout(150);}
 await new Promise(r=>setTimeout(r,5000));
 await phase('cached-scroll');await scroll(240,30,70);await scroll(-240,30,70);await new Promise(r=>setTimeout(r,3000));
 await phase('cached-fling');await scroll(1800,15,30);await scroll(-1800,15,30);await new Promise(r=>setTimeout(r,5000));
 await phase('hold');await new Promise(r=>setTimeout(r,3000));
 if(!diagnostic){
 await phase('stream-scroll');
 console.log('fixture prompt',execFileSync('/stock-t3/t3',['thread','send','--base-dir','/data/stock','research116-long','Synthetic streaming scroll test.','--json'],{encoding:'utf8'}));
 await new Promise(r=>setTimeout(r,1500));
 await scroll(-220,20,80);await scroll(1800,8,30);
 await phase('stream-hold');await new Promise(r=>setTimeout(r,10000));
 await phase('stream-fling');await scroll(-1800,12,30);await scroll(1800,12,30);
 await new Promise(r=>setTimeout(r,16000));
 }
 for(const {p,mode} of peers){await p.screenshot({path:`${out}/${mode}-end.png`});fs.writeFileSync(`${out}/${mode}.json`,JSON.stringify(await p.evaluate(()=>({...window.r116,trace:window.t3opt?.trace,status:window.t3opt?.engine?.status()}))));console.log(mode,await p.evaluate(()=>{const e=document.querySelector('.messages-timeline-scroll');return e?{top:e.scrollTop,height:e.scrollHeight}:null;}));}
 for(const {p,ctx,mode} of peers){const video=p.video();await ctx.close();await video.saveAs(`${out}/${mode}.webm`);}
 for(const mode of ['native','sidecar'])console.log(execFileSync('node',['/check/assert-scroll.mjs',`${out}/${mode}.json`],{encoding:'utf8'}));
}finally{for(const ctx of contexts)await ctx.close().catch(()=>{});await browser.close();}
