// Icon contrast in the Buttons zone: icon pixels vs the key under them
// (icon hidden), 5th percentile, every key, every theme. 3:1 needed.
// Usage: node icons.mjs page.html [view]   view: layers (default) | lib:albums
import { chromium } from 'playwright'; import path from 'path'; import { PNG } from 'pngjs';
const [,,file,view='layers']=process.argv;
const b=await chromium.launch(); const p=await (await b.newContext({viewport:{width:1500,height:1000}, deviceScaleFactor:2})).newPage();
await p.route('**/*', r=>{const u=r.request().url(); return (u.startsWith('file://')||u.startsWith('data:'))?r.continue():r.abort();});
await p.goto('file://'+path.resolve(file)); await p.addStyleTag({content:'.masthead,section.handover,.spec,.section-title{display:none !important}'});
const L=c=>{const f=v=>{v/=255;return v<=0.03928?v/12.92:((v+0.055)/1.055)**2.4}; return 0.2126*f(c[0])+0.7152*f(c[1])+0.0722*f(c[2]);};
const CR=(a,b)=>{const x=L(a),y=L(b);return (Math.max(x,y)+0.05)/(Math.min(x,y)+0.05);};
const THEMES=['t-dawn','t-ember','t-tidal','t-dark','t-amoled','t-nightlight','t-nocturne','t-abyss','t-olive'];
const keys=[['prev','.row-1 .btn:first-child'],['play','.btn-play'],['next','.row-1 .btn:last-child'],['lib','.btn-lib'],['save','.row-2 .btn:last-child'],['now','.btn-now'],['sound','.btn-sound'],['layers','.btn-layers']];
let worst=[99,''];
for (const th of THEMES) { const row=[th.padEnd(13)];
  await p.evaluate(([th,v])=>{const ph=document.querySelector('.phone.'+th); if(v==='layers'||v==='now') setView(ph,v); else {const m=v.split(':'); libGo(ph,m[1],m[2]||'');}},[th,view]);
  for (const [k,sel] of keys) { const el=await p.$(`.phone.${th} ${sel}`); if(!(await el.isVisible())) continue; await el.scrollIntoViewIfNeeded();
    const A=PNG.sync.read(await el.screenshot()); await el.evaluate(e=>e.querySelectorAll('svg').forEach(s=>s.style.visibility='hidden')); const B=PNG.sync.read(await el.screenshot()); await el.evaluate(e=>e.querySelectorAll('svg').forEach(s=>s.style.visibility=''));
    const rgb=(await el.evaluate(e=>getComputedStyle(e).color)).match(/[\d.]+/g).slice(0,3).map(Number); const v=[];
    for(let i=0;i<A.data.length;i+=4){ const d=Math.abs(A.data[i]-B.data[i])+Math.abs(A.data[i+1]-B.data[i+1])+Math.abs(A.data[i+2]-B.data[i+2]); if(d>60) v.push(CR(rgb,[B.data[i],B.data[i+1],B.data[i+2]])); }
    v.sort((a,b)=>a-b); const r=v[Math.floor(v.length*0.05)]; if(r<worst[0]) worst=[r,th+' '+k]; row.push(`${k}:${r.toFixed(2)}`); }
  console.log(row.join(' ')); }
console.log('WORST icon:', worst[0].toFixed(2), worst[1]);
await b.close();
