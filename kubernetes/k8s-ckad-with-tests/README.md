# Kubernetes Certified Application Developer (CKAD) with Tests

UdemyのK8s講座を使った学習のメモ

https://www.udemy.com/course/certified-kubernetes-application-developer/

## 既存Podに対する修正

- yamlがないので既存Podからyamlを取得
  - `kubectl get pod <pod-name> -o yaml > pod-definition.yaml`
- 直接定義をいじる
  - `kubectl edit pod <pod-name>`
- `kubectl create deployment nginx --image=nginx--dry-run=client -o yaml > nginx-deployment.yaml`
  - dry-runでyaml作れる

## K8sのheadless service

ヘッドレスサービスがピンときてなかったのでメモ

https://eng-blog.iij.ad.jp/archives/9998#%e3%83%98%e3%83%83%e3%83%89%e3%83%ac%e3%82%b9%e3%82%b5%e3%83%bc%e3%83%93%e3%82%b9

- Cluster IPをNone設定にして作成
- LBとしてIPを発行せず、関連するPodをラウンドロビンするサービスになる
- これをStatefulSetで行った場合、各Pod別のDNSを発行する形になる
  - `sample-statefulset-headless-0.sample-headless.default.svc.cluster.local`
  - DBなどmasterやslaveといった形でアクセスする先を切り替えるためのもの
