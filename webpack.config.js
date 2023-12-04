const path = require('path');
const HtmlPlugin = require('html-webpack-plugin');
const WasmPlugin = require('@wasm-tool/wasm-pack-plugin');

const libConfig = (env, argv) => ({
    mode: 'development',
    entry: {
        lib: './src/lib.js'
    },
    output: {
        path: __dirname + '/dist',
        filename: '[name].[contenthash].js',
        library: 'wypst',
        libraryTarget: 'umd',
    },
    plugins: [
        new HtmlPlugin({
            template: __dirname + '/src/debug.html'
        }),
        new WasmPlugin({
            crateDirectory: __dirname + '/src/core',
            outDir: __dirname + '/src/core/pkg',
            outName: 'core'
        })
    ],
    devtool: argv.mode === 'production' ? false : 'source-map',
    devServer: {
        port: 8080,
    },
    experiments: {
        asyncWebAssembly: true,
    },
    module: {
        rules: [
            {
                test: /\.js$/,
                exclude: /node_modules\/(?!katex)/,
                use: {
                    loader: 'babel-loader',
                    options: {
                        presets: ['@babel/preset-flow'],
                    },
                },
            }
        ],
    },
});

module.exports = [libConfig];
