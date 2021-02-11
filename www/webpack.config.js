const path = require('path');
const webpack = require('webpack');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const HtmlWebpackPartialsPlugin = require('html-webpack-partials-plugin');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const ExtraWatchWebpackPlugin = require('extra-watch-webpack-plugin');
const { CleanWebpackPlugin } = require('clean-webpack-plugin');

module.exports = {
    entry: path.resolve(__dirname, 'index.js'),
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: '[name].bundle.js',
    },
    plugins: [
        new CleanWebpackPlugin(),
        new HtmlWebpackPlugin({
            template: './index.html'
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
        new HtmlWebpackPartialsPlugin({
          path: './neander.html',
          location: "neanderdiv"
        }),
        new HtmlWebpackPartialsPlugin({
          path: './ahmes.html',
          location: "ahmesdiv"
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
                test: /\.html$/,
                use: ['html-loader']
            }
        ]
    }
};

