# gRPC公式ドキュメント

[Introduction to gRPC | gRPC](https://grpc.io/docs/what-is-grpc/introduction/)

## ｇRPCとはなにか


`Remote Procedure Calls` を実現するプロトコル。Google開発。`Protocol Buffers` などのIDLを使用してインターフェースの定義をし、対応している各言語別のコードを生成する、というのが基本的な流れ。

また通信には4種類あり、必要に応じて使い分けをする形になる。

1. Unary RPC (シンプル RPC)
2. Server streaming RPC (サーバーサイドストリーミング RPC)
3. Client streaming RPC (クライアントサイドストリーミング RPC)
4. Bidirectional streaming RPC (双方向ストリーミング RPC)


## Quick Start

```bash
pip install grpcio
pip install grpcio-tools # Protocol Buffersのコンパイラやコード生成のプラグインなど

# サンプルコード
git clone -b v1.46.3 --depth 1 --shallow-submodules https://github.com/grpc/grpc
cd grpc/examples/python/helloworld

# 別コンソールで立ち上げる
python greeter_server.py
python greeter_client.py
```
