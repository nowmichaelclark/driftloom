// v17: contrast, focus rings and keyboard for the Library and Now Playing views.
// Usage: node views_a11y.mjs page.html [text|marks|rings|keys]   (default: all)
// Text: each kind of new text, every theme, from pixels (text vs the same
// screenshot with the text made transparent), 5th percentile, like words.mjs.
import { chromium } from 'playwright'; import path from 'path'; import { PNG } from 'pngjs';
const fd = path.resolve('node_modules/@fontsource');
const css = [400,500,600,700,800].map(w=>`@font-face{font-family:'Inter';font-weight:${w};src:url('file://${fd}/inter/files/inter-latin-${w}-normal.woff2') format('woff2');}`).join('')+[400,500,600,700].map(w=>`@font-face{font-family:'Space Grotesk';font-weight:${w};src:url('file://${fd}/space-grotesk/files/space-grotesk-latin-${w}-normal.woff2') format('woff2');}`).join('');
const [,,file,only='']=process.argv;
const b=await chromium.launch(); const ctx=await b.newContext({viewport:{width:1500,height:1000}, deviceScaleFactor:2}); const p=await ctx.newPage();
const errs=[]; p.on('pageerror',e=>errs.push(String(e)));
await p.route('**/*', r=>{const u=r.request().url(); if(u.startsWith('file://')||u.startsWith('data:')) return r.continue(); if(u.includes('fonts.googleapis.com')) return r.fulfill({contentType:'text/css',body:css}); return r.abort();});
await p.goto('file://'+path.resolve(file)); await p.evaluate(()=>document.fonts.ready);
await p.addStyleTag({content:'.masthead,section.handover,.spec,.section-title,.item-caption{display:none !important} .phone .playhead i.on::before{display:none !important}'});
const L=c=>{const f=v=>{v/=255;return v<=0.03928?v/12.92:((v+0.055)/1.055)**2.4}; return 0.2126*f(c[0])+0.7152*f(c[1])+0.0722*f(c[2]);};
const CR=(a,b)=>{const x=L(a),y=L(b);return (Math.max(x,y)+0.05)/(Math.min(x,y)+0.05);};
const THEMES=['t-dawn','t-ember','t-tidal','t-dark','t-amoled','t-nightlight','t-nocturne','t-abyss','t-olive'];
async function state(th, st, scroll) {
  await p.evaluate(([th,st,scroll])=>{ const ph=document.querySelector('.phone.'+th);
    if(st==='layers'||st==='now') setView(ph,st); else { const m=st.split(':'); libGo(ph,m[1],m[2]||''); }
    ph.querySelectorAll('.scroll').forEach(s=>s.scrollTop= scroll==='end'? s.scrollHeight:0); updateScrollEdges(); }, [th,st,scroll||'']);
}
async function reveal(el){ await el.evaluate(e=>{ const sc=e.closest('.scroll'); if(sc){ const r=e.getBoundingClientRect(), s=sc.getBoundingClientRect(); if(r.top<s.top+4||r.bottom>s.bottom-4) sc.scrollTop+= (r.top-s.top)-24; } }); await el.scrollIntoViewIfNeeded(); await el.evaluate(e=>window.scrollBy(0, e.getBoundingClientRect().top - innerHeight/2)); }

