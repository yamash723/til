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
# Namespaceの作成
$ kubectl create namespace sample-namespace

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

# Secretの作成や取得
$ kubectl create secret generic --save-config sample-db-auth --from-file=./username 
$ kubectl get secret sample-db-auth -o json | jq -r .data.username | base64 --decode

# Nodeをスケジュール対象から外す / 対象に戻す
$ kubectl cordon node-name
$ kubectl uncordon node-name

# NodeからPodを排出する
$ kubectl drain node-name --force --ignore-deamonsets

# Taintsを付与/削除する
$ kubectl taint node xxxxxx env=prd:NoSchedule
$ kubectl taint node xxxxxx env-

# ServiceAccountの作成
$ kubectl create serviceaccount sample-serviceaccount

# Helm
$ helm repo add xxxxxx
$ helm repo list
$ helm search repo xxxxx
$ helm test
$ helm install xxxx
$ helm template 
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

### Config & Storage APIs

####  環境変数

- `env` / `envFrom` でPodテンプレートにわたす事ができる
- 情報源は5種類
  - 静的設定
  - Podの情報
    - `valueFrom.fieldRef.fieldPath` などで指定
  - コンテナの情報
    - `valueFrom.resourceFieldRef.containerName` などで指定
  - Secretリソースの機密情報
  - ConfigMapの設定値
- `args` などで環境変数を使用する場合、 `$()` で展開する

####  Secret

- Opaque
  - 場合1MBまで値を保存可能
  - Base64で保存されてるため取得時はデコードが必要
- TLS
  - Ingressから参照可能
- Dockerレジストリ
  - コンテナレジストリの認証に使用可能。手動で作るのが楽
  - イメージを取得する場合の認証情報の指定は、Podのspecで `imagePullSecrets` を指定する
- Basic認証
- SSH認証
  - 環境変数として渡すか、Volumeとしてマウントの2種類を選択できる
  - Volumeとしてマウントした場合は動的な更新が可能になる

#### ConfigMap

- Key-Valueで設定情報などを保存する。nginx.confなどの設定ファイル自体を保存することも可能
  - バイナリもOK
- 格納可能サイズは1MB
- 環境変数として渡すか、Volumeとしてマウントの2種類を選択できる
  - `configMapKerRef` で環境変数として参照
- スクリプトを保存しておき、ConfigMapをマウントして実行する、という技も使える

#### PersistentVolumeClaim

- PersistentVolumeClaimとはなにか
  - PersistentVolume
    - 外部のVolumeを提供するシステムと連携して、新規Volume作成や削除を行うもの
  - PersistentVolumeClaim
    - 作成したPersistentVolumeをPodから使用するためのアサインを行うもの

#### Volume

- emptyDir
  - Podの一時的なディスク領域。Podが死ぬと消える
  - `sizeLimit` で制限をかけることも可能
  - `medium`  に `Memory` を指定することで高速な `tmpfs` 領域を使用可能
- hostPath
  - Node上の領域にコンテナをマッピングするもの
- downwardAPI
  - Podの情報などをファイルとして配置するプラグイン
  - ConfigMapなどのMountと同じ
- projected
  - secretやConfigMapなどのボリュームマウントを1箇所に集約するもの

#### PersistentVolume

- 厳密にはClusterリソースに含まれる
- 基本的にネットワーク越しにディスクをアタッチするタイプ
- CSI（Container Storage Interface）経由でさまざまなプロバイダを利用可能
- PersistentVolumeClaimからの要求があった場合、要求サイズを満たす最小サイズのPersistentVolumeを割り当てる
- Reclaim Policy
  - 利用し終わったあとの処理方法
  - 種類
    - Delete
      - 実態を削除。外部ボリュームのDynamicProvisioning時に利用されることが多い
    - Retain
      - 消さずに保持。ただし他のPodなどにMountされることはない
    - Recycle
      - `rm -rf /` してから再利用する
      - 廃止が検討されているので非推奨
