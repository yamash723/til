React Tutorial
===================

公式のチュートリアルを実施

公式サイト
--------------

[https://reactjs.org/tutorial/tutorial.html]

メモ
------------

* JSX構文はビルド時に`React.createElement`に変換される
  * Before: `<div class='shopping-list' />`
  * After: `React.createElement('div', { className: 'shopping-list' })`
  * `createElement()`を使用すればもっと細かいAPIを触れるがtutorialでは触らない
* `this.state`に状態を持つ。変更する場合は`this.setState({})`を呼ぶこと
  * `setState`は引数の内容と`this.state`の内容をマージしてくれる