Cargoについての話
=======================

Cargoのリリース設定
-------------------

* `Cargo.toml`上では2種類のリリース設定がある。`dev`と`release`
* それぞれを個別にカスタマイズする時は下記の通り

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

* `opt-level`
  * 最適化レベル。デフォルトはそれぞれ`dev`が0、`release`が3
  * 0-3の範囲で、数字が大きいほど効果が高い
  * 効果が高い分遅いよね

Crateの公開
-----------------------

### ドキュメントコメント

* `///` を使用することでドキュメントコメントとなる
* ドキュメントコメント内ではMarkdownがサポートされている
  * ドキュメントコメント内のコードブロックはDoc testで実行される
* `cargo doc`でドキュメントを作成。`cargo doc --open`で作成後にドキュメントをブラウザで開く
* 使用例はExampleだが、ほかにもよく使われるセクションは下記の通り(ただし色々あるので公式Doc見てみるとよい)
  * Panic
    * 該当機能がPanicを起こす条件などを記載
  * Errors
    * `Result`型で返す場合のエラー種別と条件
  * Safety
    * 対象機能が`unsafe`な場合、なぜ安全でないかを説明

```rust
/// Adds one to the number given.
///
/// # Example
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, my_crate::add_one(5));
/// ```
fn add_one(x: i32) -> i32 {
  x + 1
}
```

* `//!`を使用することでCrate全体に対してのドキュメントコメントを記述することができる
* 慣習的には`src/lib.rs`の最上部に記載

### APIのエクスポート

* Crateの制作時に色々とモジュールを作っていくとpublicなAPIがどんどん深い階層に出来上がることがある
* そのままだと使用する側も難しい&ドキュメント上でも探すのが大変
* そういった場合は`lib.rs`上で`pub use`するとよい。ドキュメントトップにもでてくるし、使用時も浅い階層で使用可能

サブプロジェクト
---------------------------

* プロジェクト内にサブプロジェクトを作成する場合
  * プロジェクトルート上などで`cargo new xxxx`を実行
  * ルート側の`Cargo.toml`に下記を追加
    * dependenciesに`add-one = { path = "add-one" }`
    * `[workspace]`を追加
  * こうすることで親側で`extern crate add_one`が可能
* 当然だが上記の方法で作成したcrateの`Cargo.toml`に追加したCrateは親ではextern不可
* テストを実行する際、`cargo test`ではルートプロジェクトのテストしか実行されない
  * `cargo test --all`でサブプロジェクト含めてすべてテスト実施
  * buildも同じ
  * `cargo test -p xxxx`で特定のサブプロジェクトのみ実施可能