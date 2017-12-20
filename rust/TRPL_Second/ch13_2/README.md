イテレータの話
==========================

* Rustではイテレータの作成は遅延実行
  * `xxx.iter()`で作成後、forなどで使用して初めて評価
* イテレータの実装は `Iterator` traitで実装
  * `next` メソッドで次の値を`Option`で包んで返す
* `iter()`はmut要求
  * 内部でシーケンスの場所を保持しているので
* VectorからIteratorを作成する場合の種類
  * `iter()`
    * Vectorの要素に対するImmutableな参照でIteratorを作成
  * `iter_mut()`
    * Vectorの要素に対するMutableな参照でIteratorを作成
  * `into_iter()`
    * Vectorの要素の実体でIteratorを作成。いわゆるmove
* `.map(|x| x).collect()`でIteratorをベつの形に変換
  * この時の戻り値は型推論に全部任せられない
    * `let v2: Vec<_> =` とする。`Vec<_>`はVectorの中身は型推論して、となる
* `filter()`で内容をフィルタリング
  * このとき作成するVecの型に注意。
  * `Vec<Show>`か`Vec<&Show>`かで使用するメソッドが変わってくる
    * 前者ならmoveする`into_iter`で、後者なら`iter`
* `Counter::new().zip(Counter::new().skip(1))`について
  * zipはIteratorを受け取り、中身をnextで取りながら取り出し元とペアのtupleに変換
  * 数が合わないときは少ないほうに合わせる
  * `(0..).zip(iter)`と`xxxx.iter().enumerate()`は同じ