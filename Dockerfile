# ベースイメージとして公式のRust最新版を使用
FROM rust:latest

# コンテナ内の作業ディレクトリを設定（Rustプロジェクト用）
WORKDIR /usr/src/myapp

# Rustのバージョン確認（環境が正しく動いているかチェック）
RUN rustc --version && cargo --version

# 基本的なビルドツールをインストール（必要に応じて追加可能）
RUN apt-get update && apt-get install -y \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# コンテナ起動時のデフォルトコマンド（インタラクティブなシェルを提供）
CMD ["bash"]
