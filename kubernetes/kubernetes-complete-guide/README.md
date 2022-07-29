# Kubernetes完全ガイド 第2版

K8sの基礎を学ぶために全体をざっと読む。

## 目次

- 第1章 Dockerの復習と「Hello, Kubernetes」
- 第2章 なぜKubernetesが必要なのか？
- 第3章 Kubernetes環境の選択肢
- 第4章 APIリソースとkubectl
- 第5章 Workloads APIsカテゴリ
- 第6章 Service APIsカテゴリ
- 第7章 Config＆Storage APIsカテゴリ
- 第8章 Cluster APIsカテゴリとMetadata APIsカテゴリ
- 第9章 リソース管理とオートスケーリング
- 第10章 ヘルスチェックとコンテナのライフサイクル
- 第11章 メンテナンスとノードの停止
- 第12章 高度で柔軟なスケジューリング
- 第13章 セキュリティ
- 第14章 マニフェストの汎用化を行うオープンソースソフトウェア
- 第15章 モニタリング
- 第16章 コンテナログの集約
- 第17章 Kubernetes環境でのCI/CD
- 第18章 マイクロサービスアーキテクチャとサービスメッシュ
- 第19章 Kubernetesのアーキテクチャを知る
- 第20章 Kubernetesとこれから

## サンプルデータ

https://github.com/MasayaAoyama/kubernetes-perfect-guide

## 環境

