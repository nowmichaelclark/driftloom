// Contact sheets: phones in chosen views, laid out with captions.
// Usage: node sheet.mjs page.html out.png "views" "themes" [pct] [cols]
//   views  comma list of: layers | lib:albums | lib:albums:album | lib:loops |
//          lib:more | lib:more:advanced | now      (suffix @scroll scrolls to the end)
//   themes comma list of t-* classes, or "all"
// Env: CROP=dyn crops to the Dynamic + Buttons zones; SPACING=1 applies the
//      WCAG 1.4.12 text-spacing override; PAUSED=1 pauses every phone;
//      FC=dark|light renders in forced colours; RM=1 reduced motion.
import { chromium } from 'playwright'; import path from 'path'; import fs from 'fs';
const fd = path.resolve('node_modules/@fontsource');
const css = [400,500,600,700,800].map(w=>`@font-face{font-family:'Inter';font-weight:${w};src:url('file://${fd}/inter/files/inter-latin-${w}-normal.woff2') format('woff2');}`).join('')+[400,500,600,700].map(w=>`@font-face{font-family:'Space Grotesk';font-weight:${w};src:url('file://${fd}/space-grotesk/files/space-grotesk-latin-${w}-normal.woff2') format('woff2');}`).join('');
const [,,file,out,viewsArg='lib:albums',themesArg='all',pct='100',colsArg='']=process.argv;
const b=await chromium.launch(); const ctx=await b.newContext({viewport:{width:1500,height:1000}, deviceScaleFactor:2, reducedMotion: process.env.RM?'reduce':'no-preference'}); const p=await ctx.newPage();
await p.route('**/*', r=>{const u=r.request().url(); if(u.startsWith('file://')||u.startsWith('data:')) return r.continue(); if(u.includes('fonts.googleapis.com')) return r.fulfill({contentType:'text/css',body:css}); return r.abort();});
if (process.env.FC) await p.emulateMedia({forcedColors:'active', colorScheme:process.env.FC});
await p.goto('file://'+path.resolve(file)); await p.evaluate(()=>document.fonts.ready);
await p.addStyleTag({content:'.masthead,section.handover,.spec,.section-title,.sim-toggle,.item-caption,.item-label{display:none !important} .gallery{gap:20px !important} .phone .playhead i.on::before{animation-play-state:paused !important; animation-delay:-1.6s !important}'});
if (process.env.SPACING) await p.addStyleTag({content:'.phone *{line-height:1.5 !important;letter-spacing:0.12em !important;word-spacing:0.16em !important} .phone p{margin-bottom:2em !important}'});
if (pct!=='100') await p.evaluate(v=>{document.documentElement.style.fontSize=v+'%';},pct);
const all=['t-dawn','t-ember','t-tidal','t-dark','t-amoled','t-nightlight','t-nocturne','t-abyss','t-olive'];
const themes=themesArg==='all'?all:themesArg.split(',');
const views=viewsArg.split(',');
const cells=[];
for (const v of views) for (const th of themes) {
  const [view, how] = v.split('@');
  await p.evaluate(([th,view,how,paused])=>{
    const ph=document.querySelector('.phone.'+th);
    if (paused && ph.classList.contains('playing')) ph.querySelector('.btn-play').click();
    if (view==='layers'||view==='now') window.setView(ph,view); else { const m=view.split(':'); window.libGo(ph,m[1],m[2]||''); }
    window.fitAll && window.fitAll();
    ph.querySelectorAll('.scroll').forEach(s=>{ s.scrollTop = how==='scroll' ? s.scrollHeight : 0; });
    window.updateScrollEdges && window.updateScrollEdges();
  },[th,view,how||'',!!process.env.PAUSED]);
  await p.waitForTimeout(120);
  const el=await p.$('.phone.'+th); await el.scrollIntoViewIfNeeded();
  let clip=null;
  if (process.env.CROP==='dyn') { const bb=await el.boundingBox(); const d=await (await p.$('.phone.'+th+' .zone-dynamic')).boundingBox(); clip={x:bb.x, y:d.y-10, width:bb.width, height:bb.y+bb.height-d.y+10}; }
  const buf= clip ? await p.screenshot({clip}) : await el.screenshot();
  cells.push({label:`${th.slice(2)} · ${v}${pct!=='100'?' · '+pct+'%':''}`, src:'data:image/png;base64,'+buf.toString('base64')});
}
const cols=+colsArg || Math.min(cells.length, themes.length);
const html=`<html><body style="margin:0;background:#141418;font:12px Inter,sans-serif;color:#c8c8d0">
<div style="display:grid;grid-template-columns:repeat(${cols},auto);gap:18px 14px;padding:18px;width:max-content">
${cells.map(c=>`<figure style="margin:0"><figcaption style="margin:0 0 6px 4px;letter-spacing:.04em">${c.label}</figcaption><img src="${c.src}" style="width:340px;display:block"></figure>`).join('')}
</div></body></html>`;
const q=await ctx.newPage(); await q.setContent(html); await q.waitForTimeout(200);
await q.screenshot({path:out, fullPage:true, scale: process.env.HALF ? 'css' : 'device'});
console.log('wrote', out, cells.length, 'cells');
await b.close();
