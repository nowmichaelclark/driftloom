// v17: the light-theme playhead glint. Checks that it twinkles only while
// playing, is gone when paused, is still under reduced motion, and is gone in
// forced colours; and how much of the lamp's ring it covers at its brightest.
// The lamp itself is measured by nontext.mjs (glint hidden).
// Usage: node glint.mjs page.html
import { chromium } from 'playwright'; import path from 'path'; import { PNG } from 'pngjs';
const file=process.argv[2];
const b=await chromium.launch();
async function page(opts={}, media={}) { const p=await (await b.newContext({viewport:{width:1500,height:1000}, deviceScaleFactor:2, ...opts})).newPage();
  await p.route('**/*', r=>{const u=r.request().url(); return (u.startsWith('file://')||u.startsWith('data:'))?r.continue():r.abort();});
  if (Object.keys(media).length) await p.emulateMedia(media);
  await p.goto('file://'+path.resolve(file)); await p.addStyleTag({content:'.masthead,section.handover,.spec,.section-title{display:none !important}'}); return p; }
const st=(p,th)=>p.evaluate(th=>{const c=getComputedStyle(document.querySelector('.'+th+' .playhead i.on'),'::before'); return {content:c.content, display:c.display, anim:c.animationName, dur:c.animationDuration, op:(+c.opacity).toFixed(2)};},th);
const light=['t-dawn','t-ember','t-tidal','t-olive'], dark=['t-dark','t-amoled','t-nightlight','t-nocturne','t-abyss'];
let p=await page();
for (const th of light) { const a=await st(p,th); console.log(th.padEnd(13),'playing:',`anim ${a.anim} ${a.dur}`); }
console.log('dark themes have a glint:', (await Promise.all(dark.map(th=>st(p,th)))).some(a=>a.content!=='none' && a.content!=='normal'));
// opacity over one cycle (sampled), Dawn
const ops=[]; for (let i=0;i<9;i++){ ops.push((await st(p,'t-dawn')).op); await p.waitForTimeout(500); } console.log('Dawn opacity every 0.5 s:', ops.join(' '));
await p.click('.t-dawn .btn-play'); await p.waitForTimeout(300); const pz=await st(p,'t-dawn'); console.log('paused: anim', pz.anim, 'opacity', pz.op);
await p.click('.t-dawn .btn-play'); await p.waitForTimeout(600);
// coverage at peak: pixels on the lamp's ring band that differ with / without the glint
await p.addStyleTag({content:'.phone .playhead i.on::before{animation-play-state:paused !important;animation-delay:-1.6s !important}'});
for (const th of light) { const el=await p.$('.'+th+' .playhead i.on'); await el.scrollIntoViewIfNeeded(); const bb=await el.boundingBox();
  const clip={x:bb.x-6,y:bb.y-8,width:bb.width+12,height:bb.height+16};
  const A=PNG.sync.read(await p.screenshot({clip})); const tag=await p.addStyleTag({content:`.${th} .playhead i.on::before{display:none !important}`}); const B=PNG.sync.read(await p.screenshot({clip})); await tag.evaluate(t=>t.remove());
  // ring band: 0..2px outside the lamp (lamp = cell grown 2px top and bottom)
  let ring=0, hit=0, fill=0, fhit=0, fhit10=0; const L={x:6,y:6,w:bb.width,h:bb.height+4};
  for(let y=0;y<A.height;y++)for(let x=0;x<A.width;x++){ const cx=x/2, cy=y/2; const dx=Math.max(L.x-cx,cx-(L.x+L.w),0), dy=Math.max(L.y-cy,cy-(L.y+L.h),0); const d=Math.max(dx,dy);
    const i=(y*A.width+x)*4; const diff=Math.abs(A.data[i]-B.data[i])+Math.abs(A.data[i+1]-B.data[i+1])+Math.abs(A.data[i+2]-B.data[i+2]);
    if(d>0.25&&d<=2){ring++; if(diff>30) hit++;} if(d===0){fill++; if(diff>30) fhit++; if(diff>10) fhit10++;} }
  console.log(th.padEnd(13),`at its brightest the glint changes ${(hit/ring*100).toFixed(0)}% of the ring and ${(fhit/fill*100).toFixed(0)}% of the lamp's fill (${(fhit10/fill*100).toFixed(0)}% counting any visible change)`);
}
const r=await page({reducedMotion:'reduce'}); const a=await st(r,'t-dawn'); console.log('reduced motion: anim', a.anim, 'opacity', a.op);
const f=await page({}, {forcedColors:'active'}); const fc=await st(f,'t-dawn'); console.log('forced colours: display', fc.display);
await b.close();
