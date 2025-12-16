# .docker/Dockerfile

# --- 构建阶段 ---
FROM rust:bookworm AS builder

WORKDIR /app

# 1. 复制依赖文件
COPY Cargo.toml Cargo.lock ./

# 2. 创建临时 src/main.rs（让 Cargo 认为这是个有效二进制包）
RUN mkdir -p src && echo "fn main() { panic!(\"This is a dummy binary!\"); }" > src/main.rs

# 3. 现在可以安全运行 cargo fetch（会缓存依赖）
RUN cargo fetch --locked

# 4. 删除临时文件，替换为真实源码
RUN rm -f src/main.rs
COPY src ./src

# 5. 真正构建
RUN SQLX_OFFLINE=true cargo build --release --locked


# --- 运行阶段 ---
FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && \
  apt-get install -y --no-install-recommends ca-certificates && \
  rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/rust_jx3_server .

EXPOSE 3000

CMD ["./rust_jx3_server"]