An I/O Project: Building a Command Line Program
==============================================

rustで`grep`を作ってみるチャプター

* 最初に一気に作ってからリファクタリング
* ドメイン考えてstrust作成
  * ありがちなのが引数の取得
    * mainで引数の数チェックなどを行わず、`Config` structのほうで処理をする。役割をまとめる
    * 値チェック時も安易にpanicさせず、`Result`を使用してコントロールする
* `Result`を展開するときにはいろいろな開け方があるが、`unwrap_or_else`を使用すると`Err`の場合にブロックを使用可能
* `Result<(), Box<Error>>`について
  * `Ok(())`で値を返しているがまだいまいち理解してない。Ok内のカッコはなんぞや・・・？
  * Boxに関しては対象をヒープ領域に入れるもの、らしい。まだ理解できていない
    * 今後のチャプターに詳細があるっぽい
* `lib.rs`のものを`main.rs`で使用する場合は`extern crate minigrep;`で指定
  * `extern`: crateのインポート
  * `use`: crateのmoduleインポート
* ライフタイムの設定をする場合があることを理解する
  * 引数で渡された文字列の一部を返す場合
    * 実体で返すのはコピーする分無駄。ポインタで返したい
    * そうなると引数の内容をスライスで返すのが良い
    * その場合どこまで生存させるかをコンパイラが判断つかないため、明示的にスライス元の引数にライフタイムの設定をする
      * 引数からのスライスのポインタなのか、関数内で新たに作成したメモリのポインタなのかは関数定義上不明
* 標準出力は`println`マクロ、標準エラーは`eprintln`を使用
* ifも式なので処理結果を代入可能
* `env::var("CASE_INSENSITIVE").is_err();` で環境変数を取得可能
  * `is_err()`は`Result`の結果がErrの場合はtrueとなる