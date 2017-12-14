Running Organization
======================================

テスト構成周りの話

* PJルートに`tests`ディレクトリを置くことでもテストコードと認識される
  * さらにその中から指定して実行も可能
    * `tests/integration_test.rs`の場合なら`cargo test --test integration_test`
* `//!`でモジュール全体の説明を、`///`で関数の説明をする
* DocTestはこれらのドキュメントコメントの中身の関数をテストする