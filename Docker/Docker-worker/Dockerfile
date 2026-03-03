# ==============================
# Etapa 1: Build
# ==============================
FROM rust:latest as builder

WORKDIR /usr/src/app

# Copiamos workspace completo
COPY . .

# Compilamos solo el worker
RUN cargo build --release -p worker

# ==============================
# Etapa 2: Runtime ligero
# ==============================
FROM debian:bookworm-slim

WORKDIR /app

# Copiamos el binario del worker
COPY --from=builder /usr/src/app/target/release/worker /app/worker

# Certificados SSL (necesarios para reqwest)
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# El comando lo define docker-compose
