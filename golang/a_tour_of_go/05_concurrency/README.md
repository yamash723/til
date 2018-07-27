Methods and interfaces
================

https://go-tour-jp.appspot.com/concurrency/1

Goroutines
------------------------

* Goの軽量スレッド
* `go f(x, y, z)`でgoroutineが実行
  * `f,x,y,z`は実行元で評価され、評価された値を用いた実行は新しいgoroutineで行われる
* 同じアドレス空間で実行されるので、共有メモリへのアクセスは必ず同期する必要がある

```golang
go say("world")
```

Channels
---------------

* チャンネルオペレータの`<-`で値を送受信する
* `ch <- v` でvをチャンネルchへ送信
* `v := <-ch` でchで受信した変数をvへ割当
* `ch := make(chan int)`で生成
* 基本的に片方の準備ができるまでは送受信がロックされる
  * `<-ch`で受信待ちしても送信されてこない場合は`deadlock`で落ちる

```golang
ch := make(chan int)
```

Buffered Channels
--------------------------

* バッファとして使えるチャンネル
* `ch := make(chan int, 100)`でバッファの長さを渡して初期化
* バッファが詰まると送信がブロックされ、空の場合は受信がブロックされる
* `cap(ch)`でバッファのキャパシティを取得できる

```golang
ch := make(chan int, 100)
```

Range and Close
------------------

* チャンネルを明示的にクローズ可能
* `v, ok := <-ch`で確認可能
  * 受信する値がない、かつcloseしているならokが`false`
* 基本closeする必要はないが、rangeループなどで使用して受け手が知る必要な場合に使用

```golang
v, ok := <-ch
```

Select
-------------------

* goroutineを複数の通信操作で待たせる
* `case`のいずれかが準備できるようになるまでブロックし、準備ができたら実行
* 複数の`case`の準備ができたらランダムに実行
  * どの`case`も準備できていなかったら`default`が実行される

```golang
select {
case c <- x:
case <-quit:
}
```

sync.Mutex
----------------

* 排他制御を行うライブラリ
  * `Lock`と`Unlock`で囲むことで排他制御で実行するコードを定義
* `Unlock`は`defer`で遅延実行させてもよい

```golang
mut.Lock()
mut.Unlock()
```