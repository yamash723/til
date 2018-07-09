Packages, variables, and functions.
================

https://go-tour-jp.appspot.com/basics/1

コマンド
----------------

```bash
# ビルド
go build main.go

# ビルドして実行
go run main.go

# フォーマット
go fmt main.go
```

基本
---------------

* 行末セミコロンは不要
* `return`は必須
* packageで構成。`main`パッケージからプログラム開始
* パッケージのインポートは`import`で宣言。複数は`import ( .. )`で可能
* 関数名などの最初の文字が大文字の物は外部パッケージから参照可能
  * `math.pi` -> 参照不可
  * `math.Pi` -> 参照可

関数
-----------------

関数の連続した2つ以上の引数が同じ型の場合は省略可能

```golang
func add(x, y int) int {
    x + y
}
```

複数の戻り値を戻すことが可能

```golang
func swap(x, y string) (string, string) {
    return x, y
}

func main() {
    z, b := swap("Hello", "World")
}
```

戻り値に名前を付けておくことで`return`時に戻り値を省略することが可能  
※可読性落ちるのであまり利点はない・・・？

```golang
func split(sum int) (x, y int) {
    x = sum * 4 /9
    y = sum - x
    return
}
```

変数
--------------------

`var`で変数宣言。型は最後に型を記載  
引数同様省略することが可能

```golang
var i int
var a, b, c string
```

宣言時に初期化した場合は型推論により型宣言を省略可能

```golang
var i = 1
var a, b, c = "A", 1, true
```

代入文`=:`を使用することで`var`を使用しない暗黙的な型宣言を行うことが可能  
ただし関数外では使用不可

```golang
// error
a := 1

func some() {
    b := 1
}
```

宣言しに初期化しなかった場合はゼロ値になる

```golang
var i int       // 0
var f float64   // 0
var b bool      // false
var s string    // ""
```


基本型
----------------

基本型は下記の通り

* bool
* string
* int(int8/16/32/64)
* uint(uint8/16/32/64)
* byte(uint8のalias)
* rune(int32のalias)
* float32/64
* complex64/128

型変換は`T(v)`によって行う。Goは明示的な型変換が必要

```golang
i := 42
f := float64(i)
u := uint(f)
```

定数
------------------

`const`キーワードで宣言  
文字、文字列、真偽値、数値でのみ使用可能。また`:=`の代入式は使用不可

```golang
const Pi = 3.14
```