- DynamicProvisioning
  - PersistentVolumeClaimが発行されたタイミングで動的にPersistentVolumeを作成する
  - リサイズがサポートされている場合リサイズを行うことが可能
  - 事前に `allowVolumeExpantion: true` を指定しておくことが必要
- ファイルシステムではなくブロックデバイスとして作成も可能
- PersistentVolumeClaimからスナップショットをとることも可能
- StatefulSetの場合 `spec.volumeClaimTemplate` で別途定義することなくPersistentVolumeClaimを作成可能
- `subPath` を使用することで特定ディレクトリをルートにすることができる
  - 同一Podのコンテナ別でルートを設定する場合などに使用

### Cluster APIs / Metadata APIs

- Cluster APis
  - セキュリティ周りの設定やクォータ設定など、クラスタ制御に関わるリソース
  - Node / Namespace / PersistentVolume / ResourceQuota / etc....
- Metadata APIs
  - クラスタ上にコンテナを起動させるのに利用するリソース
  - LimitRange / HorizontalPodAutoscaler / PodDisruptionBudget

### リソース管理とオートスケーリング

- CPUのリソース制限
  - 周波数ではなく1vCPU = 1000msecとする
  - Podの `resources.requests` 配下はリソースの下限になる。空きNodeに指定した量のリソースがない場合はスケジューリングされない
  - Limitは上限
    - Limitだけを設定した場合、Requestも同様の数値が設定される
- Storage
  - kubectl logsコマンドで出力するためのログも内部で保管されており、これもEphemeral Storageに含まれる
  - StorageのLimitを超えてしまうとPodがEvictされる
- 各リソースは完全に枯渇するとクラスター自体やNode全体に影響を及ぼす。それを回避するためにシステム向けのリソースが確保されている
  - `kube-reserved` / `system-reserved`
  - Eviction Managerがシステム全体が高負荷にならないように管理している
    - Eviction Thresholdに `soft` / `hard` の2種類がある
    - `soft` の場合はSIGTERM、`hard` の場合はSIGKILLが送られる

### Cluster Autoscaler

- Pending状態のPodが出てきたタイミングで発動する
  - Requestが高い場合、実際の負荷がそうでもないのに新規でNodeが追加される
  - 大事なこと
    - RequestとLimitに顕著な差をつけないこと
    - Requestを大きくしすぎないこと
  - OOMが発生しない程度のリソース割当、が目安

### LimitRange

- Namespaceに対して各種制限のデフォルト値を設定するもの
- typeでPod、Container、PersistentVolumeとそれぞれに設定できる

### ResourceQuota

- Namespace単位で理想可能なリソースを制限できる
- configMapは10個まで、など

### HorizontalPodAutoscaler

- Deployment / ReplicaSet / ReplicationControllerのレプリカ数をCPU負荷などに応じてスケールさせるリソース
  - 30秒に1回チェックを行う
    - スケールインは最大5分に1回
    - スケールアウトは最大3分に1回
- 指定したリソース状況（CPU使用率50%など）に合わせる形でスケールする
  - CPU以外のリソースを使う場合、Prometheusなどのメトリクスサーバと連携する必要がある
- `spec.behavior` でオートスケールの頻度などをリソース単位で指定できるようになった

### VerticalPodAutoscaler

- コンテナに割り当てるCPU/メモリのリソースを自動的にスケールさせる
- コンテナの名称を指定し、オートスケールの有効化設定と割当可能なリソースの成約をかけることができる
  - 正規表現が使用できないことに注意
- 推奨値だけ計算する、といったことも可能
- なお、VerticalPodAutoscalerはK8sのコア機能でないことに注意

## ヘルスチェックとコンテナのライフサイクル

### ヘルスチェック

- 3種類のヘルスチェック
  - Liveness : Pod内のコンテナが正常に動作しているかの確認
    - 失敗したらコンテナ再起動
  - Readiness : Podがリクエストを受け付けることができるかの確認
    - 失敗したらトラフィックを流さない
  - Startup : Podの初回起動が完了したかの確認
    - 他のProbeを実行しない
