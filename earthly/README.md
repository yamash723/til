# Earthly

Makefile + Dockerのようなビルドツール

## Docs

- [Earthly - Better Builds](https://earthly.dev/)

## Tutorial

[Get Earthly - Better Builds](https://earthly.dev/get-earthly)

### Install

```
$ brew install earthly/earthly/earthly && earthly bootstrap
```

ビルド用のコンテナを立ち上げるので、Dockerは事前に起動しておくこと

```
 !  ~/g/g/y/til   *  brew install earthly/earthly/earthly && earthly bootstrap                                                                      3m  土  7/ 9 09:52:25 2022
Warning: earthly/earthly/earthly 0.6.19 is already installed and up-to-date.
To reinstall 0.6.19, run:
  brew reinstall earthly
           buildkitd | Starting buildkit daemon as a docker container (earthly-buildkitd)...
           buildkitd | ...Done
           bootstrap | Bootstrapping successful.
 ~/g/g/y/til   *…  docker ps                                                                                                                      22.3s  土  7/ 9 09:53:46 2022
CONTAINER ID   IMAGE                       COMMAND                  CREATED          STATUS          PORTS                                                NAMES
2aa05daf7919   earthly/buildkitd:v0.6.19   "/usr/bin/entrypoint…"   57 seconds ago   Up 57 seconds   127.0.0.1:8371->8371/tcp, 127.0.0.1:8373->8373/tcp   earthly-buildkitd
 ~/g/g/y/til   *… 
```

また各テキストエディタでシンタックスハイライト用の拡張が出ているので必要に応じて追加。
