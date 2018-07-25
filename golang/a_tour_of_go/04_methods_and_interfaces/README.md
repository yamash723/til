Methods and interfaces
================

https://go-tour-jp.appspot.com/methods/1

Methods
------------------------

* Goにはクラスはないが型にメソッドを定義可能
* `func`とメソッド名の間に自身の引数リストをレシーバとして設定
* `struct`以外の任意の型にメソッド宣言が可能
  * ただし、同じパッケージになければならない点に注意
  * 組み込みの型などには設定不可。`type`でalias設定を行う必要あり

```golang
func (v Vertex) Abs() float64 {
    ......
}

// Error
func (x int) Xxx() { }

// Success
type MyInt int
func (x MyInt) Xxx() { }
```

Pointer receivers
-------------------------

* メソッド宣言時のレシーバにポインタを使用可能
  * ポインタではなく変数宣言の場合はコピーを操作することになる
* 一般的には変数レシーバとポインタレシーバを混在させることは避ける

```golang
func (v *Vertex) Abs() float64 {
    ......
}
```

Interface
------------------

* `type xxxx interface`で定義
* interfaceの実装は明示的に宣言する必要はない
* インターフェースの値は、値と型のタプルで表せる
  * `(インターフェースを実装した構造体等の値, インターフェースを実装した型)`
* 実装先の値がnilの場合、メソッドのレシーバではnilが呼ばれる
  * Goではこれをぬるぽしないで処理するのが一般的
  * インターフェース型の変数に対して、実装済の型の変数(nil値))を代入してメソッド呼び出し -> レシーバにnil
  * nilのインターフェース型に対してメソッド呼び出し -> ランタイムエラー
* メソッドを持たないインターフェースは空のインターフェースと呼ばれる
  * 空のインターフェースは任意の型の値を保持できる
  * any型のような？

```golang
type Test interface {
  M() int
}

type MyInt int

// これでTestインターフェースの実装となる
func (i *Int) M() int {
  return i
}

interface {}
```