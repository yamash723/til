# TerraformでEKSクラスタを作ってみる

最低限の記述はこれだけ

```yml
terraform {
  required_version = "~> 1.2.3"

  backend "s3" {
    bucket = "yamash723-tfstate-bucket"
    key    = "study-k8s/terraform.tfstate"
    region = "ap-northeast-1"
  }

  required_providers {
    aws = {
      version = "~> 4.20.1"
      source  = "hashicorp/aws"
    }
    kubernetes = {
      source  = "hashicorp/kubernetes"
      version = "~> 2.11.0"
    }
  }
}

provider "aws" {
  region  = "ap-northeast-1"
  profile = "terraform"
}

locals {
  cluster_name = "study-k8s-cluster"
}

module "vpc" {
  source  = "terraform-aws-modules/vpc/aws"
  version = "~> 3.14.2"

  name = local.cluster_name
  cidr = "10.0.0.0/16"

  azs = [
    "ap-northeast-1a",
    "ap-northeast-1c",
    "ap-northeast-1d"
  ]

  public_subnets = [
    "10.0.0.0/24",
    "10.0.1.0/24",
    "10.0.2.0/24"
  ]

  private_subnets = [
    "10.0.100.0/24",
    "10.0.101.0/24",
    "10.0.102.0/24"
  ]

  enable_nat_gateway   = true
  single_nat_gateway   = true
  enable_dns_hostnames = true
}

module "eks" {
  source  = "terraform-aws-modules/eks/aws"
  version = "~> 18.24.1"

  cluster_name    = local.cluster_name
  cluster_version = "1.22"

  vpc_id     = module.vpc.vpc_id
  subnet_ids = module.vpc.private_subnets

  enable_irsa                    = true
  cluster_endpoint_public_access = true

  eks_managed_node_groups = {
    blue = {}
    green = {
      min_size     = 2
      max_size     = 2
      desired_size = 2

      instance_types = ["t3.small"]
      capacity_type  = "SPOT"
    }
  }
}
```

```bash
$ terraform init
$ terraform plan
$ terraform apply

$ aws --profile terraform eks update-kubeconfig --name study-k8s-cluster
$ kubectl get pods -A

$ terraform destroy
```
