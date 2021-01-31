const TerserPlugin = require('terser-webpack-plugin');
const HTMLWebpackPlugin = require('html-webpack-plugin');


module.exports = env => {
    const dev = !env.production;

    return {
        mode: dev ? 'development' : 'production',
        entry: './src/index.ts',
        output: {
            filename: 'bundle.[contenthash:8].js',
            publicPath: '/',
        },
        resolve: {
            extensions: [
                '.ts', '.tsx',  '.js', '.jsx'
            ]
        },
        module: {
            rules: [
                {
                    test: /\.(t|j)sx?$/,
                    exclude: /node_modules/,
                    loader: 'babel-loader',
                },            
            ]
        },   
        plugins: [
            new HTMLWebpackPlugin({
                minify: true,
                inject: false,
                scriptLoading: 'defer',
                template: './src/template.ejs',
                inlineSource: '.css$',
            }),
        ],
        optimization: {
            minimize: !dev,
            minimizer: [
                new TerserPlugin({
                    parallel: true,
                }),
            ],
            splitChunks: {
                chunks: 'all'
            }
        },
        experiments: {
            asyncWebAssembly: true,
        },
        devServer: {
            contentBase: './static',
            compress: true,
            port: 11000
        }
    }
}