[kubernetes-sigs/kind: Kubernetes IN Docker - local clusters for testing Kubernetes](https://github.com/kubernetes-sigs/kind)

```bash
kind create cluster --image kindest/node:v1.24.1 --name kindcluster
kind create cluster --image kindest/node:v1.24.1
```

## 読書メモ

### namespace

- kube-system
  - クラスタのコンポーネントやアドオン
- kube-public
  - 全ユーザーが利用できるConfigMapなど
- kube-node-lease
  - Nodeのハートビート情報
- default

- namespaceはマイクロサービスを開発するチームごとにわけるのがいい
- prd / stgのような環境はクラスタごと分けるのがいい

 ### kubectl

```bash
# マニフェストファイルの適用
$ kubectl apply -f xxx.yaml
$ kubectl apply -f . # ディレクトリ内のファイルをファイル名順に適用
$ kubectl apply -f . -R # 再帰的に反映
$ kubectl apply -f . --prune # マニフェストの定義に存在しないものは削除

$ kubectl diff -f xxx.yaml

# 定義ファイルを直接編集
$ kubectl edit pod sample-pod

# リソース情報の更新
# env / image / resources / selector / serviceaccount / abject
$ kubectl set image pod sample-pod nginx-container=nginx:1.16

# リソース使用量の確認
$ kubectl top nodes
$ kubectl top pod

# リソースの取得
$ kubectl get pods
$ kubectl get pods -L label1 # 指定ラベルの情報を取得
$ kubectl get pods --show-labels # ラベルを表示
$ kubectl get pods -o wide # 詳しい情報つき
$ kubectl get pods --watch
$ kubectl get pods --watch --output-watch-events

# Podに入っての操作
$ kubectl exec -it sample-pod -- bash

# リソース指定の削除 ※waitで削除完了を待機
$ kubectl delete pod sample-pod
$ kubectl delete pod sample-pod --wait

# Podの再起動 ※Deployment単位
$ kubectl rollout deployment sample-deployment

# すべてのPodが削除されるまで待つ
$ kubectl wait --for=delete pod all

# アノテーションの付与
$ kubectl annotate pods sample-annotations annotation3=val3
$ kubectl annotate pods sample-annotations annotation3=val3 --overwrite
$ kubectl annotate pods sample-annotations annotation3- # 削除

# ラベルの付与
$ kubectl label pods sample-label label3=val3
$ kubectl label pods sample-label label3=val3 --overwrite
$ kubectl label pods sample-label label3- # 削除

# rollout
$ kubectl rollout status deployment xxxxx
$ kubectl rollout history deployment xxxxx
$ kubectl rollout undo deployment xxxxx --to-revision 1 # 基本使わない
$ kubectl rollout pause deployment xxxxx # 更新の一時停止
$ kubectl rollout resume deployment xxxxx # 更新の再開

# 手動でDeploymentを作成する
$ kubectl create deployment sample-deployment-by-cli --image nginx:1.16

# エフェメラルコンテナを使ったデバッグ
$ kubectl debug sample-pod --image=amsy810/tools:v2.0 -it -- bash

# Pod / Deployment / Serviceへのポートフォワーディング
$ kubectl port-forward sample-pod 8888:80

# ログの確認
$ kubectl logs -f --timestamps sample-pod

# コンテナからファイルコピー
$ kubectl cp sample-pod:/etc/hostname ./hostname

# Podがうまく立ち上がらない場合、ENTRYPOINTを上書きして手動実行する方法もある
$ kubectl run --image=nginx:1.16 --restart=Never --rm -it sample-debug --command -- bash

# ReplicaSetのスケーリング
$ kubectl scale replicaset sample-rs --replicas 5
```

- `kubernetes create` は `--save-config` オプションがない場合、適用したマニフェスト情報を保持しない
- `--server_side` オプションを使うことでServer-side Applyにすることができる
  - 衝突の検知に有効
- metadata.generateNameを指定することでリソースにprefixをつけることができる
  - ただしapplyでは使用不可
- manifestファイルはは複数リソースを定義できる
  - `---` で区切る
  - ConfigMapなど共用されるものは別にしておくのが無難
- アノテーションとラベル
  - アノテーション
    - システムコンポーネントが利用するメタデータ
    - リソースに対するメモ書き
    - `kubectl.kubernetes.io/last-applied-configuration` にはapplyで適用したマニフェストの定義が保存されている
    - EKSなど環境特有の機能が実装されている場合、アノテーションを使うのがほとんど
  - ラベル
    - リソースの管理に使用するメタデータ
- apply実行時にpruneオプションを付与することで、マニフェストから削除されたリソースを削除できる
  - CIなどではこの方法を用いるといい
- ポートフォワーディング中は複数のPodが対象だったとしても1台のみに限定される
- OSSのSternを使うとログの確認が楽になる
- krewを使って各種プラグインを入れると便利
- Podが起動しないデバッグの際、`kubectl run` コマンドでENTRYPOINTを上書きして立ち上げるテクニックがある

### Workloads APIs

#### Pods

- 同一Pod内のコンテナはIPアドレスを共有しているのでlocalhostで通信可能
- サブコンテナはプロキシや設定値の動的な書き換え、ローカルキャッシュやSSL終端用など
- nginxとredisのようなメインコンテナを1つのPodに入れるのは推奨されない
- デザインパターン
  - サイドカー
    - GitのSyncやログの転送などの補助的なもの
  - アンバサダー
    - 外部のシステムと接続する際の代理
  - アダプタ
    - 外部からのリクエストに対して差分を吸収する
- `hostNetwork` オプションを使うことでホストのネットワークを使用してPodを起動できる
  - ホスト側のネットワークを監視したりするエッジケースで使うこと
  - この状態でクラスタ内のDNSを参照させたい場合は `dnsPolicy` を　`ClusterFirstWithHostNet` にする必要がある
- DNSサーバーの設定を手動で設定する場合、 `spec.dnsPolicy: None` にして `dnsConfig` を設定する
  - その他Nodeの設定を引き継ぐ、などもある
  - デフォルトはクラスタ内のDNSに問い合わせる
- `spec.hostAliases` でPod内のすべてのhostsを書き換えることができる
- `spec.workingDir` でWorkingディレクトリを上書きすることができる

#### ReplicaSet / ReplicationController

- ReplicationControllerという名称からReplicaSetという名称に変わっている
- Podのレプリカを作成し、指定された数のPodを維持するもの

#### Deployment

- 複数のReplicaSetを管理してローリングアップデートやロールバックを実現するリソース
  - Deployment -> ReplicaSet -> Pod の関係
- apply時に `--record` オプションを指定する事でコマンドの実行履歴を保持することができる
  - rollback時の参考情報に使えるが、業務上利用することは殆どない
  - `Flag --record has been deprecated, --record will be removed in the future`
- 変更があった場合ReplicaSetが作成されるが、この変更はPodの内容の変更が必要
  - `spec.template` に変更があると作成される
- `rollout` することができるが、それよりは古い定義のマニフェストをapplyしたほうがいい
- Deploymentのアップデート戦略は2種類
  - `Recreate`
    - 一度全Podを削除して作り直す。早いしリソースを節約できるがダウンタイムが発生する
  - `RollingUpdate`
    - いわゆるローリングアップデート
    - 許容される不足Pod数や超過Pod数を指定することもできる
      - Pod数やパーセンテージでの指定も可能
