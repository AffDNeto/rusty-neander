const path = require('path');
const webpack = require('webpack');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const HtmlWebpackPartialsPlugin = require('html-webpack-partials-plugin');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const ExtraWatchWebpackPlugin = require('extra-watch-webpack-plugin');
const { CleanWebpackPlugin } = require('clean-webpack-plugin');

package = require('./package.json');

module.exports = {
    devtool: 'eval-cheap-source-map',
    entry: {
        app: path.resolve(__dirname, 'index.js'),
        vendor: Object.keys(package.dependencies),
        neander: path.resolve(__dirname, 'neander.js'),
        ahmes: path.resolve(__dirname, 'ahmes.js'),
        ramses: path.resolve(__dirname, 'ramses.js'),
        cesar: path.resolve(__dirname, 'cesar.js')
    },
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: '[name].[hash].js',
    },
    plugins: [
        new CleanWebpackPlugin(),
        new HtmlWebpackPlugin({
            hash: true,
            template: './index.html',
            chunks: ['vendor', 'app'],
            filename: "./index.html"
        }),
        new HtmlWebpackPlugin({
            hash: true,
            template: './neander.html',
            chunks: ['vendor', 'neander'],
            filename: "./neander.html"
        }),
        new HtmlWebpackPlugin({
            hash: true,
            template: './ahmes.html',
            chunks: ['vendor', 'ahmes'],
            filename: "./ahmes.html"
        }),
        new HtmlWebpackPlugin({
            hash: true,
            template: './ramses.html',
            chunks: ['vendor', 'ramses'],
            filename: "./ramses.html"
        }),
        new HtmlWebpackPlugin({
            hash: true,
            template: './cesar.html',
            chunks: ['vendor', 'cesar'],
            filename: "./cesar.html"
        }),
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, "../crate-wasm")
        }),
        // Have this example work in Edge which doesn't ship `TextEncoder` or
        // `TextDecoder` at this time.
        new webpack.ProvidePlugin({
          TextDecoder: ['text-encoding', 'TextDecoder'],
          TextEncoder: ['text-encoding', 'TextEncoder']
        }),
        new ExtraWatchWebpackPlugin({
          files: ['./*.html']
        })
    ],
    module: {
        rules: [
            {
                test: /\.css$/,
                use: [
                    'style-loader',
                    'css-loader'
                ]
            },
            {
                test: /\.(scss)$/,
                use: [{
                    loader: 'style-loader', // inject CSS to page
                }, {
                    loader: 'css-loader', // translates CSS into CommonJS modules
                }, {
                    loader: 'postcss-loader', // Run post css actions
                    options: {
                        plugins: function () { // post css plugins, can be exported to postcss.config.js
                            return [
                                require('precss'),
                                require('autoprefixer')
                            ];
                        }
                    }
                }, {
                    loader: 'sass-loader' // compiles Sass to CSS
                }]
            },
            {
                test: /\.html$/,
                use: ['html-loader']
            }
        ]
    }
};