// ---------------------------------------------------------------- text
const TEXT=[
 ['tab','lib:albums','.tab[aria-selected="false"] .tab-word'], ['tab (selected)','lib:albums','.tab[aria-selected="true"] .tab-word'],
 ['action key','lib:albums','.lib-actions .vkey'], ['row title','lib:albums','.lrow.album:nth-child(4) .lrow-title > span'],
 ['row meta','lib:albums','.lrow.album:nth-child(4) .lrow-meta'], ['row key','lib:albums','.lrow.album:nth-child(4) > .vkey'],
 ['back key','lib:albums:album','.back'], ['album name','lib:albums:album','.hero-title'], ['album meta','lib:albums:album','.hero-meta'],
 ['track no.','lib:albums:album','.lrow.track:nth-child(5) .trk'], ['last track meta','lib:albums:album@end','.lrow.track:last-child .lrow-meta'],
 ['delete key','lib:albums:album@end','.end-keys .vkey'], ['loop meta','lib:loops','.lrows.loops .lrow:nth-child(4) .lrow-meta'],
 ['THEME','lib:more','.more-head .sec'], ['theme name','lib:more','.theme-name'], ['Advanced','lib:more','.go-name'], ['go hint','lib:more','.go-hint'],
 ['FILES','lib:more:advanced','.adv-sec .sec'], ['hint','lib:more:advanced','.adv-sec .hint'], ['setting name','lib:more:advanced','.setrow-name'],
 ['switch "off"','lib:more:advanced','.setrow .vkey[role="switch"] span'], ['diagnostics','lib:more:advanced','.diag'],
 ['trim value','lib:more:advanced@end','.trim-val'], ['colophon','lib:more:advanced@end','.colophon'],
 ['np album','now','.np-album'], ['np 3 of 8','now','.np-of'], ['np time','now','.np-time > span'], ['np Next:','now','.np-next'], ['np next name','now','.np-next b'], ['leave album','now','.np-action'],
];
async function textCR(el){
  await reveal(el);
  const A=PNG.sync.read(await el.screenshot()); const col=await el.evaluate(e=>getComputedStyle(e).color);
  await el.evaluate(e=>e.style.setProperty('color','transparent','important')); const B=PNG.sync.read(await el.screenshot()); await el.evaluate(e=>e.style.removeProperty('color'));
  const rgb=col.match(/[\d.]+/g).slice(0,3).map(Number); const vals=[];
  for(let i=0;i<A.data.length;i+=4){ const d=Math.abs(A.data[i]-B.data[i])+Math.abs(A.data[i+1]-B.data[i+1])+Math.abs(A.data[i+2]-B.data[i+2]); if(d>60) vals.push(CR(rgb,[B.data[i],B.data[i+1],B.data[i+2]])); }
  vals.sort((a,b)=>a-b); return vals.length? vals[Math.floor(vals.length*0.05)] : NaN;
}
if (!only || only==='text') {
  console.log('TEXT CONTRAST (5th percentile of glyph pixels vs what is behind them; 4.5:1 needed)');
  console.log('kind'.padEnd(18)+THEMES.map(t=>t.slice(2,9).padStart(8)).join(''));
  let worst=[99,''];
  for (const [k,st,sel] of TEXT) { const row=[k.padEnd(18)]; const [s,sc]=st.split('@');
    for (const th of THEMES) { await state(th,s,sc); const el=await p.$(`.phone.${th} ${sel}`); const v=await textCR(el); row.push(v.toFixed(2).padStart(8)); if(v<worst[0]) worst=[v,th+' '+k]; }
    console.log(row.join('')); }
  console.log('WORST text:', worst[0].toFixed(2), worst[1]);
}

// ---------------------------------------------------------------- marks
async function shot(el,pad=8){ await reveal(el); const bb=await el.boundingBox(); const clip={x:bb.x-pad,y:bb.y-pad,width:bb.width+2*pad,height:bb.height+2*pad}; return {png:PNG.sync.read(await p.screenshot({clip})), clip, bb}; }
const px=(S,x,y)=>{const X=Math.round((x-S.clip.x)*2), Y=Math.round((y-S.clip.y)*2); const i=(Y*S.png.width+X)*4; return [S.png.data[i],S.png.data[i+1],S.png.data[i+2]];};
async function hiddenShot(el, cssText, pad){ const tag=await p.addStyleTag({content:cssText}); const s=await shot(el,pad); await tag.evaluate(t=>t.remove()); return s; }
// icon pixels vs what is behind them (icon hidden), 5th percentile, like text
async function iconCR(btn, color){ const A=await shot(btn,0); const B=await hiddenShot(btn, '.__x{}', 0); await btn.evaluate(e=>e.querySelector('svg').style.visibility='hidden'); const C=await shot(btn,0); await btn.evaluate(e=>e.querySelector('svg').style.visibility='');
  const rgb=(await btn.evaluate(e=>getComputedStyle(e).color)).match(/[\d.]+/g).slice(0,3).map(Number); const vals=[];
  for(let i=0;i<A.png.data.length;i+=4){ const d=Math.abs(A.png.data[i]-C.png.data[i])+Math.abs(A.png.data[i+1]-C.png.data[i+1])+Math.abs(A.png.data[i+2]-C.png.data[i+2]); if(d>60) vals.push(CR(rgb,[C.png.data[i],C.png.data[i+1],C.png.data[i+2]])); }
  vals.sort((a,b)=>a-b); return vals[Math.floor(vals.length*0.05)]; }
