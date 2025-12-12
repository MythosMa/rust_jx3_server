# 构建阶段：使用官方 Rust 镜像
FROM rust:latest AS builder

WORKDIR /app

# 复制依赖信息，利用 Docker 缓存加速构建
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --locked

# 复制全部源码并真正构建
COPY . .
RUN cargo build --release --locked

# 运行阶段：使用 Debian Slim（必须 glibc）
FROM debian:bookworm-slim

WORKDIR /app

# 运行依赖
RUN apt-get update && apt-get install -y \
  ca-certificates \
  && rm -rf /var/lib/apt/lists/*

# 复制 Rust 二进制
COPY --from=builder /app/target/release/rust_jx3_server .

RUN chmod +x rust_jx3_server

EXPOSE 3000

CMD ["./rust_jx3_server"]
