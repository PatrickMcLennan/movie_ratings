"use strict";

const path = require("path");
const Dotenv = require("dotenv-webpack");

module.exports = {
  entry: {
    app: path.resolve(__dirname, `../entryPoint.ts`),
  },
  output: {
    path: path.resolve(__dirname, `../dist`),
    filename: `app.[contenthash].js`,
  },
  module: {
    rules: [
      {
        test: /\.(ts|tsx|js|jsx)$/,
        exclude: /(node_modules)/,
        use: `swc-loader`,
      },
      {
        test: /\.mjs/,
        resolve: {
          fullySpecified: false,
        },
      },
      {
        test: /\.(png|svg|jpg|jpeg|gif)$/i,
        type: "asset/resource",
      },
      {
        test: /\.(woff|woff2|eot|ttf|otf)$/i,
        type: "asset/resource",
      },
    ],
  },
  plugins: [
    new Dotenv({
      path: path.resolve(__dirname, `../../.env`),
    }),
  ],
  resolve: {
    extensions: [".js", ".mjs", ".jsx", ".ts", ".tsx"],
  },
};
