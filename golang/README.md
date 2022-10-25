# Golang

## 2022/10/25

GoでWakatimeのクライアントを書いていた時のメモ

- Githubのクライアントを参考に作成
  - https://github.com/google/go-github
  - 全体的に作りが良さそうなので参考に
- 認証は提供してないので、認証通したHttpClientを渡せ、というスタイル
- Wakatimeも同じで行こうとしたがAPI Keyを使った認証の場合はそのままだと難しいのでデフォルトヘッダーを用意してあげることにした
  - Client構造体にDefaultHeaderを用意しておいてあげて、使うほうが任意でそこに入れる形
- ちなみにHttp通信のMockは `httpmock` を使用。なかなか使いやすい
- nullable（golangの場合はnilable?）がポインタ型を使うのに違和感
  - 使っていけばなれるかな？
- 言語仕様がミニマムなので手を出しやすい印象はある。書いてて記法に困ることはあんまりない
