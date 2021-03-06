並列処理の話
===================

Thread
---------

* `thread::spawn`でクロージャーを渡して新規スレッドを作成
  * `thread::spawn(|| { xxxx })`
* スレッドなのでむろんメインスレッドが終了するとプロセス終了となる
  * 作成したスレッドを待機するのは`JoinHandle`になる
  * 作成時の戻り型が`JoinHandle`なので、その値の`join`メソッドをよぶことで作成したスレッドの終了を待機する
* `move`クロージャ
  * クロージャ作成時に`move`をつけることで、対象クロージャ内で参照する変数が束縛している値の所有権を移譲することができる
* スレッド間のメッセージングは`mpsc::channel()`でチャンネルを用意し、それぞれのトランスミッターとレシーバを使用する
  * `mpsc::channel()`の戻り値はタプル
    * 第1要素がトランスミッター
    * 第2要素がレシーバ
  * トランスミッターは`send`で送信、レシーバは`recv`で受信
    * 受信は2種類あり
      * `recv`はスレッドをブロックして受信まで待機
      * `try_recv`はノンブロッキング
    * 作成したトランスミッターは複製が可能
      * `mpsc::Sender::clone`で複製
      * 受信は同じレシーバになる

Mutex
----------

* メモリ共有の排他制御を行うもの
  * `Mutex::new(xxx)`で初期化
* もしもロック取得時にパニックが子ったら別スレッドはアクセス不可になるもよう
* 複数スレッドでメモリ共有を行う場合、`move`クロージャだとMutexの所有権が移ることに注意
  * 1つ目のスレッドにMutecの所有権が移る → 別スレッド生成時にはmove済なので渡せない
  * その場合は所有権を複数持たせることになるが、`Rc`は使用できない
    * `Rc`はスレッド間の共有不可である
    * `Arc`を使用すること
      * 機能は同じ
      * なぜ分かれている？ -> スレッド間の共有は高コスト。マルチスレッド時のみ使えばよい

SendとSync
-----------------

* 並列処理を行うためのトレイト
* `Send`トレイトを実装している場合、スレッド間で安全に受け渡しできる
* `Sync`トレイトを実装している場合、複数のスレッドから並列に処理されたとしてもメモリ安全である