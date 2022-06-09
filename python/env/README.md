# Python開発環境構築メモ

## ランタイム

asdfを使用

```bash
asdf install python 3.10.5
asdf global python 3.10.5
asdf reshim python                              


echo -e "\nsource "(brew --prefix asdf)"/asdf.fish" >> ~/.config/fish/config.fish
```

ターミナル再起動

## VSCode拡張

- Python
  - インストール後、オプションからFlake8とMypyをEnable化
- Python Type Hint
