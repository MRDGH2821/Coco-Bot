# syntax=docker/dockerfile:1

# Multi-architecture build support
FROM --platform=$BUILDPLATFORM tonistiigi/xx:1.6.1 AS xx

# Build stage - Use official Rust image with cross-compilation support
FROM --platform=$BUILDPLATFORM rust:1.87-slim-bookworm AS builder

# Copy cross-compilation helper
COPY --from=xx / /

# Install build dependencies and cross-compilation tools
# hadolint ignore=DL3008
RUN apt-get update && apt-get install --no-install-recommends -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    git \
    clang \
    gcc-aarch64-linux-gnu \
    gcc-arm-linux-gnueabihf \
    libc6-dev-arm64-cross \
    libc6-dev-armhf-cross \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy dependency manifests and build script first for better caching
COPY Cargo.toml Cargo.lock build.rs ./

# Copy git folder for build context (if needed for version info, etc.)
COPY .git/ ./.git/

# Set up Rust cross-compilation target
ARG TARGETPLATFORM
RUN case "$TARGETPLATFORM" in \
    "linux/amd64") echo "x86_64-unknown-linux-gnu" > /tmp/target ;; \
    "linux/arm64") echo "aarch64-unknown-linux-gnu" > /tmp/target ;; \
    "linux/arm/v7") echo "armv7-unknown-linux-gnueabihf" > /tmp/target ;; \
    *) echo "Unsupported platform: $TARGETPLATFORM" && exit 1 ;; \
    esac

# Pre-fetch dependencies with cross-compilation setup
RUN RUST_TARGET=$(cat /tmp/target) && \
    rustup target add "${RUST_TARGET}" && \
    xx-apt-get update && xx-apt-get install --no-install-recommends -y \
    pkg-config \
    libssl-dev

# Copy the actual source code
COPY src/ ./src/

# Build the actual application
RUN RUST_TARGET=$(cat /tmp/target) && \
    PKG_CONFIG_ALLOW_CROSS=1 \
    xx-cargo build --release --target "${RUST_TARGET}" && \
    cp target/"${RUST_TARGET}"/release/coco-bot /app/coco-bot && \
    xx-verify /app/coco-bot

# Runtime stage - Use minimal base image
FROM debian:bookworm-slim

# Install runtime dependencies
# hadolint ignore=DL3008
RUN apt-get update && apt-get install --no-install-recommends -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# Create a non-root user
RUN groupadd -r coco && useradd -r -g coco -s /bin/false coco

# Set the working directory
WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/coco-bot /app/coco-bot

# Copy assets (fonts and meme templates)
COPY --chown=coco:coco src/assets/ /app/assets/

# Make binary executable and change ownership
RUN chmod +x /app/coco-bot && \
    chown -R coco:coco /app

# Switch to non-root user
USER coco

# Health check (optional - adjust based on your bot's capabilities)
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD pgrep -x coco-bot || exit 1

# Expose any ports if needed (Discord bots typically don't need this)
# EXPOSE 8080

# Set environment variables
ENV RUST_LOG=info
ENV RUST_BACKTRACE=1

# Run the application
CMD ["/app/coco-bot"]

# Build arguments for dynamic labeling
ARG BUILDTIME
ARG VERSION
ARG REVISION

# Labels for better container management
LABEL org.opencontainers.image.title="Coco Bot"
LABEL org.opencontainers.image.description="Rust port of KittyBot for the CS@unimelb Discord server."
LABEL org.opencontainers.image.version="${VERSION:-dev}"
LABEL org.opencontainers.image.authors="MRDGH2821 <ask.mrdgh2821@outlook.com>"
LABEL org.opencontainers.image.url="https://github.com/MRDGH2821/Coco-Bot"
LABEL org.opencontainers.image.source="https://github.com/MRDGH2821/Coco-Bot"
LABEL org.opencontainers.image.licenses="Apache-2.0"
LABEL org.opencontainers.image.created="${BUILDTIME}"
LABEL org.opencontainers.image.revision="${REVISION}"