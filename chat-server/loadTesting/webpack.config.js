const path = require('path')
const glob = require('glob')
const {CleanWebpackPlugin} = require('clean-webpack-plugin')
const srcDir = './src'

const entries =
  glob.sync('**/*.test.ts', {cwd: srcDir})
    .map(filePath =>
      [filePath.split('/').join('-').split('.')[0], path.resolve(srcDir, filePath)]
    )

module.exports = {
  mode: 'production',
  entry: Object.fromEntries(entries),
  output: {
    path: path.resolve(__dirname, 'dist'),
    libraryTarget: 'commonjs',
    filename: '[name].bundle.js'
  },
  module: {
    rules: [
      {
        test: /\.ts$/,
        use: 'babel-loader'
      }
    ]
  },
  stats: {
    colors: true
  },
  target: 'web',
  externals: /^(k6|https?\:\/\/)(\/.*)?/,
  devtool: 'source-map',
  plugins: [
    new CleanWebpackPlugin()
  ]
}