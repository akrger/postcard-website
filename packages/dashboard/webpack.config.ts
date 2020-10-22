import path from "path";
import webpack from "webpack";
import WebpackDevServer from "webpack-dev-server";
import ReactRefreshWebpackPlugin from "@pmmmwh/react-refresh-webpack-plugin";
// TODO: Fix this
declare module "webpack" {
  interface Configuration {
    devServer?: WebpackDevServer.Configuration;
  }
}

const config: webpack.Configuration = {
  entry: "./src/index.tsx",
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        exclude: /node_modules/,
        use: {
          loader: "babel-loader",
          options: {
            presets: ["@babel/preset-env"],
            root: "../../",
            plugins: ["react-refresh/babel"],
          },
        },
      },
      {
        test: /\.s[ac]ss$/i,
        use: ["style-loader", "css-loader", "sass-loader"],
      },
    ],
  },
  devtool: "source-map",

  devServer: {
    //index: "",
    //contentBase: "./dist",
    hot: true,
    //host: "127.0.0.1",
    //https: false,
    port: 8082,
    //overlay: true,
    //inline: true,
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
    // alias: {
    //   components: path.resolve("./src/components"),
    //   stores: path.resolve("./src/stores"),
    //   utils: path.resolve("./src/utils"),
    // },
  },
  output: {
    filename: "bundle.js",
    path: path.resolve(__dirname, "dist"),
  },
  plugins: [new ReactRefreshWebpackPlugin()],
};

export default config;