- ヘルスチェックの方式も3種類
  - exec : コマンドの実行
  - httpGet : GETリクエストを投げる
    - headerなども細かく指定可能
  - tcpSocket : TCPセッションが確立できるかどうか
- gRPCの場合はヘルスチェック機能が用意されているので、grpc_health_probeを使うといい
- 初回ヘルスチェック開始までの遅延秒数も設定できるが、それよりはStartupProbeを使ったほうがいい
- Pod ready++(ReadinessGate)
  - 通常のPodのReady判断だけでは不十分な場合に使用
  - クラウドのLBとの連携に時間がかかるなど
  - これを無視して名前解決できるようにするオプションもあるが、特別なユースケースでない限りおすすめはしない

### コンテナのライフサイクルと再始動

- 停止時の挙動はrestartPolicyで指定
  - Always : Podが停止すると常に再起動。ただしJobの場合は指定できない
  - OnFailure : 予期せぬ停止（終了コード0以外）の場合再起動
  - Never : Podが停止しても再起動しない

### Init Containers

- Podのメインコンテナを起動する前に別コンテナを起動させる機能
  - セットアップ前のスクリプトなどをメインコンテナに含まないようにできる
  - セキュリティ上の都合やイメージの肥大化を防ぐため
- 複数指定可能、かつ実行は直列で順番に行われる

### postStart / preStop

- コンテナの起動時と停止直前に任意のコマンドを実行
  - `spec.containers.lifecycle` 内に記述
  - execかhttpGetのどちらかが利用可能
- postStart
  - Entrypointの実行とほぼ同時に実行されてしまうため、Entrypointが実行されるより先にpostStartが実行される可能性に注意
- preStop
  - 終了要求が来たあとで実行
- 注意点
  - 最低1回実行されるが、複数回実行される可能性があることに注意
  - postStart実行中はProbeが実行されないので、デッドロックするようなものは実行しないようにする

### Podの安全な停止とタイミング

- Podの削除要求が来た場合の動作
  - 「preStop + SIGTERM」「Serviceからの除外設定」が非同期で実行
    - 非同期のため、Serviceからの除外設定が行われる前にプロセスが停止してしまう場合もある
    - preStopで数秒sleep入れるといった手がある
    - またGracePeriodSecondsの秒内に処理を終わらせる必要ガある。これを超えるとSIGKILLが飛んでくる
- kubectlでPodを削除する場合、即座に削除したければ--forceをつけるといい

## メンテナンスとノードの停止

- Nodeをスケジュール対象から外す場合は `kubectl cordon` を使用する
- 外しただけでは新規Podのスケジュールがされないだけで、すでにスケジュールされているPodはそのまま
  - `kubectl drain` でPodを排出する
    - drain実行時にスケジュール対象から外れるようになるため、事前にcordonは実行しておく必要は無い
  - ローカルストレージは削除される

## PodDisruptionBudget(PDB)による安全な退避

- Nodeが排出処理を行う際、特定Deployment配下のレプリカが一気に停止するとダウンタイムが発生する可能性がある
- 回避策として、排出時に停止できるPodの数を指定することが可能
  - `minAvailable` 最小起動数
  - `maxUnavailable` 最大停止数
  - なおパーセンテージでも指定可能
    - AutoScalerなどでPod数が変動する場合はパーセンテージ設定がおすすめ

## 高度で柔軟なスケジューリング

- フィルタリングとスコアリング
  - フィルタリング
    - スケジュールするPodが割当可能なノードを計算
    - リソースの空きがあるか、指定されたラベルかどうかなど
  - スコアリング
    - フィルタリング後のノードを順位づけし、適したノードを計算する
    - できるだけ分散したり、利用するイメージが既にPullされている、などで
- マニフェストで指定するスケジューリング
  - 配置したいノードの指定
    - nodeSelector: 簡易的なAffinity
    - Node Affinity: 特定のノード上で実行
    - Node AntiAffinity: 特定のノード以外で実行
    - Inter-Pod Affinity: 特定のPodがいるドメイン上で実行
    - Inter-Pod AntiAffinity: 特定のPodがいないドメイン上で実行
  - 配置したくないノードの指定
    - Taints
      - Effectで動作を指定可能
        - PreferNoSchedule: 可能な限りスケジューリングしない
        - NoSchedule: スケジューリングしない。既存Podはそのまま
        - NoExecute: 実行を許可しない。既存Podは停止
    - Tolerations
