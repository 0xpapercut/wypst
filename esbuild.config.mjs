import esbuild from 'esbuild';

esbuild.build({
    entryPoints: ['./wypst.js'],
    bundle: true,
    minify: true,
    sourcemap: true,
    target: ['es6'],
    outfile: './dist/wypst_inlined.js',
    format: 'iife',
    globalName: 'wypst',
    loader: {
        '.wasm': 'binary'
    },
});
