// Ghi thong tin bun / bun-baseline (Windows + Linux) vao manifest.json va checksums/.
// Dung: node scripts/update-bun-manifest.mjs <thu-muc-chua-bin> <phien-ban-bun>
import { createHash } from 'node:crypto';
import fs from 'node:fs';
import path from 'node:path';

const TARGETS = {
  'x86_64-pc-windows-msvc': {
    bun: { file: 'bun-x86_64-pc-windows-msvc.exe', alias: 'bun.exe' },
    'bun-baseline': { file: 'bun-baseline-x86_64-pc-windows-msvc.exe', alias: 'bun-baseline.exe' },
  },
  'x86_64-unknown-linux-gnu': {
    bun: { file: 'bun-x86_64-unknown-linux-gnu', alias: 'bun' },
    'bun-baseline': { file: 'bun-baseline-x86_64-unknown-linux-gnu', alias: 'bun-baseline' },
  },
};

const [dir, version] = process.argv.slice(2);
if (!dir || !/^\d+\.\d+\.\d+$/.test(version ?? '')) {
  console.error('Dung: update-bun-manifest.mjs <thu-muc-chua-bin> <phien-ban-bun x.y.z>');
  process.exit(1);
}

const root = path.resolve(import.meta.dirname, '..');
const manifestPath = path.join(root, 'manifest.json');
const manifest = JSON.parse(fs.readFileSync(manifestPath, 'utf8'));
fs.mkdirSync(path.join(root, 'checksums'), { recursive: true });

for (const [target, tools] of Object.entries(TARGETS)) {
  const assets = ((manifest.assets ??= {})[target] ??= {});
  for (const [name, { file, alias }] of Object.entries(tools)) {
    const filePath = path.join(dir, file);
    if (!fs.existsSync(filePath)) continue;
    const data = fs.readFileSync(filePath);
    const sha256 = createHash('sha256').update(data).digest('hex');
    assets[name] = { file, alias, version, sha256, size: data.length };
    for (const n of [file, alias]) {
      const line = `${sha256}  ${n}\n`;
      fs.writeFileSync(path.join(root, 'checksums', `${n}.sha256`), line);
      fs.writeFileSync(path.join(dir, `${n}.sha256`), line);
    }
    console.log(`[${target}] ${name}: ${version} ${sha256} ${data.length} bytes`);
  }
}

fs.writeFileSync(manifestPath, JSON.stringify(manifest, null, 2) + '\n');
