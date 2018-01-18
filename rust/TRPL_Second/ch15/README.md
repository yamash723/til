スマートポインタの話
===================

> スマートポインタ

※C++とかの話。Rustのようにスコープ外れたりしたら自動で開放？違いは？要勉強

Rustにおけるスマートポインタは三種類

* `Box<T>`
  * ヒープに値を割り当てる
* `Rc<T>`
  * 複数の所有権を許可する
* `Ref<T>` / `RefMut<T>`
  * `RefCell<T>`を使ってアクセス。詳細は後で

`Box<T>`
----------------------

* `let b = Box::new(5)` で任意の値を使用することができる
  * この時newに渡された値はヒープに保存される
  * そのヒープにある値のポインタを値として持ったBoxがスタックに格納される
  * スコープが外れた時はスタックのBox、ヒープの値両方が解放される
  * とはいえ`i32`はそもそもプリミティブでスタック行くので意味はない
* Rustはコンパイル時に型のサイズを知っていなければならない
  * ただ再帰型のような無限に続く可能性のあるものはサイズがわからない
  * そういうものは`Box<T>`で包んであげるとよい(Box自体はサイズがわかっている)

### 再帰の例

```rust
enum List {
    Cons(i32, List),
    Nil,
}
```

Consタプル内に同じListが定義されている。`Cons(i32, Box<List>),`と変えればよい

### `Deref`

* `Deref`はトレイト
* 実装することで参照外しの動作を実装することができる
  * `let y = *x;`のようにアスタリスクで参照外しをすることができる
  * このアスタリスクによる参照外しは`*(y.deref())`と同じ

下記のような書き方が可能

```rust
fn main {
    let name = MyBox::new(String::from("Rust"));
    hello(&name);
}

fn hello(name: &str) {
    println!("Hello {}", name);
}
```

`MyBox` `String`共に`deref`を実装しているので、型強制が発生する

なお、型変換をしない場合は次のような記法になる

```rust
fn main() {
    let name = MyBox::new(String::from("Rust"));
    // nameを参照外しし、そこからスライスを作成
    hello(&(*name)[..]);
}
```

#### 型強制

* Rustの数少ない自動変換
* `Deref<Target=T>`を実装している型`U`があるとき、`&U`は`&T`に自動的に変換される
* よって次のような変換が発生する
  * `&MyBox<String>`を`&String`に変換
  * `&String`を`&str`に変換

### `Drop`

* 値がスコープ外になった時の動作をカスタマイズするトレイト
  * 例にあげると`Box<T>`は自分が参照しているヒープの値をクリアしている
    * スタックにある`Box<T>`がスコープから外れる → `Drop`トレイトにヒープの値を削除する処理を入れている
* `Drop`トレイトで実装される`drop()`で値のクリアが行われている

```rust
fn main() {
    let c = CustomSmartPointer { data: String::from("my stuff") };

    // このような書き方をすると、明示的なdropをスコープを外れた時の自動dropで
    // 計2回dropが発生してしまうためコンパイルエラーになる
    c.drop();

    // こうして変数ごとdropすること
    drop(c);

    println!("CustomSmartPointers dropped before the end of main.");
}
```

`Rc<T>`
----------------------

* `Rc`は`Reference counting`の略称
* 所有権を共有するためのもの
* シングルスレッドでのみ使用可能
* `Rc::new()`で作成
  * `Rc::clone(&xxxx)`で参照を増やすことができる
  * `xxx.clone()`メソッドもあるが、こちらはディープコピー
  * `Rc::strong_count(&a))`で現在の参照数を確認可能

`RefCell<T>`
-------------------------

* imutableなものに対して値を変更するためのもの
* `Rc`と違い所有権は1つ
  * `Box`との違いは所有権/参照ルールのチェックタイミングの違い
    * `Box` -> コンパイル時にチェック。コンパイルエラー
    * `Rc` -> 実行時にチェック。panic発生
* `RefCell::new(vec![])`等で初期化
  * `xxx.borrow` で参照のみのものを取得
  * `xxx.borrow_mut` で所有権ありのものを取得
