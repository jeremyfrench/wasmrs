const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const CopyPlugin = require("copy-webpack-plugin");

module.exports = {
    entry: { 
	bm: './jssrc/index.js',
	stats: './jssrc/stats.js',
    },
    output: {
        path: path.resolve(__dirname, '../dist'),
        filename: '[name].js',
        libraryTarget: 'var',
    library: 'EntryPoint'

    },
module: {
    noParse: [
         /benchmark/,
        ]
  },

    plugins: [
        new CopyPlugin({
            patterns: [
                { from: 'static' }
            ]
        }),
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, ".")
        }),
        // Have this example work in Edge which doesn't ship `TextEncoder` or
        // `TextDecoder` at this time.
        new webpack.ProvidePlugin({
          TextDecoder: ['text-encoding', 'TextDecoder'],
          TextEncoder: ['text-encoding', 'TextEncoder']
        })
    ],
    devServer: {
	    host: '0.0.0.0',
	    port: 8080,
	    allowedHosts: "all",
    },
    mode: 'development',
    experiments: {
	    asyncWebAssembly: true
    }
};
