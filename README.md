# Usage sample of clap

[![Test](https://github.com/pddg/clap_sample/actions/workflows/test.yaml/badge.svg)](https://github.com/pddg/clap_sample/actions/workflows/test.yaml)

clapを使ったCLIオプション実装のサンプル集

- [positional_arg](./positional_arg/)
  - 位置引数。`cargo run -- hoge --fuga` のようなエスケープされた位置引数を含む。
- [optional_arg](./optional_arg/)
  - 一つの引数を取るオプション引数。例えば `-n HOGE` や `--name HOGE` など。
- [flag](./flag/)
  - 引数を取らないオプション引数。例えば `--verbose` 。
- [multiple_value](./multiple_value/)
  - 一つの引数を取るオプション引数だが、カンマ区切りで複数の値を渡せる。例えば `--fruits apple,orange` 。j
- [choices](./choices/)
  - いくつかの候補から選択するオプション引数。例えばログレベルなど。
- [env_var](./env_var/)
  - 環境変数と引数のマッピング
- [subcommand](./subcommand/)
  - 複数のサブコマンドの実装および、複数のサブコマンドにまたがったグローバルなオプション引数の実装。

## Environment

- Rust 1.64 or later

## Author

- pddg

## License

Apache 2.0 Software License