if (!only || only==='marks') {
  console.log('\nNON-TEXT (3:1 needed)');
  console.log('LED: lit LED centre vs the key around it · now-playing glyph vs panel (5th pct) · swatch edge vs panel · selected-swatch ring vs panel · progress fill vs rail · fill vs panel · latched Library key: LED core vs its bezel, bezel vs the key (icons: see icons.mjs)');
  for (const th of THEMES) {
    await state(th,'lib:albums'); const out=[th.padEnd(13)];
    // LED on the selected tab
    { const led=await p.$(`.phone.${th} .tab[aria-selected="true"] .led`); const S=await shot(led,10); const c=[S.bb.x+S.bb.width/2, S.bb.y+S.bb.height/2];
      const fill=px(S,c[0]-12,c[1]); out.push('LED '+CR(px(S,...c),fill).toFixed(2)); }
    // now-playing glyph: pixels vs behind (glyph hidden)
    { const m=await p.$(`.phone.${th} .lrow.current .now-mark`); await reveal(m); const A=await shot(m,0); await m.evaluate(e=>e.style.visibility='hidden'); const B=await shot(m,0); await m.evaluate(e=>e.style.visibility='');
      const rgb=(await m.evaluate(e=>getComputedStyle(e).color)).match(/[\d.]+/g).slice(0,3).map(Number); const v=[];
      for(let i=0;i<A.png.data.length;i+=4){ const d=Math.abs(A.png.data[i]-B.png.data[i])+Math.abs(A.png.data[i+1]-B.png.data[i+1])+Math.abs(A.png.data[i+2]-B.png.data[i+2]); if(d>60) v.push(CR(rgb,[B.png.data[i],B.png.data[i+1],B.png.data[i+2]])); }
      v.sort((a,b)=>a-b); out.push('glyph '+v[Math.floor(v.length*0.05)].toFixed(2)); }
    await state(th,'lib:more');
    { let worstEdge=99, ring=99; for (const sw of await p.$$(`.phone.${th} .sw`)) { const S=await shot(sw,4); const sel=await sw.evaluate(e=>e.getAttribute('aria-checked')==='true');
        // left edge, middle height: panel 2px outside, edge = strongest pixel in the first 1.5px (3 device px) inside
        const y=S.bb.y+S.bb.height/2, panel=px(S,S.bb.x-2,y); let best=0; for (const dx of [0,0.5,1]) best=Math.max(best, CR(px(S,S.bb.x+dx,y),panel));
        if (sel) ring=Math.min(ring,best); else worstEdge=Math.min(worstEdge,best); }
      out.push('swatch edge '+worstEdge.toFixed(2)+' ring '+ring.toFixed(2)); }
    await state(th,'now');
    { const bar=await p.$(`.phone.${th} .np-bar`); const S=await shot(bar,8); const y=S.bb.y+S.bb.height/2;
      const fill=px(S,S.bb.x+S.bb.width*0.12,y), rail=px(S,S.bb.x+S.bb.width*0.75,y), panel=px(S,S.bb.x+S.bb.width*0.12,S.bb.y-6);
      out.push('fill/rail '+CR(fill,rail).toFixed(2)+' fill/panel '+CR(fill,panel).toFixed(2)); }
    await state(th,'lib:albums');
    { const k=await p.$(`.phone.${th} .btn-lib`); const S=await shot(k,0); const led=await k.$('.led'); const lb=await led.boundingBox();
      const c=[lb.x+lb.width/2, lb.y+lb.height/2]; const core=px(S,...c), fill=px(S,c[0]-16,c[1]+10);
      let bez=null, bezC=0; for (let d=lb.width/2+0.25; d<=lb.width/2+1.5; d+=0.25){ const q=px(S,c[0]-d,c[1]); const v=CR(q,fill); if(v>bezC){bezC=v; bez=q;} }
      out.push('latched LED core/bezel '+CR(core,bez).toFixed(2)+' bezel/key '+bezC.toFixed(2)); }
    console.log(out.join(' · '));
  }
}

