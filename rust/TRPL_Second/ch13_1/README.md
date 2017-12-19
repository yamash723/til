関数型言語的なRustの話
==========================

* 匿名関数の話
  * `let method = |param| { param }` で定義
  * 構造体のGenericsで使うときはwhereで境界指定ぽい
  * CacherのvalueをHashMap化
    * ダメアプローチ
      * getで取得 → パターンマッチでErrだったらvalue.insert.....
        * get取得時点で束縛されているのであとからinsertによる変更はNG
    * 良きアプローチ
      * `entry(hoge).or_insert(hage)`
      * リファレンスをよく読もう

## クロージャ3種の話

参考にkeenさんのブログ

[http://keens.github.io/blog/2016/10/10/rustnokuro_ja3tanewotsukutterikaisuru/](http://keens.github.io/blog/2016/10/10/rustnokuro_ja3tanewotsukutterikaisuru/)

* Rustのクロージャには3種類
  * Fn
    * `&self`
  * FnMut
    * `&mut selt`
      * 無名関数内で補足する値がmutの場合に使用
  * FnOnce
    * `self`
* クロージャは無名関数内で使用する関数外の変数を含めてクロージャ
* 使用する無名関数外の変数によってクロージャはFn/FnMut/FnOnesとなる