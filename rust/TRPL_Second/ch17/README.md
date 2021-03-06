Rustでのオブジェクト指向
=========================

* Rustでは継承とは違うアプローチをとっている
  * 継承は必要以上のコードを共有する危険性を持っている
  * 違うアプローチ = トレイトオブジェクト

トレイト境界
---------------

静的ディスパッチ。パフォーマンスはこちらが優れる

対象の構造体や関数に対し、ジェネリクス型にトレイトの実装を保証させることができる

```rust
// 書き方は2種類
fn sample1<T: Draw>(param: T) { }
fn sample2<T>(param: T) where T: Draw  { }
```

この場合、`Draw`トレイトを実装した型でなければ引数としてNGとなる

トレイトオブジェクト
---------------------

動的ディスパッチ。パフォーマンスとしては劣るので注意

```rust
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<Draw>>,
}
```

上記の場合、構造体`Screen`のcomponentsへは「`Box`で包んであげていれば、`Draw`トレイトを実装した型であればなんでもよい」
となる

この`Box`はポインタ参照であればよいので、`Rc`などでもよい

オブジェクト指向のデザインパターン
-------------------------------------

* Stateパターンなど、オブジェクト指向のデザインパターンをRustで使用することはできる
* ただしRustでそれが最善かどうかはよく考えて使用すること