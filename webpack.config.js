const ForkTsCheckerWebpackPlugin = require("fork-ts-checker-webpack-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin")
const TsconfigPathsPlugin = require("tsconfig-paths-webpack-plugin");
const { CleanWebpackPlugin } = require('clean-webpack-plugin');
const { resolve } = require("path");

module.exports = (env, args) => {
  const CWD = process.cwd();
  const production = (args.mode === "production");

  return {
    entry: resolve(CWD, "assets/javascript/entry/application_v1.tsx"), 

    output: {
      path: resolve(CWD, "public"),
      filename: production ? "[name].[contenthash].js" : "[name].[fullhash].js",
      clean: true
    },

    devServer: {
      static: {
        directory: resolve(CWD, "public"),
        publicPath: "/static",
      },
      devMiddleware: {
        writeToDisk: true
      },
      hot: true
    },

    module: {
      rules: [
        // CSS
        {
          test: /\.css$/,
          exclude: /node_modules/,
          use: [
            { loader: 'style-loader', },
            { loader: 'css-loader', options: { importLoaders: 1 } },
            { loader: 'postcss-loader' }
          ]
        },
        // TypeScript
        {
          test: /\.(js|jsx|tsx|ts)$/,
          exclude: /node_modules/,
          use: {
            loader: "babel-loader",
            options: {
              presets: [
                "@babel/preset-env",
                "@babel/preset-react",
                "@babel/preset-typescript"
              ]
            }
          }
        },
      ]
    },

    resolve: {
      extensions: ["*", ".js", ".jsx", ".ts", ".tsx"],
      plugins: [
        new TsconfigPathsPlugin({
          configFile: resolve(CWD, "tsconfig.json"),
          extensions: [".js", ".jsx", ".json", ".ts", ".tsx"]
        })
      ]
    },

    plugins: [
      new HtmlWebpackPlugin({
        template: resolve(CWD, "assets/html/layouts/application_v1.html"),
        filename: resolve(CWD, "assets/html/layouts/dist/application_v1.html"),
        publicPath: "/static"
      }),
      new ForkTsCheckerWebpackPlugin({ async: false }),
      new CleanWebpackPlugin(),
    ],
  }
}
