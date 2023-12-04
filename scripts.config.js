const path = require('path');
const ShellPlugin = require('webpack-shell-plugin-next');

const scriptsDir = path.resolve(__dirname, 'scripts');
const symbolGenFilename = 'symbol_gen.js';

const symbolSourceConfig = {
    mode: 'development',
    target: 'node',
    entry: path.resolve(__dirname, 'node_modules/katex/src/symbols.js'),
    output: {
        path: path.resolve(scriptsDir, 'dist'),
        filename: 'symbols.js'
    },
    devtool: 'inline-source-map',
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
}

const symbolGenConfig = {
    mode: 'development',
    target: 'node',
    entry: path.resolve(scriptsDir, symbolGenFilename),
    output: {
        path: path.resolve(scriptsDir, 'dist'),
        filename: symbolGenFilename
    },
    plugins: [
        new ShellPlugin({
            onBuildEnd: {
                scripts: [`node ${path.resolve(scriptsDir, 'dist', symbolGenFilename)}`],
                blocking: true,
                parallel: false
            }
        })
    ]
}

module.exports = [symbolSourceConfig, symbolGenConfig]
