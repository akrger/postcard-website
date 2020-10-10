const path = require("path");

module.exports = {
  entry: "./src/index.tsx",
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: "ts-loader",
        exclude: /node_modules/,
      },
      {
        test: /\.css$/i,
        use: ["style-loader", "css-loader"],
      },
    ],
  },
  devServer: {
    //index: "",
    contentBase: "./dist",
    hot: true,
    //host: "127.0.0.1",
    //https: false,
    port: 8082,
    // proxy: {
    //   context: () => true,
    //   target: "http://localhost:8080",
    //},
    proxy: {
      "/": {
        target: "http://localhost:8080",
        //pathRewrite: { '^/api': '' },
      },
    },
    historyApiFallback: true,
    //publicPath: "/",
  },
  resolve: {
    extensions: [".tsx", ".ts", ".js"],
  },
  output: {
    filename: "bundle.js",
    path: path.resolve(__dirname, "dist"),
  },
};
