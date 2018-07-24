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

```golang
func (v *Vertex) Abs() float64 {
    ......
}
```