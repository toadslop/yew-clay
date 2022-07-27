import resolve from "@rollup/plugin-node-resolve";
import commonjs from "@rollup/plugin-commonjs";
import typescript from "@rollup/plugin-typescript";
import dts from "rollup-plugin-dts";
import replace from "@rollup/plugin-replace";
import scss from "rollup-plugin-scss";

const packageJson = require("./package.json");

export default [
  {
    input: "src/index.ts",
    output: [
      {
        file: packageJson.module,
        format: "esm",
        sourcemap: true,
      },
    ],
    plugins: [
      resolve(),
      commonjs(),
      typescript({ tsconfig: "./tsconfig.json" }),
      replace({
        "process.env.NODE_ENV": JSON.stringify("production"),
      }),
      scss(),
    ],
  },
  {
    input: "src/build/types/index.d.ts",
    output: [{ file: "src/build/index.d.ts", format: "esm" }],
    plugins: [dts()],
  },
];
