# Terraform

## 参考資料

- [Terraform by HashiCorp](https://www.terraform.io/)

## Terraformとは

[Documentation | Terraform by HashiCorp](https://www.terraform.io/docs)

> Terraform is an infrastructure as code (IaC) tool that allows you to build, change, and version infrastructure safely and efficiently. This includes both low-level components like compute instances, storage, and networking, as well as high-level components like DNS entries and SaaS features.

インフラをコードで表現するためのOSSであり、複数のクラウドプラットフォームを抽象化したもの。

## 使い方

```bash
terraform init
terraform plan # 実行計画
terraform apply
terraform destroy
```

## ベストプラクティス

[Terraform を使用するためのベスト プラクティス  |  Google Cloud](https://cloud.google.com/docs/terraform/best-practices-for-terraform?hl=ja)

## CloudFormationとの比較

[AKIBA.AWS ONLINE #03 -「TerraformとCloudFormationどちらを採用すべき？」というタイトルで登壇しました | DevelopersIO](https://dev.classmethod.jp/articles/terraform-or-cloudformation/)

- CloudFormation
  - AWS提供
  - JSONかYamlで記述
  - AWS上で実行される
- Terraform
  - HashiCorp提供
  - 独自のHCLで記述
  - 実行した環境（ローカルPC）などのマシン上で実行

ロックインにはなるがAWSしか使わないならCloudFormationでもいいのでは、という感想。

## 関連ツール

- tfenv
  - [Terraformのマルチバージョン管理(tfenv)](https://zenn.dev/gekal/articles/tfenv-multi-terraform)
- tflint
  - [terraform-linters/tflint: A Pluggable Terraform Linter](https://github.com/terraform-linters/tflint)
- tfnotify
  - planやapply結果をSlackやGitHubに通知
- tfsec
  - terraformのコードからセキュリティ問題を検知
- tfupdate
  - terraformでのDependabot
