const ForkTsCheckerWebpackPlugin = require("fork-ts-checker-webpack-plugin");
const TsconfigPathsPlugin = require("tsconfig-paths-webpack-plugin");
const { CleanWebpackPlugin } = require('clean-webpack-plugin');
const { resolve } = require("path");

module.exports = (env, args) => {
  const CWD = process.cwd();
  const production = (args.mode === "production");

  return {
    entry: resolve(CWD, "assets/javascript/application_v1.tsx"), 

    output: {
      path: resolve(CWD, "public"),
      filename: production ? "application_v1.[contenthash].js" : "application_v1.[fullhash].js",
      clean: true
    },

    experiments: {
      asyncWebAssembly: true,
    },

    devServer: {
      headers: {
        "Access-Control-Allow-Origin": "*",
      },
      liveReload: false,
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
          use: [ "style-loader", "css-loader", "postcss-loader" ]
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
      ],
      fallback: {
        path: require.resolve("path-browserify")
      },
    },

    plugins: [
      new ForkTsCheckerWebpackPlugin({ async: false }),
      new CleanWebpackPlugin(),
    ],
  }
}
