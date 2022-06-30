# Django Challenge

公式チュートリアルを参考にちょっとやってみる

[さぁ始めましょう | Django ドキュメント | Django](https://docs.djangoproject.com/ja/4.0/intro/)=

## 環境構築

VSCodeのRemote Container使うので準備。

### Dockerfile

```Dockerfile
ARG VARIANT="3.10-bullseye"
FROM mcr.microsoft.com/vscode/devcontainers/python:0-${VARIANT}

# [Choice] Node.js version: none, lts/*, 16, 14, 12, 10
ARG NODE_VERSION="none"
RUN if [ "${NODE_VERSION}" != "none" ]; then su vscode -c "umask 0002 && . /usr/local/share/nvm/nvm.sh && nvm install ${NODE_VERSION} 2>&1"; fi

ENV POETRY_HOME=/opt/poetry
RUN curl -sSL https://raw.githubusercontent.com/python-poetry/poetry/master/get-poetry.py | python && \
    cd /usr/local/bin && \
    ln -s /opt/poetry/bin/poetry && \
    chmod +x /usr/local/bin/poetry
```

### docker-compose.yml
```yml
version: '3'

services:
  mysql:
    platform: linux/x86_64
    image: mysql:8.0
    environment:
      MYSQL_ROOT_PASSWORD: password
    volumes:
      - mysql-data:/var/lib/mysql
    ports:
      - '3306:3306'

  app:
    build:
      context: .
      dockerfile: Dockerfile
      args:
        - VARIANT=3.10
        - NODE_VERSION=lts/*
    stdin_open: true
    working_dir: /workspace
    tty: true
    volumes:
      - ../:/workspace
    depends_on:
      - mysql

volumes:
  mysql-data:
```

### devcontainer.json

```json
{
	"name": "Python 3",
	"dockerComposeFile": "docker-compose.yml",
	"service": "app",
	"workspaceFolder": "/workspace",

	"customizations": {
		"vscode": {
			"settings": { 
				"python.defaultInterpreterPath": "/usr/local/bin/python",
				"python.linting.enabled": true,
				"python.linting.pylintEnabled": false,
        "python.linting.flake8Enabled": true,
				"python.formatting.autopep8Path": "/usr/local/py-utils/bin/autopep8",
				"python.formatting.blackPath": "/usr/local/py-utils/bin/black",
				"python.formatting.yapfPath": "/usr/local/py-utils/bin/yapf",
				"python.linting.banditPath": "/usr/local/py-utils/bin/bandit",
				"python.linting.flake8Path": "/usr/local/py-utils/bin/flake8",
				"python.linting.mypyPath": "/usr/local/py-utils/bin/mypy",
				"python.linting.pycodestylePath": "/usr/local/py-utils/bin/pycodestyle",
				"python.linting.pydocstylePath": "/usr/local/py-utils/bin/pydocstyle",
				"python.linting.pylintPath": "/usr/local/py-utils/bin/pylint"
			},

			"extensions": [
				"ms-python.python",
				"ms-python.vscode-pylance"
			]
		}
	},

	"remoteUser": "vscode"
}
```

各ファイルを `.devcontainer` に配置してからRemote Containerに接続

## Djangoのインストールと初期設定

```bash
poetry init
poetry add django mysqlclient

# 仮想環境を有効化
#   ※VSCode上でPythonインタープリタをPoetryの環境に切り替えておく
poetry shell
```

django-admin startproject config .
python manage.py startapp app
```

とりあえずこれで準備は完了

Django関連のその他コマンドメモ

```bash
# 開発サーバーの起動
python manage.py runserver

# マイグレーション時に発行されるSQLの確認
python manage.py sqlmigrate app_label migration_name

# 対話シェルの起動
python manage.py shell
```
