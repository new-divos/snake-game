const path = require("path");
const CopyWebpackPlugin = require("copy-webpack-plugin");

const publicPath = path.resolve(__dirname, "public");

module.exports = {
    entry: "./bootstrap.js",
    output: {
        path: publicPath,
        filename: "bootstrap.js"
    },
    mode: "development", 

    module: {
        rules: [
            {
                test: /\.tsx?$/,
                use: 'ts-loader',
                exclude: /node_modules/,
            },
        ],
    },
    resolve: {
        extensions: ['.tsx', '.ts', '.js'],
    },

    plugins: [
        new CopyWebpackPlugin({
            patterns: [
                {
                    from: "./index.html",
                    to: path.resolve(publicPath, "index.html")
                }
            ]
        })
    ]
}