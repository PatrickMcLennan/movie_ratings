"use strict";

const common = require("./webpack.common");
const { merge } = require("webpack-merge");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const path = require("path");
const { config } = require("dotenv");

config({ path: path.resolve(__dirname, `../../../.env`) });

module.exports = merge(common, {
  mode: `development`,
  devServer: {
    client: {
      overlay: {
        errors: true,
        warnings: false,
      },
    },
    compress: true,
    historyApiFallback: true,
    hot: true,
    open: true,
    host: "0.0.0.0",
    port: 3000,
  },
  module: {
    rules: [
      {
        test: /\.(sa|sc|c)ss/,
        use: [
          { loader: `style-loader` },
          {
            loader: `css-loader`,
            options: {
              sourceMap: true,
            },
          },
          {
            loader: `sass-loader`,
            options: {
              sourceMap: true,
            },
          },
        ],
      },
    ],
  },
  devtool: `source-map`,
  plugins: [
    new HtmlWebpackPlugin({
      filename: `index.html`,
      template: path.resolve(__dirname, `../template.html`),
      inject: `body`,
    }),
  ],
});
