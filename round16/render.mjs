// node render.mjs in.svg out.png size [viewBox]
import { createRequire } from 'module'; const { chromium } = createRequire(process.cwd() + '/')('playwright');
import fs from 'fs';
const [,, inp, out, size='512', vb] = process.argv;
let svg = fs.readFileSync(inp, 'utf8');
if (vb) svg = svg.replace(/viewBox="[^"]*"/, `viewBox="${vb}"`);
const s = +size;
svg = svg.replace('<svg ', `<svg width="${s}" height="${s}" `);
const b = await chromium.launch(); const p = await b.newPage({ viewport: { width: s, height: s }, deviceScaleFactor: 1 });
await p.setContent(`<html><body style="margin:0">${svg}</body></html>`);
await p.screenshot({ path: out, clip: { x: 0, y: 0, width: s, height: s } });
await b.close();
