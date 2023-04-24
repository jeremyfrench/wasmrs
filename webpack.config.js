const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const CopyPlugin = require("copy-webpack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");

module.exports = {
    entry: { 
	bm: './jssrc/index.js',
	stats: './jssrc/stats.js',
	styles: './scss/main.scss',    
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
        ],
rules: [
       {
              test: /\.css$/,
              use: [MiniCssExtractPlugin.loader, 'style-loader', 'css-loader']
      },
{
  test: /\.(scss)$/,
  use: [
  MiniCssExtractPlugin.loader,{
    // inject CSS to page
    loader: 'style-loader'
  }, {
    // translates CSS into CommonJS modules
    loader: 'css-loader'
  }, {
    // Run postcss actions
    loader: 'postcss-loader',
    options: {
      // `postcssOptions` is needed for postcss 8.x;
      // if you use postcss 7.x skip the key
      postcssOptions: {
        // postcss plugins, can be exported to postcss.config.js
        plugins: function () {
          return [
            require('autoprefixer')
          ];
        }
      }
    }
  }, {
    // compiles Sass to CSS
    loader: 'sass-loader'
  }]
}
	]
  },

    plugins: [
	new MiniCssExtractPlugin({
	  filename: "css/[name].css" 
	}),
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
