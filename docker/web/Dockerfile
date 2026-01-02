FROM rust:1.92-slim-bookworm
WORKDIR /app

# 1. Install dependencies for Dioxus & WASM
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

# 2. Install WASM target and Dioxus CLI
RUN rustup toolchain install stable
RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-binstall
RUN cargo binstall dioxus-cli --force

# 3. Copy the ENTIRE workspace (Required for paths to resolve)
COPY . .

# 4. Navigate to the web crate
WORKDIR /app/crates/storable-web

# 5. Serve the app on port 3000
EXPOSE 3000
CMD ["dx", "serve", "--port", "3000"]