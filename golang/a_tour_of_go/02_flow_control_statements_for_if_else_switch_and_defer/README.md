Flow control statements: for, if, else, switch and defer
================

https://go-tour-jp.appspot.com/flowcontrol/1

For
----------------

* セミコロン
* 中括弧は必要
* 初期化と後処理は任意
  * `for ; sum < 10; { }`
* Goに`while`はなし
  * 設定値なしの`for`で代用

```golang
for i := 0; i < 10; i++ {

}

// same while
for {

}
```

If
----------------

* 括弧は不要。中括弧は必要
* 条件値の前に変数宣言可能
  * スコープはif文内。else内も使用可能

```golang
if x < 0 {

}

if v := math.Pow(x, n); v < lim {

}
```

Switch
------------------

* 各`case`で`break`は必要ない
* `if`同様変数宣言可能
* 条件のない`switch`が可能
  * 長くなりがちな`if else`をシンプルに書ける
  * `case`に条件を記載することが可能

```golang
switch os := runtime.GOOS; os {
case "darwin":
  fmt.Println("OS X.")
}

switch {
case x >= 10:
  fmt.Println("high")
case x < 10:
  fmt.Println("low")
}
```

Defer
------------

* `defer`に渡された関数の実行を呼び出しもとの終了まで遅延させる
* コネクションのCloseなどで使用
* 複数の`defer`を行った場合、LIFOの順で処理される

```golang
func main() {
  defer fmt.Println("world")

  fmt.Println("hello")
}
```