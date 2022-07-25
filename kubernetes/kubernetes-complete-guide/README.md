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

 - kubectx / kubensでコンテキストの切り替え可能
