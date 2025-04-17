# Use an official Rust image as a base.
# You can pin a specific version like: FROM rust:1.77-slim-bookworm
# 'slim' uses Debian slim, which is smaller than default but usually sufficient.
# 'bookworm' is the Debian version (current stable as of early 2024).
# Using ARG allows you to override the version at build time if needed.
ARG RUST_VERSION=stable
FROM rust:${RUST_VERSION}-slim-bookworm AS builder

# Set the working directory inside the container
WORKDIR /usr/src/app

# Install essential build tools (some crates need a C compiler/linker)
# and potentially other tools you might need (e.g., openssl).
# Use --no-install-recommends to keep the image smaller.
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    build-essential \
    pkg-config \
    libssl-dev \
    # Add any other system dependencies your project might need here (e.g., libpq-dev for Postgres)
    && \
    # Clean up apt cache to reduce image size
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Copy your Cargo configuration files first.
# This leverages Docker's layer caching. If these files don't change,
# Docker won't need to re-download dependencies in subsequent builds.
COPY Cargo.toml Cargo.lock ./

# Build dependencies.
# Create a dummy main.rs file so cargo can build just the dependencies.
RUN mkdir src && \
    echo "fn main() {println!(\"Building dependencies...\");}" > src/main.rs && \
    # Build only the dependencies to cache them in this layer
    # Use --release if you primarily build release versions
    cargo build --locked && \
    # Remove the dummy source file after building dependencies
    rm -rf src

# Copy the rest of your project source code
COPY . .

# Optional: Build the project immediately (useful for CI or final image)
# For a development environment, you might skip this and build interactively later.
# RUN cargo build --release

# --- Development Stage ---
# If you just want a container to develop *in*, you might stop here
# or use a simpler base and install rustup manually.
# The official rust image is convenient because it sets up PATH etc.

# For a development environment where you want to keep the container running:
# Reset ENTRYPOINT/CMD inherited from the base Rust image if needed.
# The base rust image doesn't have a CMD that keeps it alive by default.
# Use a command that keeps the container running indefinitely.
CMD ["tail", "-f", "/dev/null"]