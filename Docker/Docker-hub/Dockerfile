# ==============================
# Etapa 1: Build
# ==============================
FROM rust:latest as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

# ==============================
# Etapa 2: Runtime
# ==============================
FROM debian:bookworm-slim
WORKDIR /app

COPY --from=builder /usr/src/app/target/release/hub /app/hub
COPY --from=builder /usr/src/app/target/release/api_gateway /app/api_gateway
COPY --from=builder /usr/src/app/target/release/monitor /app/monitor
COPY --from=builder /usr/src/app/target/release/logger /app/logger

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*
