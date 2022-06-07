# K8s ChangeLog

K8sのChangeLogを読んでいく。なお読むのは1.19以降のみ。

※学習のベースにしようと思っている[Kubernetes完全ガイド 第2版](https://www.amazon.co.jp/dp/B08FZX8PYW/)が1.18準拠のため

また全部目を通すとしんどいのでWhat's New!のみ確認

## 資料

[kubernetes/CHANGELOG at master · kubernetes/kubernetes](https://github.com/kubernetes/kubernetes/tree/master/CHANGELOG)

## v1.24.0

[kubernetes/CHANGELOG-1.24.md at master · kubernetes/kubernetes](https://github.com/kubernetes/kubernetes/blob/master/CHANGELOG/CHANGELOG-1.24.md#major-themes)

- 非推奨になっていたdockershimコンポーネントが削除
- 今後新しいベータAPIはデフォルトで有効化されなくなる
  - 既存のものは有効化のまま
- ストレージプラグインの移行作業進行中

### GA

- ストレージ容量のトラッキング
- 自動ボリューム拡張
- NonPreemptingPriority
  - Pod preemptionの有効・無効を管理するオプション
### Beta

- OpenApi v3でAPIを公開する機能
- gRPC probe機能
  - HTTPエンドポイントの公開
  - startup/liveness/readiness probeを設定可能
- kubeletのimage credential providersのサポート
### Alpha

- リリース成果物に対するcosignによる署名
  - 実験的なサポート
  - SLSA準拠関連
- Contextual Logging
  - 関数の呼び出し元がロギングの各種設定を制御（出力の書式設定、詳細度、追加の値、名前）
- ClusterIP用のIPアドレスソフトリザーブ
  - 特性に応じて静的/動的にIPアドレスを割り当てる際に衝突リスクを軽減できる

## v1.23.0

[kubernetes/CHANGELOG-1.23.md at master · kubernetes/kubernetes](https://github.com/kubernetes/kubernetes/blob/master/CHANGELOG/CHANGELOG-1.23.md#whats-new-major-themes)

- FlexVolumeが非推奨化
- ロギングオプションのいくつかが非推奨化
  - [System Logs | Kubernetes](https://kubernetes.io/docs/concepts/cluster-administration/system-logs/#klog)
- SLSA Lv1準拠のための証明書ファイル生成
- KubeletがCRI v1をサポート & デフォルト化
- kube-schedulerの複数拡張点を持つプラグインの設定を簡略化
- CSI Migrationの更新

### GA

- IPv4/IPv6 デュアルスタックネットワーキング
- HorizontalPodAutoscaler v2
  - 自動スケールアウト
- Generic ephemeral volumes
- Skip Volume Ownership
  - マウント次の再帰的なパーミッション変更をスキップ
- CSIドライバのfsGroupのパーミッション宣言機能

### Beta

- PodSecurity
  - PodSecurityPolicyの代替
- 構造化ロギング

## v1.22.0

[kubernetes/CHANGELOG-1.22.md at master · kubernetes/kubernetes](https://github.com/kubernetes/kubernetes/blob/master/CHANGELOG/CHANGELOG-1.22.md#whats-new-major-themes)

- いくつかのベータ版APIの削除
  - 今後はGA版などを利用
- K8sのリリース頻度を年4回から年3回へ
- Windows向け開発ツールの提供
- etcdのバージョンを3.5.0へ更新

### GA

- Kubernetesクライアントのcredential plugin
- Server-side Apply
  - Applyする際にクライアントサイドではなく文字通りサーバーサイドで差分計算をする
  - フィールドの所有者をトラッキングするマージアルゴリズム
- CSIServiceAccountToken

### Alpha

- kubeadmのコントロールプレーンを非rootで実行可能
- Kubernetesノードのシステムスワップサポート
- クラスターレベルでのなseccompのデフォルト設定
- メモリの割り当てをコントロール&分離するためにAPI提供
- エフェメラルコンテナのAPI変更 & 改善
  - 新しいAPIになったので `kubectl` が古いと新しいAPIは呼べないことに注意

## v1.21.0

[kubernetes/CHANGELOG-1.21.md at master · kubernetes/kubernetes](https://github.com/kubernetes/kubernetes/blob/master/CHANGELOG/CHANGELOG-1.21.md#whats-new-major-themes)

- PodSecurityPolicy非推奨
  - 代替手段が今後出てくる
- APIリファレンスの移設
  - [Kubernetes API | Kubernetes](https://kubernetes.io/docs/reference/kubernetes-api/)
- kubectlのkustomizeがアップデート
  - v2.0.3 -> v4.0.5
- デフォルトコンテナのアノテーション
  - kubectl.kubernetes.io/default-containerアノテーションをつかってコンテナを事前に指定することができる
  - 複数コンテナを含むPodの場合、コンテナリストの先頭に対してコマンドが実行されるので、意図しないコマンド実行を引き起こしていた
- リリースタイムライン内kubeletが構造化ロギングを採用

### GA

- Secret / ConfigMapのイミュータブル化

### Beta

- ストレージ容量のトラッキング
- Generic ephemeral volumes
- CSIServiceAccountToken
- CSIヘルスモニタリング（Second Alpha）

## v1.20.0

[kubernetes/CHANGELOG-1.20.md at master · kubernetes/kubernetes](https://github.com/kubernetes/kubernetes/blob/master/CHANGELOG/CHANGELOG-1.20.md#whats-new-major-themes)

- Dockershim非推奨化
- client-go クレデンシャルプラグインが環境変数を介して現在のクラスタ情報を渡せるようになった
- go1.15.5対応
- K8sコンポーネントログのサニタイズ機能
  - `k8s.io/klog` のFilter
- 実行中Podのリソースメトリクス取得
  - 参考 : [Kubernetes 1.20: Metrics Changes と SIG Instrumentation の変更内容 - Qiita](https://qiita.com/watawuwu/items/ab5a0f11210d15cc6a86#pod-resource-metrics)
- kubeadmの非推奨機能削除
- Cloud Controller Managerのバイナリ非提供
  - 各クラウドプロバイダが自分たちで提供することを期待

### GA

- SupportNodePidsLimit / SupportPodPidsLimit
  - Node / Pod単位での消費可能なPID数の制限
- CSI Volume Snapshot
- TokenRequest / TokenRequestProjection
  - ServiceAccountで発行したトークンをPodのVolumeに動的にマウントする機能
- node.k8s.io APIグループがv1beta1からv1に昇格

### Beta

- API Priority / Fairness
  - kube-apiserverが優先度に応じて受けたリクエストをカテゴライズするもの
- Non-recursive Volume Ownership (FSGroup)
  - fsgroup設定時であっても、必要な場合のみ再帰的なパーミッションの変更を行う機能
- CSIDriver policy (FSGroup)
- RootCAConfigMap
  - BoundServiceAccountTokenVolumeから分離
- kubectl debug
  - ただしエフェメラルコンテナはAlphaのままなので注意
- setHostnameAsFQDN
  - Podのホスト名を完全修飾ドメイン名とする

### Alpha

- CronJobコントローラーv2
- IPv4/IPv6デュアルスタックの再実装
- CSIServiceAccountTokenの導入

## v1.19.0

[kubernetes/CHANGELOG-1.19.md at master · kubernetes/kubernetes](https://github.com/kubernetes/kubernetes/blob/master/CHANGELOG/CHANGELOG-1.19.md#whats-new-major-themes)

- 非推奨APIを使用していた場合に警告を表示するようになった
  - 警告はAdmission Webhooksで取得可能
- API実装における新しいポリシーをv1.20に適用するとのアナウンス（ベータ機能のライフサイクル）
  - 9ヶ月以内にGAに到達しベータ版を削除するか、新しいベータ版を用意して以前のベータ版を廃止する。のどちらか
  - 機能が長期間ベータ版のままになることを防ぐため
- EndpointSlicesのデフォルト有効化
- Kubernetesコンテナイメージがコミュニティ管理下へ
- Kubernetes Dashboardのv2がリリース
- Kubernetesのサポートを9ヶ月から1年へ延長

### GA

- Ingress
- seccomp（(Secure computing mode）

### Beta

- KubeSchedulerConfiguration（kube-schedulerの振る舞いをカスタマイズ）
  - [スケジューラーの設定 | Kubernetes](https://kubernetes.io/ja/docs/reference/scheduling/config/)
- CSI Migration - AzureDisk & vSphere
- Secret / ConfigMapをイミュータブルとしてマーク可能
- Windows向けCSI Proxy
- Windowsのcontainerdサポート

### Alpha

- Pod/Nodeデバッグ用のCLIサポート拡張（既存Alpha機能の拡張）
  - 対象Podに対してデバッグコンテナをサイドカーとして追加
  - Nodeデバック用のPodを作成する
    - ホスト側のネームスペースを使い、かつホストのrootをコンテナの/hostにマウントしたPodを作成するもの
- ストレージ容量のトラッキング
   - CSIドライバー向けのストレージ容量をレポートするAPI追加
- CSIヘルスモニタリング
  - CSIドライバーが下位レイヤーのストレージから異常なボリュームの状態をK8sと共有できるようになった
- Generic ephemeral volumes
  - エフェメラルボリュームのプラグインサポート
  - ダイナミックプロビジョニングをサポートしたストレージドライバをエフェメラルボリュームとして使用できるようになった
- 構造化ロギング
  - コンポーネントのログを構造化
  - プレーンテキスト → JSONか？
