More types: structs, slices, and maps.
================

https://go-tour-jp.appspot.com/moretypes/1

Pointers
------------------------

* 型の前に`*`でポインタ。ゼロ値は`null`
* 変数の前に`&`でポインタ引き出し
* 変数の前に`*`でポインタ先の値取得

```golang
i := 42
p = &i

fmt.Println(*p) // p経由で値取得
*p = 21         // p経由でiに代入
```

Structs
-------------------------

* 構造体
* `type xxxx struct`で定義
* 初期化時は中括弧内でフィールドの値を設定
  * 名前付きでフィールド指定可能
  * 値設定しない場合はゼロ値
* `s := struct { X int }{}`のように無名構造体を作成することが可能

```golang
type Vertex struct {
    X int
    Y int
}

Vertex{0, 1}        // X: 0, Y: 1
Vertex{X: 0, Y: 1}  // X: 0, Y: 1
Vertex{}            // X: 0, Y: 0
```

Arrays
-----------------------------

* 配列の長さは型宣言時に`[型の長さ]int`で宣言
* 宣言時の初期化は中括弧で指定

```golang
var values [10]int
var values [3]int{0, 1, 2}
```

Slices
------------------------------

* 配列内の要素を範囲指定した参照
* `a[low:hight]`の形式で指定
* `hight`の範囲は含まれない。`low <= value < hight`
* あくまで参照なので、内容の変更をした場合大本の配列が変更される
* `[]int{0, 0}`のような宣言でスライスの作成が可能
  * 実際には`[2]int`が作成され、参照スライスとなる
* `low``hight`はそれぞれ0と配列の長さが初期値となっており省略可能
* ゼロ値は`null`
* 組み込みの`make`を使用して動的に作成することも可能
  * `make([]int, 5)`で要素数5のスライスを作成
  * `make([]int, 0, 5)`で要素数0の容量5のスライスを作成
* スライスへ要素を追加する場合は`append`を使用。容量が足らない場合はより大きいサイズの配列が割り当てられる
  * 配列の再作成なので`for`などで連続して追加する場合は事前に拡張がよいか

```golang
a := [4]int{0, 1, 2, 3, 4}
s := a[1:4] // 0, 1, 2, 3

// 下記はすべて等価
a[0:4]
a[:4]
a[0:]
a[:]

append(s, 5)
```

Range
---------------

* スライスやマップを1つずつ処理するために使用
  * 値の範囲のRangeではない
* `range xxxx`で指定
* 1要素ごとにインデックスと値の2要素を返す
* 値のみを使用する場合など、不要なものは`_`で捨てることができる
  * 逆にインデックスのみ必要な場合は`, _`を省略可能

```golang
a := []int{1, 2}

for i, v := range pow { }
for _, v := range pow { }
for i := range pow { }
```

Maps
-----------------

* キーと値を関連付ける辞書型
* 型の表現は`map[キーの型]値の型`
* ゼロ値は`nil`
* リテラル以外にも`make`関数で初期化したものを作成可能
  * 単純な型名だけをリテラルで渡しているのならば型を省略可能
* 要素の削除は`delete`
* 要素の存在チェックは2つめの戻り値で判定
  * `elem, ok := m[key]`
  * 存在する場合は`ok == true`

```golang
var m map[string]int
m = make(map[string]int)

mp := map[string]SimpleStruct{
  "A": {1, 2}
}

delete(mp, "A")
```

Function
--------------

* 関数も第一級オブジェクトのように引数で渡すことが可能
* 関数はクロージャ。クロージャ内で使用されている変数へバインドされる

```golang
func add(x, y int, fn func(int, int) int) int {
  return fn(x, y)
}

func adder() func(int) int {
  sum := 0
  return func(x int) int {
    sum += x
    return sum // sumにバインドされる
  }
}
```