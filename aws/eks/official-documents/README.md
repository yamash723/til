# EKSの公式ドキュメントメモ

[Amazon Elastic Kubernetes Service ドキュメント](https://docs.aws.amazon.com/ja_jp/eks/index.html)

## eksctlのインストール

```
brew tap weaveworks/tap
brew install weaveworks/tap/eksctl
eksctl version
```

## クラスターとノードの作成

クラスター作成時にはノードタイプの選択が可能

- Fargate
- マネージド（EC2インスタンスを使う場合はこちら）

```
eksctl create cluster --name hello-world-cluster --region ap-northeast-1 --fargate
```

## ネームスペース作成

```
kubectl create namespace eks-sample-app
```

## マニフェスト適用

```
kubectl apply -f eks-sample-deployment.yaml
```

## PodがRunningにならない

```
kubectl get pods -n eks-sample-app

NAME                                           READY   STATUS    RESTARTS   AGE
eks-sample-linux-deployment-65b7669776-9zzhz   0/1     Pending   0          25m
eks-sample-linux-deployment-65b7669776-kcllb   0/1     Pending   0          25m
eks-sample-linux-deployment-65b7669776-xck6s   0/1     Pending   0          25m
```

```
kubectl describe pod -n eks-sample-app

Events:
  Type     Reason            Age                From               Message
  ----     ------            ----               ----               -------
  Warning  FailedScheduling  0s (x27 over 27m)  default-scheduler  0/2 nodes are available: 2 node(s) had taint {eks.amazonaws.com/compute-type: fargate}, that the pod didn't tolerate.
```

Pod が許可できないテイントがノードにあるのでスケジュールできないもよう

[Amazon EKS でのポッドステータスのトラブルシューティング](https://aws.amazon.com/jp/premiumsupport/knowledge-center/eks-pod-status-troubleshooting/)


```
kubectl get nodes -o custom-columns=NAME:.metadata.name,TAINTS:.spec.taints

NAME                                                        TAINTS
fargate-ip-192-168-114-28.ap-northeast-1.compute.internal   [map[effect:NoSchedule key:eks.amazonaws.com/compute-type value:fargate]]
fargate-ip-192-168-99-219.ap-northeast-1.compute.internal   [map[effect:NoSchedule key:eks.amazonaws.com/compute-type value:fargate]]
```

調べてみたらFargateプロファイルの追加が必要だった

[GoのアプリをEKS(Fargate)にデプロイしてみた話](https://zenn.dev/hisamitsu/articles/53d0dd94e08b2b#fargate%E3%83%97%E3%83%AD%E3%83%95%E3%82%A1%E3%82%A4%E3%83%AB%E3%82%92%E5%A4%89%E6%9B%B4%E3%81%99%E3%82%8B)

```
eksctl create fargateprofile \
    --cluster hello-world-cluster \
    --name fp-eks-sample-app \
    --namespace eks-sample-app
```

一度リセットして再度マニフェスト読み込み

```
kubectl delete -f eks-sample-deployment.yaml
kubectl apply -f eks-sample-deployment.yaml
```

無事動いた

```
kubectl get pods -n eks-sample-app

NAME                                           READY   STATUS    RESTARTS   AGE
eks-sample-linux-deployment-65b7669776-29j8r   1/1     Running   0          93s
eks-sample-linux-deployment-65b7669776-58jn4   1/1     Running   0          93s
eks-sample-linux-deployment-65b7669776-9pf42   1/1     Running   0          93s
```

Pod内の確認も問題なし

```
kubectl exec -it pod/eks-sample-linux-deployment-65b7669776-29j8r -n eks-sample-app -- /bin/bash
curl eks-sample-linux-service
cat /etc/resolv.conf
```

お掃除

```
kubectl delete namespace eks-sample-app
eksctl delete cluster --name hello-world-cluster --region ap-northeast-1
```
