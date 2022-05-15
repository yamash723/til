# Kubernetes公式ドキュメントメモ

[Kubernetesドキュメント | Kubernetes](https://kubernetes.io/ja/docs/home/)

ドキュメントを読み進めた際のメモを下記に随時残していく

## コンセプト

[コンセプト | Kubernetes](https://kubernetes.io/ja/docs/concepts/)

### Kubernetesとは何か？

> Kubernetesは、宣言的な構成管理と自動化を促進し、コンテナ化されたワークロードやサービスを管理するための、ポータブルで拡張性のあるオープンソースのプラットフォームです。Kubernetesは巨大で急速に成長しているエコシステムを備えており、それらのサービス、サポート、ツールは幅広い形で利用可能です。
> 
> Kubernetesの名称は、ギリシャ語に由来し、操舵手やパイロットを意味しています。Googleは2014年にKubernetesプロジェクトをオープンソース化しました。Kubernetesは、本番環境で大規模なワークロードを稼 働させたGoogleの15年以上の経験と、コミュニティからの最高のアイディアや実践を組み合わせています。


### Kubernetesが提供するもの

- サービスディスカバリーと負荷分散
  - DNS名または独自のIPアドレスを使ってコンテナを公開可能
  - トラフィックに応じてネットワークトラフィックを分散させることが可能
- ストレージ オーケストレーション
  - ローカルストレージやパブリッククラウドプロバイダーなど、選択したストレージシステムを自動でマウント可能
- 自動化されたロールアウトとロールバック
  - デプロイのためのコンテナ作成、削除など
- 自動ビンパッキング
  - 要件とリソースに基づいてコンテナを自動で配置
- 自己修復
  - 処理に失敗したコンテナの再起動など
- 機密情報と構成管理
  - パスワードやOAuthトークン、SSHキーなどの機密情報を保持、管理できる

### Kubernetesマスター

クラスターの望ましい状態を維持することが責務。`kubectl` を使った場合はこのマスターとやり取りしていることになる。

マスターは可用性と冗長性のために複製も可能。

#### コンポーネント

- kube-apiserver: Kubernetes APIを外部に提供するKubernetesコントロールプレーンのコンポーネント（水平スケール可能）
- kube-scheduler: 各Podに対してNodeの割当を行う
- kube-controller-manager
  - ノードコントローラー: ノードがダウンした場合の通知と対応を担当
  - レプリケーションコントローラー: システム内の全レプリケーションコントローラーオブジェクトについて、Podの数を正しく保つ
  - エンドポイントコントローラー: ServiceとPodを紐付ける = エンドポイントオブジェクトを注入します
  - サービスアカウントとトークンコントローラー: 新規の名前空間に対して、デフォルトアカウントとAPIアクセストークンを作成
- etcd: 「あるべき状態」を保持する分散型KVS
- cloud-controller-manager: クラウドサービス特有の制御ロジックを担当
  - ノードコントローラー: ノードが応答を停止した後、クラウドで削除されたかどうかを判断するため、クラウドプロバイダーをチェック
  - ルーティングコントローラー: 基盤であるクラウドインフラでルーティングを設定
  - サービスコントローラー: クラウドプロバイダーのロードバランサーの作成、更新、削除を行う

#### マスターとノード間の通信

- クラスター → マスター
  - `kube-apiserver` のHTTPSエンドポイントへのリクエスト
- マスター → クラスター
  - `kubelet` のHTTPSエンドポイントへのリクエスト
  - もしくはSSHトンネリング ※現在非推奨

### Kubernetesノード

アプリケーションとクラウドワークフローを実行するマシンやVMのこと。アプリケーションのコンポーネントである `Pod` をホストする
運用者は基本このノードと直接やり取りすることはない。

ノー上の `kubelet` がコントロールプレーンに自己登録するか、手動でNodeオブジェクトを登録することでノードを追加することが出来る

#### コンポーネント

- kubelet: Masterと通信
- kube-proxy: 各ノードのKubernetesネットワークサービスを反映するネットワークプロキシ
- コンテナランタイム: コンテナの実行を担当

#### ノードのステータス確認

```
kubectl describe node <ノード名をここに挿入>
```

- Addresses
- Conditions: Runningなノードのステータス
  - Ready: 有効なノードでProdを配置可能かどうか
  - DiskPressure: ノードのディスク容量が圧迫されている
  - MemoryPressure: ノードのメモリが圧迫されている
  - PIDPressure: プロセスが圧迫されている
  - NetworkUnavailable: ネットワークが適切に設定されていない
- Capacity: ノードが持っているリソースの合計量
- Allocatable: 通常のPodによって消費されるノード上のリソースの量
- Info: カーネルのバージョンやOS名など一般的な情報

### アドオン

DNSやWebUIなどのアドオンがある

### Service

Podの集合で実行されているアプリケーションをネットワークサービスとして公開する方法

ServiceSpecのtype指定で公開方法を変えることができる

- ClusterIP (既定値)
  - クラスター内の内部IPでServicを公開。クラスター内からのみ到達可能
- NodePort
    - NAT使用。<NodeIP>:<NodePort>でクラスター外部からアクセス可能
    - ClusterIPのスーパーセット
- LoadBalancer
    - 外部ロードバランサを作成し固定の外部IPを割り当てる
    - NodePortのスーパーセット
- ExternalName
    - externalNameで指定した名前のCNAMEレコードを返し、任意の名前を使って公開

### Pod

- 1つ以上のコンテナで構成されたデプロイ可能なユニット
- StatefulSetリソースを使うとステートフルなPodを使うことが出来る
  - レプリカセット内のPodそれぞれで独自のデータを保持する
  - PodごとにDNSで名前解決できるホスト名を付ける場合にも使える

### ローリングアップデート

下記が可能

- コンテナイメージのアップデートを介した、ある環境から別の環境へのアプリケーションの昇格
- 以前のバージョンへのロールバック
- ダウンタイムなしでのアプリケーションのCI/CD

### ConfigMap

機密性のないデータをキーと値のペアで保存するために使用されるAPIオブジェクト

PodにConfigMap使用している状態で、ConfigMapだけを更新してもPodに反映されない。作り直しが必要

```bash
kubectl apply -f example-redis-config.yaml
kubectl apply -f redis-pod.yaml

kubectl describe configmap/example-redis-config

# 何かしら編集
vi example-redis-config.yaml

# ConfigMapに反映。k8s上でも変わっているが、Podには反映されていない
kubectl apply -f example-redis-config.yaml
kubectl describe configmap/example-redis-config

# これで作り直すと反映される
kubectl delete pod redis
kubectl apply -f redis-pod.yaml
```

## ネットワーク

- Endpointリソース
  - ロードバランサーがルーティングする宛先を管理するもの
- ヘッドレスサービス
  - 内部ロードバランサーを構成せず、DNSの設定だけを行うモード
  - Endpointのアドレスが直接DNSに登録され、ラウンドロビンで動作する

## チュートリアル

[チュートリアル | Kubernetes](https://kubernetes.io/ja/docs/tutorials/)

> プロダクションのトラフィックを処理するKubernetesクラスターには、最低3つのノードが必要です

CLI各種操作方法

```
# ネームスペースの作成
kubectl create namespace

# リソースの一覧を表示
kubectl get nodes
kubectl get service
kubectl get deployments
kubectl get rs
kubectl get pods
kubectl get pods --output=wide # IPなどの追加情報も確認できる
kubectl get event # k8sのイベントを確認できる

# デプロイの作成  
kubectl create deployment

# サービスの作成
kubectl expose deployment

# クラスター外からクラスター内部のIPにアクセスするための機能
kubectl proxy

# Podに対してラベル操作を行う
kubectl label pods

# リソースの詳細情報を表示
kubectl describe

# マニフェストファイルの反映
kubectl apply

# リソースの削除
kubectl delete

# Pod上のコンテナログを表示
kubectl logs

# Pod上のコンテナでコマンドを実行
kubectl exec

# デプロイやレプリカセットのスケーリング
kubectl scale

# コンテナイメージの更新
kubectl set image

# ロールアウト操作
kubectl rollout status
kubectl rollout undo
```

### Tips

minikubeでLoadBalancerを使ったServiceを作る場合は、別ターミナルで `minikube tunnel` を立ち上げておけばOK