// ---------------------------------------------------------------- rings
if (!only || only==='rings') {
  console.log('\nFOCUS RING on new controls (5th percentile, each pixel before vs after focus; 3:1 needed)');
  const kinds=[['tab','lib:albums','.tab[aria-selected="true"]'],['row','lib:albums','.lrow.album:nth-child(2) .lrow-main'],['row key','lib:albums','.lrow.album:nth-child(2) > .vkey'],
    ['back','lib:albums:album','.back'],['swatch','lib:more','.sw[aria-checked="false"]'],['Advanced','lib:more','.go'],['switch','lib:more:advanced','.setrow .vkey[role="switch"]'],
    ['leave album','now','.np-action'],['Layers key','lib:albums','.btn-layers'],['Library latched','lib:albums','.btn-lib']];
  await p.keyboard.press('Tab'); let worst=[99,''];
  for (const th of THEMES) { const row=[th.padEnd(13)];
    for (const [k,st,sel] of kinds) { await state(th,st); const el=await p.$(`.phone.${th} ${sel}`); await reveal(el); await p.evaluate(()=>document.activeElement&&document.activeElement.blur());
      const bb=await el.boundingBox(); const clip={x:bb.x-6,y:bb.y-6,width:bb.width+12,height:bb.height+12};
      const A=PNG.sync.read(await p.screenshot({clip})); await p.keyboard.press('Shift'); await el.evaluate(e=>e.focus({preventScroll:true})); const fv=await el.evaluate(e=>e.matches(':focus-visible')); const B=PNG.sync.read(await p.screenshot({clip}));
      // At each point along the straight parts of the perimeter, the strongest before/after change within
      // 0.5–5px outside the box (so the ring is found wherever it sits, and a two-colour ring scores by
      // its better band, WCAG C40); then the 5th percentile over the perimeter.
      const cr=(x,y)=>{ const X=Math.round((x+6)*2), Y=Math.round((y+6)*2); if(X<0||Y<0||X>=A.width||Y>=A.height) return 0; const i=(Y*A.width+X)*4; return CR([A.data[i],A.data[i+1],A.data[i+2]],[B.data[i],B.data[i+1],B.data[i+2]]); };
      const R=Math.min(14,bb.width/2-2,bb.height/2-2), per=[];
      for (let x=R; x<=bb.width-R; x+=1) { let t=0,u=0; for (let d=0.5; d<=5; d+=0.5){ t=Math.max(t,cr(x,-d)); u=Math.max(u,cr(x,bb.height+d)); } per.push(t,u); }
      for (let y=R; y<=bb.height-R; y+=1) { let t=0,u=0; for (let d=0.5; d<=5; d+=0.5){ t=Math.max(t,cr(-d,y)); u=Math.max(u,cr(bb.width+d,y)); } per.push(t,u); }
      per.sort((a,b)=>a-b); const v=per[Math.floor(per.length*0.05)]; if(v<worst[0]) worst=[v,th+' '+k]; row.push(`${k}:${v.toFixed(2)}${fv?'':'(no fv!)'}`); }
    console.log(row.join(' ')); }
  console.log('WORST ring:', worst[0].toFixed(2), worst[1]);
}

