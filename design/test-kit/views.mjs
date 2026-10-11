// v17: every Library / Now Playing state x 51 text sizes (100–200% in 2% steps)
// x plain / WCAG 1.4.12 text spacing, through the page's own auditPhones(),
// all nine phones at once. Also reports how full the non-scrolling pages are.
// Usage: node views.mjs page.html
import { chromium } from 'playwright'; import path from 'path';
const fd = path.resolve('node_modules/@fontsource');
const css = [400,500,600,700,800].map(w=>`@font-face{font-family:'Inter';font-weight:${w};src:url('file://${fd}/inter/files/inter-latin-${w}-normal.woff2') format('woff2');}`).join('')+[400,500,600,700].map(w=>`@font-face{font-family:'Space Grotesk';font-weight:${w};src:url('file://${fd}/space-grotesk/files/space-grotesk-latin-${w}-normal.woff2') format('woff2');}`).join('');
const SPACING='.phone * { line-height: 1.5 !important; letter-spacing: 0.12em !important; word-spacing: 0.16em !important; } .phone p { margin-bottom: 2em !important; }';
const b=await chromium.launch(); const p=await (await b.newContext({viewport:{width:1500,height:1000}})).newPage();
const errs=[]; p.on('pageerror',e=>errs.push(String(e)));
await p.route('**/*', r=>{const u=r.request().url(); if(u.startsWith('file://')||u.startsWith('data:')) return r.continue(); if(u.includes('fonts.googleapis.com')) return r.fulfill({contentType:'text/css',body:css}); return r.abort();});
await p.goto('file://'+path.resolve(process.argv[2])); await p.evaluate(()=>document.fonts.ready); await p.waitForTimeout(200);
const STATES=['lib:albums','lib:albums:album','lib:loops','lib:more','lib:more:advanced','now','layers'];
const sizes=[]; for(let s=100;s<=200;s+=2) sizes.push(s);
let total=0, bad=0;
for (const spacing of [0,1]) {
  const tag = spacing ? await p.addStyleTag({content:SPACING}) : null;
  for (const st of STATES) {
    const fails={}; let minSpare={more:1e9, now:1e9};
    for (const s of sizes) {
      const r=await p.evaluate(([s,st])=>{
        document.documentElement.style.fontSize = s===100 ? '' : s+'%';
        document.querySelectorAll('.phone').forEach(ph=>{ if(st==='layers'||st==='now') setView(ph,st); else { const m=st.split(':'); libGo(ph,m[1],m[2]||''); } });
        fitAll(); updateScrollEdges();
        const a=auditPhones();
        const spare={more:1e9, now:1e9};
        document.querySelectorAll('.phone').forEach(ph=>{
          const dyn=ph.querySelector('.dynamic-inner'), cs=getComputedStyle(dyn); const inner=dyn.getBoundingClientRect().bottom-parseFloat(cs.paddingBottom)-parseFloat(cs.borderBottomWidth);
          const go=ph.querySelector('.go'); if(go && go.offsetParent) spare.more=Math.min(spare.more, inner-go.getBoundingClientRect().bottom);
          const na=ph.querySelector('.np-action'); if(na && na.offsetParent) spare.now=Math.min(spare.now, inner-na.getBoundingClientRect().bottom);
        });
        return {issues:a.issues, spare};
      },[s,st]);
      total++;
      const k=Object.keys(r.issues); if(k.length){ bad++; k.forEach(x=>{(fails[x]=fails[x]||[]).push(s);}); }
      minSpare.more=Math.min(minSpare.more,r.spare.more); minSpare.now=Math.min(minSpare.now,r.spare.now);
    }
    const f=Object.entries(fails).map(([k,v])=>`${k} @ ${v.length>6?v.slice(0,3).join(',')+'…'+v.slice(-2).join(','):v.join(',')}%`).join('; ');
    const sp = st==='lib:more' ? ` | More: least room left below the Advanced key ${minSpare.more.toFixed(1)}px` : st==='now' ? ` | Now Playing: least room left below leave album ${minSpare.now.toFixed(1)}px` : '';
    console.log(`${spacing?'spacing':'plain  '}  ${st.padEnd(18)} ${f||'0 issues'}${sp}`);
  }
  if (tag) await tag.evaluate(t=>t.remove());
}
console.log(`audits: ${total} (x 9 phones), with any issue: ${bad}`);
console.log('page errors:', errs.length?errs:'none');
await b.close();
