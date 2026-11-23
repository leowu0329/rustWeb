# ==========================================
# 第一階段：建置環境 (Builder Stage)
# ==========================================
FROM rust:slim-bookworm AS builder

WORKDIR /app

# 1. 複製依賴清單並建立虛擬的 lib.rs 與 main.rs，預先編譯所有外部套件
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && \
    echo "" > src/lib.rs && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# 2. 複製真實原始碼與 Askama 模板資料夾
COPY src ./src
COPY templates ./templates

# 3. 更新檔案時間戳記以觸發專案本體重新編譯，產出最佳化執行檔
RUN touch src/main.rs src/lib.rs && cargo build --release

# ==========================================
# 第二階段：極簡執行環境 (Runtime Stage)
# ==========================================
FROM debian:bookworm-slim AS runtime

WORKDIR /app

# 安裝基礎憑證與時區資料
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    tzdata \
    && rm -rf /var/lib/apt/lists/*

ENV TZ=Asia/Taipei

# 建立專門存放 SQLite 資料庫的資料夾，方便掛載 Volume
RUN mkdir -p /app/data

# 僅從 Builder 階段複製最終編譯好的單一執行檔
COPY --from=builder /app/target/release/rust_web /app/rust_web

EXPOSE 3000

CMD ["/app/rust_web"]