// ---------------------------------------------------------------- keys
if (!only || only==='keys') {
  console.log('\nKEYBOARD + ARIA (Dawn)');
  const q=s=>p.$(`.phone.t-dawn ${s}`);
  const act=()=>p.evaluate(()=>{const a=document.activeElement; return (a.getAttribute('role')||a.tagName.toLowerCase())+':'+(a.getAttribute('aria-label')||a.textContent.trim()).slice(0,40);});
  await state('t-dawn','layers');
  await (await q('.btn-lib')).focus(); await p.keyboard.press('Enter');
  console.log('Library via Enter -> view', await p.evaluate(()=>document.querySelector('.t-dawn').dataset.view), '| pressed', await p.evaluate(()=>document.querySelector('.t-dawn .btn-lib').getAttribute('aria-pressed')), '| focus stays on', await act());
  const stops=[]; for (let k=0;k<9;k++){ await p.keyboard.press('Tab'); stops.push(await act()); } console.log('next tab stops:', stops.join(' | '));
  await (await q('.tab[aria-selected="true"]')).focus(); await p.keyboard.press('ArrowRight'); console.log('Tabs ArrowRight ->', await act(), '| selected', await p.evaluate(()=>document.querySelector('.t-dawn .tab[aria-selected="true"]').dataset.tab));
  await p.keyboard.press('End'); console.log('Tabs End ->', await act()); await p.keyboard.press('ArrowRight'); console.log('Tabs ArrowRight wraps ->', await act());
  await p.keyboard.press('ArrowRight'); await p.keyboard.press('ArrowRight');
  await p.keyboard.press('Tab'); console.log('Tab from the More tab ->', await act());
  await p.keyboard.press('ArrowRight'); console.log('Theme ArrowRight ->', await act(), '| phone now', await p.evaluate(()=>[...document.querySelectorAll('.phone')][0].className.match(/(?:^| )(t-[a-z]+)/)[1]), '| caption', await p.evaluate(()=>document.querySelectorAll('.theme-name')[0].textContent));
  await p.keyboard.press('ArrowLeft'); console.log('Theme ArrowLeft ->', await act(), '| phone now', await p.evaluate(()=>[...document.querySelectorAll('.phone')][0].className.match(/(?:^| )(t-[a-z]+)/)[1]));
  await p.keyboard.press('Tab'); console.log('Tab out of the radio group ->', await act());
  await p.keyboard.press('Enter'); console.log('Enter on Advanced -> focus', await act());
  await (await q('.setrow .vkey[role="switch"]')).focus(); await p.keyboard.press('Space'); console.log('Lighter switch via Space ->', await p.evaluate(()=>{const s=document.querySelector('.t-dawn .setrow [role=switch]'); return s.getAttribute('aria-checked')+' "'+s.textContent+'"';}));
  await (await q('[data-trim="5"]')).focus(); await p.keyboard.press('Enter'); await p.keyboard.press('Enter'); console.log('trim + twice ->', await p.evaluate(()=>document.querySelector('.t-dawn .trim-val').textContent));
  await (await q('.page[data-page="advanced"] .back')).focus(); await p.keyboard.press('Enter'); console.log('Back -> focus', await act());
  await (await q('.tab[data-tab="albums"]')).click(); await (await q('.lrow.current .lrow-main')).focus(); await p.keyboard.press('Enter'); console.log('Open album -> focus', await act());
  await (await q('.end-keys .vkey')).focus(); await p.keyboard.press('Enter'); console.log('delete album, first press ->', await act());
  await (await q('.page[data-page="album"] .back')).focus(); await p.keyboard.press('Enter'); console.log('Back -> focus', await act());
  await (await q('.btn-layers')).focus(); await p.keyboard.press('Enter'); console.log('Layers key -> view', await p.evaluate(()=>document.querySelector('.t-dawn').dataset.view), '| Layers key hidden', await p.evaluate(()=>document.querySelector('.t-dawn .btn-layers').hidden), '| focus', await act());
  await (await q('.btn-now')).focus(); await p.keyboard.press('Space'); console.log('Now playing via Space -> view', await p.evaluate(()=>document.querySelector('.t-dawn').dataset.view));
  console.log('aria (Now Playing):\n'+await p.locator('.t-dawn .v-now').ariaSnapshot());
  await state('t-dawn','lib:albums');
  const snap=await p.locator('.t-dawn .v-library').ariaSnapshot(); console.log('aria (Library, Albums; first 22 lines):\n'+snap.split('\n').slice(0,22).join('\n'));
  await state('t-dawn','lib:more'); console.log('aria (More):\n'+await p.locator('.t-dawn .v-library .page.more').ariaSnapshot());
  console.log('aria (Buttons zone, in Library):\n'+await p.locator('.t-dawn .zone-buttons').ariaSnapshot());
}
console.log('\npage errors:', errs.length?errs:'none');
await b.close();
