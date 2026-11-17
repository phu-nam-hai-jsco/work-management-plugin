// Ghi thong tin bun.exe / bun-baseline.exe vao manifest.json va checksums/.
// Dung: node scripts/update-bun-manifest.mjs <thu-muc-chua-exe> <phien-ban-bun>
import { createHash } from 'node:crypto';
import fs from 'node:fs';
import path from 'node:path';

const TARGET = 'x86_64-pc-windows-msvc';
const TOOLS = {
  bun: { file: `bun-${TARGET}.exe`, alias: 'bun.exe' },
  'bun-baseline': { file: `bun-baseline-${TARGET}.exe`, alias: 'bun-baseline.exe' },
};

const [dir, version] = process.argv.slice(2);
if (!dir || !/^\d+\.\d+\.\d+$/.test(version ?? '')) {
  console.error('Dung: update-bun-manifest.mjs <thu-muc-chua-exe> <phien-ban-bun x.y.z>');
  process.exit(1);
}

const root = path.resolve(import.meta.dirname, '..');
const manifestPath = path.join(root, 'manifest.json');
const manifest = JSON.parse(fs.readFileSync(manifestPath, 'utf8'));
const assets = ((manifest.assets ??= {})[TARGET] ??= {});

for (const [name, { file, alias }] of Object.entries(TOOLS)) {
  const data = fs.readFileSync(path.join(dir, file));
  const sha256 = createHash('sha256').update(data).digest('hex');
  assets[name] = { file, alias, version, sha256, size: data.length };
  for (const n of [file, alias]) {
    const line = `${sha256}  ${n}\n`;
    fs.writeFileSync(path.join(root, 'checksums', `${n}.sha256`), line);
    fs.writeFileSync(path.join(dir, `${n}.sha256`), line);
  }
  console.log(`${name}: ${version} ${sha256} ${data.length} bytes`);
}

fs.writeFileSync(manifestPath, JSON.stringify(manifest, null, 2) + '\n');
