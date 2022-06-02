# サイボウズのKubernetes研修資料

[Introduction to Kubernetes](https://cybozu.github.io/introduction-to-kubernetes/)

少し気になったので目を通してみる

## Introduction to Kubernetes

このあたりは公式Docで見た範疇

## More Introduction to Kubernetes

### ヘルスチェック

- Liveness probe
  - Pod の生存チェック用定義。一定回数失敗した場合、Pod は再起動される
- Readiness probe
  - Pod がリクエストに応答できるかどうかのチェック定義
  - 成功するまでの間、その Pod は起動が完了していないとみなされ、Service のリバプロ先に追加されない
  - Pod が起動が完了する前にリクエストが飛んでくる現象を防ぐことができる

ローリングアップデートなどで Pod が起動中にも関わらずアップデートがどんどん進む、といったことを防ぐための機能。

### Anti Affinity

Pod をスケジュールする際の細かい制御を可能とする機能。
同じノードに Pod が集中するのを防いだりする、など。

### Pod Disruption Budget

サービス稼働に必要な Pod の最小値を指定できる機能。

ノードのシャットダウンなどの際、指定した値を下回らないように待ってくれたりする

## Exercise

エクササイズ内容は全部チェックしてみた

- apacheを殺すとPodが再起動されて復帰
- liveness probeを強制失敗にするとロールアウトが途中で止まる
  - 1つめの Pod で失敗しているので先に進まない
- その状態で undo するときれいに元通り。便利