FROM rust:1.92-slim-bookworm AS builder
WORKDIR /app

# Install system dependencies for OpenSSL
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

# Build API
RUN cargo build -p storable-api --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/storable-api /usr/local/bin/storable-api

EXPOSE 8080
CMD ["storable-api"]