- PriorityClassでPodの優先度と退避を指定できる

## セキュリティ

### ServiceAccount

- UserAccount
  - IAMなどのクラウドプロバイダの仕組みとリンクしている。K8sの管理対象ではない
  - クラスタレベルなのでNamespaceの影響をうけない
- ServiceAccount
  - Namespaceに紐づくリソース
  - Pod起動時には必ずServiceAccountを割り当てる必要がある
- `service-account-token` タイプのSecret
  - 自動で作成されるが自作も可能。トークンと証明書から構成
  - Pod上にマウントされ、それを使ってPod内からK8sの操作ができる
  - なお、`imagePullSecrets`　が指定されたServiceAccountが割り当てられたPodが起動した場合、自動的にPodの `imagePullSecrets` として利用される

### RBAC

- Role Based Access Control
  - どういった操作を行うかのRoleを作成し、ServiceAccountなどのUserに対して紐付ける
  - AggregationRuleを利用することで複数のRoleを集約したRoleを作成できる
- NamespaceレベルとClusterレベルのものがある

### SecurityContext　/ PodSecurityContext

- 個々のコンテナに対するセキュリティの設定
  - 特権コンテナとして実行するなど
- PodSecurityContext
  - Podに対するセキュリティ設定
- `sysctls` でカーネルパラメータの指定が可能

### PodSecurityPolicy

- クラスタに対してセキュリティポリシーによる制限を行うリソース
- 作成可能なPodの制限やSecurityContextなどで設定した項目のデフォルト値設定など
  - ホワイトリスト形式

### NetworkPolicy

- Pod同士が通信する際のトラフィックルールを規定するもの
- すべてのK8s環境で利用できるわけではなくCNIプラグインなどが必要
- IngressとEgressで成り立っている
  - Ingress: インバウンド方向のトラフィックルール
  - Egress: アウトバウンド方向のトラフィックルール
  - Namespaceごとに作成する必要あり
  - Pod / Namespace / CIDR単位で通信の許可対象を指定可能

### PodPreset

- Podを作成する際のデフォルト設定を埋め込むリソース
  - 自動的に環境変数を埋め込む
  - 特定領域にPVをマウントするなど
- Podの定義と衝突する場合はPodPresetの書き換えは行われないことに注意

### Secretリソースの暗号化

- `kubesec`
  - AWS KMSなどのクラウドプロバイダの機能を使ってSecretの暗号化をすることが可能
    - ファイル全体ではなく、構造を保ったまま値だけ暗号化
- `SealedSecret`
  - 暗号化された独自のリソースをクラスタに登録しておく
    - 公開鍵と秘密鍵
  - クラスタ内部で復号化される
- `ExternalSecret`
  - AWS Secrets Managerなど外部のSecret Managerに保存されてる認証情報を読み込む

## OSS

- Helm
  - パッケージマネージャー
- Kustomize
  - マニフェストのテンプレーティングツール
  - namespaceの上書きや、prefix/suffixの付与などが可能
  - Overlay機能によってパッチを当てることが可能
    - 環境によってresourceを書き換えたり

## モニタリング

- Datadog
  - 監視方法
    - 特定のしきい値を超えた場合
    - Anomaly
      - 過去の状態と比較して異常を検知
    - Forecast
      - 将来のメトリクスがしきい値を超えそうな場合
    - Outlier
      - 特定のグループ内で他のメンバーと傾向が違うもの
      - 特定のコンテナだけパフォーマンスが悪いなど
- Prometheus
  - 一旦無視

## コンテナログの集約

- Fluentd
  - DeamonSetで各ノードにPodを配置する方法が一般的
- Datadog Logs
  - Agentが直接コンテナの標準出力から収集し、Datadogに送っている
  - 課金対象となるログのフィルタリング機能付き
