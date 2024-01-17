module.exports = {
    entry: './katex.js',
    output: {
        path: __dirname + '/dist',
        filename: 'bundle.js',
        library: 'katex',
        libraryTarget: 'umd',
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
        ]
    },
    mode: 'development'
};
