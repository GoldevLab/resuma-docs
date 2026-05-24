import * as esbuild from 'esbuild';
import { mkdirSync, readdirSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = dirname(fileURLToPath(import.meta.url));
const componentsDir = join(root, 'components');
const outDir = join(root, '..', 'static', 'client');

mkdirSync(outDir, { recursive: true });

const entries = readdirSync(componentsDir).filter((file) => file.endsWith('.ts'));

if (entries.length === 0) {
  console.warn('No client components found in client/components/');
  process.exit(0);
}

for (const file of entries) {
  const name = file.replace(/\.ts$/, '');
  await esbuild.build({
    entryPoints: [join(componentsDir, file)],
    outfile: join(outDir, `${name}.js`),
    bundle: true,
    format: 'esm',
    platform: 'browser',
    target: 'es2022',
    minify: true,
    loader: { '.ts': 'ts' },
  });
  console.log(`built static/client/${name}.js`);
}
