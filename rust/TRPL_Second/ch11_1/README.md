Writing Tests
======================================

テストコードの話

* `#[test]`のアノテーションをつけた関数がテスト時に実行される
* `use super::*;`は1階層上の関数をインポート指定
  * `super` = `..` のようなもの
* 検証マクロ
  * `assert_eq!` 値の一致
  * `assert_ne!` 値の不一致
  * `assert!` 真偽値
* 検証マクロの `assert_xx` の場合、対象の型に`PartialEq`および`Debug`のtraitが実装されていなければならない
* 検証マクロの引数にエラー文言フォーマットと値を入れることが可能
  * `assert!(false, "Is False: value was {}", "sample")`
* `#[should_panic]`のアノテーションを付与した場合、その関数内でpanicが発生することをテストできる
  * ただし目的のものと違うpanicが発生することもあるため、panicメッセージの一致も条件づけられる
    * `#[should_panic(expected = "Guess val.....")]`