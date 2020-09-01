# satysfi-peg-parser

[SATySFi](https://github.com/gfngfn/SATySFi) の実験的なパーサ．
Rust製のPEGパーサジェネレータである [pest](https://github.com/pest-parser/pest) を使用しています．
SATySFi の文法の一部のみサポートしています．

## TODO

- [ ] `let-mutable`/`let-rec`/`let-inline`/`let-math`
- [ ] module, struct
- [x] ヘッダ
- [x] `if` 式
- [x] `match` 式
- [x] math mode
- [x] 代数的データ型，コンストラクタ
- [ ] 演算子の優先順位
- [ ] `type` の宣言
- [x] レコードのメンバアクセス
- [ ] パス括弧
- [x] リスト・レコード型のコマンド引数
