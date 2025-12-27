FROM rust:1.92-slim-bookworm
WORKDIR /app

# Install system dependencies for OpenSSL
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY crates/storable-web ./storable-web

# Optional: install dioxus-cli if needed for dev
# RUN cargo install dioxus-cli --locked

EXPOSE 3000
CMD ["cargo", "run", "-p", "storable-web"]
