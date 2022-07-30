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

# CronJobの一時停止
$ kubectl patch  cronjob sample-cronjob -p '{"spec":{"suspend":true}}'

# CronJobを元に任意のタイミングでJobを実行
$ kubectl create job sample-job-from-cronjob --from cronjob/sample-cronjob
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

#### DaemonSet

- 各NodeにPodを1つづつ配置するリソース。ReplicaSetの特殊な形
  - レプリカ数の指定などはできないが、「特定Nodeには配置しない」ということはできる
- FluentdやDatadogなど、全Nodeで動作させたいプロセスのために使う
- アップデートはローリングアップデートの他に `OnDelete` がある
  - 即時更新はせず、Podが死んだときに、更新するもの

#### StatefulSet

- ステートフルなワークロード用。ReplicaSetの特殊な形
  - Pod名のサフィックスが数字のインデックスが付与されたものになり、Pod名が変わらない
  - PersistentVolumeを使ってデータの永続化が可能
- `volumeClaimTemplates` を指定してPodにVolumeをアタッチ
- 複数のPodが起動する場合、基本順番に作られていく。インデックス番号0から順番に
  - ただしこれを並列にすることも可能
- ローリングアップデートもあるが、少し特殊なので実際やるときは要調査

#### Job

- N並列で実行しながら、指定された回数のコンテナの実行を保証するリソース
  - ReplicaSetとの違いは「起動するPodが停止することを前提にして作られているかどうか」
- labelとselectorはk8sの方でuuidを生成するため、明示的に付与することは非推奨
- `restartPolicy` を使用して失敗時の動作を変更できる
  - `Never`
    - Podが死んだ場合、別のPodを立ち上げる
  - `OnFailure`
    - Podを再利用して実行する。NodeやIPに変更はないが、永続化していないデータは消失する
- `backoffLimit` で失敗数の許容ラインを設定できる
- 並列実行することも可能だが、成功数5のときに並列7にしても5つのPodしか上がらない
- 基本、成功回数はあとから変更不可（applyが必要）

#### CronJob

- CronJobとJobは、DeploymentとReplicaSetのような関係と同じ
- CronJobを元に任意のタイミングでJobを実行することも可能
- `spec.concurrencyPolicy` を指定することで同時実行の制御ができる
  - `Alloq` デフォルト
    - 制限を行わない
  - `Forbid`
    - 同時実行をしない
  - `Replace`
    - 前のJobをキャンセルしてJobを実行する
- Kubernetes Masterがダウンしている場合など、Jobの実行が遅延したときの許容秒数を指定できる
  - デフォルトはどんなに遅れたとしてもJobを作成するようになっている
- 作成したJobを保存する数も指定できる
  - ログ調査などに役立つが、本番ではログ集約基盤にまとめることがおすすめ

### Service APIs

- クラスタ上のコンテナに対するエンドポイントの提供や、ラベルに一致するコンテナのディスカバリに理想されるリソースたち
- k8sはクラス作成時に内部ネットワークを構築する
  - 基本Nodeごとにセグメントを切り、Node間通信はVXLANやL2Routingなどで転送し相互通信できるようにしている

#### Service

- 受信したトラフィックを複数のPodにロードバランシングする機能を提供
- 種類が複数
  - `ClusterIP`
    - クラスタ内でのみ利用可能なエンドポイントを提供する
    - 自動付与ではなく静的に指定することも可能
  - `ExternalIP`
    - NodeのIPで受けたトラフィックをコンテナに転送する
    - 特別な理由がなければ `NodePort` の利用を検討
    - type自体は `ClusterIP` になるが、 `externalIPs` でNodeのIPを指定する
  - `NodePort`
    - すべてのNodeのIP:ポート番号で受けたトラフィックをコンテナに転送する
    - 利用できるポート範囲は決まっているので注意
  - `LoadBalancer`
    - クラスタ外のロードバランサに外部疎通性のある仮想IPを払い出せる
  - `Headless`
    - バランシングなどは行わず、特定PodのIPを返すもの
    - StatefulSetで稼働しているPodをServiceとする場合など
    - ロードバランシングを行わず、DNSラウンドロビンしたいときに使う
  - `ExternalName`
    - Service名の名前解決に対して外部ドメインにあてのCNAMEを返す
    - 別名の指定や、クラスタ内から外部に接続する際のエンドポイントを切り替えやすくするためのもの
- Pod側でポートに名前をつけておくことで、Service側で指定する際に名称でポート指定をすることができる
- サービスディスカバリ
- 環境変数、DNSのAレコード、DNSのSRVレコード
    - Aレコードで指定する場合、FQDNではない省略形で参照可能だが、Namespaceが違う場合は最小の短縮形形で参照できないので注意
    - SRVレコードはポートも含めて解決できる
- 大規模クラスタの場合NodeごとにDNSキャッシュサーバを配置する仕組みがある
- セッションアフィニティの機能がある
  - 送信元IPで転送先を指定可能。この機能はNode上のiptablesで実現されている
    - 転送設定テーブルに残す秒数も指定可能
  - LoadBalancerタイプの場合、LBも同様のことを行っており、かつIPもクライアントのものにしないといけないなどの制約がある
- 二段階ロードバランシング
  - デフォルトではLBからNodeに到達した場合、そこからPodに対しても再バランシングを行っている
  - 均等になる反面、オーバーヘッドや送信元IPが消失することになるので、Node到達後は再バランシングしない設定にもできる
    - `spec.externalTrafficPolicy`
- トポロジを意識したバランシング
  - リージョンなど含めてバランシングの優先順位を決められる
  - `spec.topologyKeys` で設定
  - 同一ホスト、AZ、リージョンで優先順位の設定が可能

#### Ingress

- いわゆるL7ロードバランサ
  - URLやヘッダーでの負荷分散
- NetworkPolicyリソースに出てくるものとは別
- 2種類に大別できる
  - クラスタ外のロードバランサーを利用したIngress
    - GKE Ingressなど
  - クラスタ内にIngress用のPodをデプロイするIngress
    - Nginx Ingressなど
    - クラスタ内にロードバランシング
- これらの操作を行うIngressControllerをデプロイする必要がある
- Ingress Controllerはクラスタ内のすべてのIngressリソースを見てしまうので、衝突の可能性がある
  - IngressClassを設定することで分離ができる
    - 起動時にオプションを渡す、もしくはIngressリソースにアノテーションを付与する
    - このへんは今後のアップデートで改善予定
