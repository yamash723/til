パターンマッチ
==============

パターンマッチの種別
---------------------

* `match` Arms
* Conditional `if let` Expressions
* `while let` Conditional Loop
* `for` Loops
* `let` Statements
* Function Parameters

その他
-------

* 変数名の先頭がアンダーバーの場合は`unused warning`が発生しない
  * 使用しなくてもmoveは発生している
* `Some(ref name) => {}`といった参照でのマッチングが可能
  * `Some(ref mut name) => {}`でミュータブルも可能
* `Message::Hello { id: id_variable @ 3...7 }`と@を使用して条件指定しつつバインドすることができる