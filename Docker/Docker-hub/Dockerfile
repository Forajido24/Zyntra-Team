# ETAPA DE CONSTRUCCIÓN
# Usamos 1.83 o superior para evitar el error de las librerías ICU
FROM rust:1.83-slim as builder

# Instalamos las herramientas de compilación y librerías de desarrollo necesarias
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

# Copiamos los archivos del workspace
COPY Cargo.toml Cargo.lock ./

# Copiamos todas las carpetas según tu jerarquía
COPY shared/ ./shared/
COPY hub/ ./hub/
COPY api_gateway/ ./api_gateway/
COPY monitor/ ./monitor/
COPY logger/ ./logger/

# Ahora cargo build encontrará pkg-config y OpenSSL sin problemas
RUN cargo build --release

# ETAPA DE EJECUCIÓN (Imagen final ligera)
FROM debian:bookworm-slim

# Instalamos las librerías de ejecución para que los binarios funcionen
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copiamos los binarios compilados
COPY --from=builder /usr/src/app/target/release/hub /app/hub
COPY --from=builder /usr/src/app/target/release/api_gateway /app/api_gateway
COPY --from=builder /usr/src/app/target/release/monitor /app/monitor
COPY --from=builder /usr/src/app/target/release/logger /app/logger

EXPOSE 3000 3001 3002 4000

CMD ["/app/hub"]
