import esbuild from 'esbuild';
import fs from 'fs';

const entryPoint = './wypst.js';
const isDevelopment = process.env.NODE_ENV === 'development';

// Inline wasm build
esbuild.build({
    entryPoints: [entryPoint],
    bundle: true,
    minify: !isDevelopment,
    sourcemap: isDevelopment,
    target: ['es6'],
    outfile: './dist/wypst.min.js',
    format: 'iife',
    globalName: 'wypst',
    loader: {
        '.wasm': 'binary'
    },
});

// Normal build
esbuild.build({
    entryPoints: [entryPoint],
    bundle: true,
    minify: !isDevelopment,
    sourcemap: isDevelopment,
    target: ['es6'],
    outfile: './dist/wypst.js',
    format: 'iife',
    globalName: 'wypst',
    loader: {
        '.wasm': 'file'
    },
    metafile: true,
    assetNames: 'wypst',
});

// Copy CSS files
fs.copyFileSync('node_modules/katex/dist/katex.css', 'dist/wypst.css');
fs.copyFileSync('node_modules/katex/dist/katex.min.css', 'dist/wypst.min.css');
