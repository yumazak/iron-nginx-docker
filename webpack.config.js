module.exports = {
  entry: './src/index.js',
  output: {
  //  出力ファイルのディレクトリ名
    path: `${__dirname}/public/js`,
    // 出力ファイル名
    filename: 'bundle.js'
  },
  module: {
    rules: [
      {
        test: /\.js$/,
        use: [
          {
            loader: 'babel-loader',
            options: {
              presets: [
                'env', 'react'
              ],
              sourceMap: true
            }
          }
        ],
        exclude: /node_modules/
      }
    ]
  },
  watchOptions: {
    poll: true
  }
}
