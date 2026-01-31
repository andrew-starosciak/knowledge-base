# Stage 1: Build the Rust binary
FROM rust:1.83-bookworm AS builder

WORKDIR /build

# Copy manifests first for better layer caching
COPY Cargo.toml Cargo.lock ./

# Create dummy src to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN CARGO_BUILD_JOBS=2 cargo build --release
RUN rm -rf src

# Copy actual source and static files, then rebuild
COPY src ./src
COPY static ./static
RUN touch src/main.rs && CARGO_BUILD_JOBS=2 cargo build --release

# Stage 2: Runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    python3 \
    python3-pip \
    python3-venv \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install yt-dlp
RUN pip3 install --break-system-packages yt-dlp

# Create non-root user
RUN useradd -ms /bin/bash appuser

# Create directories
RUN mkdir -p /app /data && chown -R appuser:appuser /app /data

WORKDIR /app

# Copy binary from builder
COPY --from=builder /build/target/release/engine /app/engine

# Copy static files
COPY --chown=appuser:appuser static /app/static

USER appuser

# Default database location
ENV DB_PATH=/data/knowledge.db
ENV PORT=3000

EXPOSE 3000

# Default: run web server
ENTRYPOINT ["/app/engine", "--database", "/data/knowledge.db"]
CMD ["serve", "--port", "3000"]
