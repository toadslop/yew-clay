const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const CopyWebpackPlugin = require("copy-webpack-plugin");

const distPath = path.resolve(__dirname, "./dist");
module.exports = (_env, argv) => {
  return {
    devServer: {
      static: distPath,
      compress: argv.mode === "production",
      port: 8000,
    },
    entry: "./bootstrap.js",
    output: {
      path: distPath,
      filename: "alert.js",
      webassemblyModuleFilename: "alert.wasm",
    },
    module: {
      rules: [
        {
          test: /\.s[ac]ss$/i,
          use: ["style-loader", "css-loader", "sass-loader"],
        },
        {
          test: /\.wasm$/,
          type: "webassembly/sync",
        },
      ],
    },
    plugins: [
      new CopyWebpackPlugin({
        patterns: [{ from: "./static", to: distPath }],
      }),
      new WasmPackPlugin({
        crateDirectory: ".",
        extraArgs: "--no-typescript",
      }),
    ],
    experiments: {
      syncWebAssembly: true,
    },
  };
};
