FROM rust:1.92-slim-bookworm AS builder
WORKDIR /app

# Install system dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy root Cargo.toml and ALL crates
COPY . .

# Build the API crate from the workspace root
RUN cargo build -p storable-api --release

# Final runtime stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/storable-api /usr/local/bin/storable-api

EXPOSE 8080
CMD ["storable-api"]