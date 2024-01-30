import { build } from "esbuild";
import { polyfillNode } from "esbuild-plugin-polyfill-node";

build({
  entryPoints: ['package.js'],
  bundle: true,
  outfile: 'src/package.js',
  format: 'esm',
  // sourcemap: 'inline',
  minify: true,
  plugins: [
		polyfillNode(),
	],
}).catch(() => process.exit(1));