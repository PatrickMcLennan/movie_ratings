"use strict";

const common = require("./webpack.common");
const path = require("path");
const { merge } = require("webpack-merge");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const OptimizeCSSAssetsPlugin = require("optimize-css-assets-webpack-plugin");
const { CleanWebpackPlugin } = require("clean-webpack-plugin");
const ScriptExtHtmlWebpackPlugin = require("script-ext-html-webpack-plugin");
const TerserPlugin = require("terser-webpack-plugin");

module.exports = merge(common, {
  mode: `production`,
  module: {
    rules: [
      {
        test: /\.(sa|sc|c)ss/,
        use: [
          { loader: MiniCssExtractPlugin.loader },
          { loader: `css-loader` },
          {
            loader: `postcss-loader`,
          },
          { loader: `sass-loader` },
        ],
      },
    ],
  },
  optimization: {
    minimizer: [
      new OptimizeCSSAssetsPlugin({}),
      new TerserPlugin({
        extractComments: false,
      }),
    ],
  },
  plugins: [
    new HtmlWebpackPlugin({
      filename: `index.html`,
      template: path.resolve(__dirname, `../../ui/template.html`),
      inject: `head`,
      minify: {
        collapseWhitespace: true,
        removeComments: true,
      },
    }),
    new ScriptExtHtmlWebpackPlugin({
      defaultAttribute: "defer",
    }),
    new MiniCssExtractPlugin({
      filename: `[name].[contenthash].css`,
      chunkFilename: `[name].[contenthash].css`,
    }),
    new CleanWebpackPlugin(),
  ],
});
