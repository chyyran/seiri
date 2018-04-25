module.exports = {
    mode: "development",
    devtool: "source-map",
    entry: "./seiri-worker.ts",
    output: {
      filename: "seiri-worker.js"
    },
    resolve: {
      // Add `.ts` and `.tsx` as a resolvable extension.
      extensions: [".ts", ".tsx", ".js"]
    },
    module: {
      rules: [
        // all files with a `.ts` or `.tsx` extension will be handled by `ts-loader`
        { test: /\.ts?$/, loader: "ts-loader" }
      ]
    }
  };

// const webpack = require("webpack")
// const path = require("path")
// const fs = require("fs")

// module.exports = {
//     entry: [
//         "./seiri-worker.ts",
//     ],

//     output: {
//         path: path.join(__dirname, "dist"),
//         filename: "seiri-worker.js",
//         publicPath: "/static/",
//     },

//     // Enable sourcemaps for debugging webpack's output.
//     devtool: "source-map",

//     resolve: {
//         // Add '.ts' and '.tsx' as resolvable extensions.
//         extensions: [".ts", ".tsx", ".js"],
//     },

//     plugins: [
//         new webpack.DefinePlugin({
//             "process.env": {
//                 "NODE_ENV": JSON.stringify("production")
//             }
//         }),
//     ],
//     module: {
//         rules: [{
//             test: /\.ts?$/,
//             exclude: path.resolve(__dirname, "node_modules"),
//             include: path.resolve(__dirname, "src"),
//             include: path.resolve(__dirname),
//             use: [{
//                 loader: "ts-loader",
//             }],
//         }],
//     },
//     node: {
//         console: false,
//         global: true,
//         process: true,
//     },
//     // When importing a module whose path matches one of the following, just
//     // assume a corresponding global variable exists and use that instead.
//     // This is important because it allows us to avoid bundling all of our
//     // dependencies, which allows browsers to cache those libraries between builds.
//     externals: {

//     },

// };