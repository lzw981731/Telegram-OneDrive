# 使用 cargo-chef 方案提速 Rust 构建
FROM lukemathwalker/cargo-chef:latest-rust-1.85.1-alpine3.20 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
# 关键步骤：先构建依赖。如果 Cargo.toml/Cargo.lock 不变，这一层会被完美缓存
RUN apk add --update --no-cache build-base pkgconfig libressl-dev
RUN cargo chef cook --release --recipe-path recipe.json

# 构建应用程序
COPY . .
RUN cargo build --release

# 运行环境
FROM alpine:3.20
# 安装必要的运行时库（如果需要的话，这里保持与原版逻辑一致的极简环境）
RUN apk add --no-cache ca-certificates libgcc
COPY --from=builder /app/target/release/telegram-onedrive /telegram-onedrive
COPY index.html /index.html
ENV RUST_BACKTRACE=1
ENTRYPOINT [ "/telegram-onedrive" ]
