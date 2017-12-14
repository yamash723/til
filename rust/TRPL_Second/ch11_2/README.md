Running Tests
======================================

テストコードの実行周りの話

* テスト実行時、`println`などのコンソール出力は動かない
  * `cargo test -- --nocapture` で実行することで出力可能
  * `cargo test --`の`--`は、以降の内容をコマンドのオプションではなく「引数とする」ためのもの
* テストの実行管理
  * `cargo test {テストメソッド名}` で個別の実行が可能
    * なおメソッドは部分一致となる。`two`を指定すれば`add_two`も対象となる
  * `#[ignore]`アノテーションでテストの除外が可能
    * `test tests_2::expensive_test ... ignored`
  * 除外したものを実行する場合`cargo test -- --ignored`で実行可能