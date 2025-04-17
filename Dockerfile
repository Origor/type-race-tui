# Dockerfile for a Rust application using Ratatui

# Use a specific stable version of Rust on a slim Debian base (Bookworm is current stable)
# Check Docker Hub (https://hub.docker.com/_/rust) for the latest stable version number
ARG RUST_VERSION=1.86.0
FROM rust:${RUST_VERSION}-slim-bookworm AS builder

# Set the working directory
WORKDIR /usr/src/app

# Install build tools and common dependencies.
# Ratatui itself has few direct system dependencies (especially with crossterm),
# but your other crates might need things like libssl-dev, or specific C libraries.
# Add any other system dependencies your project needs here.
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    build-essential \
    pkg-config \
    libssl-dev \
    # Add other necessary libraries like: libudev-dev, libsqlite3-dev, etc. if needed by your crates
    && \
    # Clean up apt cache
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Copy Cargo configuration files
COPY Cargo.toml Cargo.lock ./

# Build dependencies only (leveraging Docker cache)
# Create a dummy main.rs to allow building dependencies without full source
RUN mkdir src && \
    echo "fn main() {println!(\"Building dependencies...\");}" > src/main.rs && \
    # Build dependencies using --release profile
    cargo build --release --locked && \
    # Remove dummy source
    rm -rf src

# Copy the rest of your application source code
COPY . .

# Build the application for release
RUN cargo build --release --locked

# --- Final Stage ---
# We can create a smaller final image by copying only the binary
# Use a minimal base image like debian:bookworm-slim
FROM debian:stable-slim AS final

# Install runtime dependencies (e.g., libssl, ca-certificates)
# If your app needs specific .so files at runtime, install them here.
# Often, for pure Rust or crates linking statically/using musl, this might be minimal.
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    # Add any other runtime libraries needed by your compiled binary
    && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/rust-type-race-tui .

# Set the entrypoint for the container
CMD ["./rust-type-race-tui"]
