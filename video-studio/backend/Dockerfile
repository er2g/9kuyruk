# Multi-stage build for Rust backend
FROM rust:1.75-slim as builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /build

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source
COPY src ./src
COPY migrations ./migrations

# Build release binary
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies (FFmpeg, PostgreSQL client, OpenSSL)
RUN apt-get update && apt-get install -y \
    ffmpeg \
    libpq5 \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /build/target/release/video-studio /app/backend

# Create storage directory
RUN mkdir -p /app/storage

# Expose API port
EXPOSE 3000

# Run backend
CMD ["/app/backend"]
