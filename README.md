# satysfi-peg-parser

[SATySFi](https://github.com/gfngfn/SATySFi) の実験的なパーサ．
Rust製のPEGパーサジェネレータである [pest](https://github.com/pest-parser/pest) を使用しています．
SATySFi の文法の一部のみサポートしています．

## TODO

- [ ] `let-mutable`/`let-rec`/`let-inline`/`let-math`
- [ ] module, struct
- [x] ヘッダ
- [ ] `if` 式
- [ ] `match` 式
- [x] math mode
- [ ] 代数的データ型，コンストラクタ
- [ ] 演算子の優先順位
- [ ] `type` の宣言
- [ ] レコードのメンバアクセス
- [ ] パス括弧
- [ ] リスト・レコード型のコマンド引数
