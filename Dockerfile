# ベースイメージとして公式Rustイメージを使用
FROM rust:1.82.0

# 必要なツールをインストール
RUN apt-get update && apt-get install -y \
    build-essential \
    libssl-dev \
    pkg-config \
    curl \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# cargo-watchをインストール（開発時のホットリロードに使用）
RUN cargo install cargo-watch

# 作業ディレクトリを作成
WORKDIR /app

# ホストマシンのコードをコピー
COPY . .

# 依存関係をダウンロードしてビルドキャッシュを作成
RUN cargo build --release

# コンテナ起動時のデフォルトコマンド
CMD ["cargo", "watch", "-x", "run"]
