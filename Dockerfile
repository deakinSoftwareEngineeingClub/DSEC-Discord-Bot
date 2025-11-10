FROM rust:trixie AS builder

WORKDIR /usr/src/dsec_bot

# Copy manifest files first to leverage Docker layer caching for dependencies
COPY Cargo.toml Cargo.lock ./

# Create a dummy main to cache dependency compilation
RUN mkdir -p src && echo 'fn main() {}' > src/main.rs

# Build dependencies (this layer will be cached)
RUN cargo build --release

# Remove dummy and copy real source
RUN rm -rf src
COPY . .

# Build the actual application
RUN cargo build --release

# Use a small image for the final build
FROM debian:bookworm-slim

# Set the working directory
WORKDIR /usr/local/bin

# Install CA certificates for TLS/HTTPS support
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the application binary from the builder stage
COPY --from=builder /usr/src/dsec_bot/target/release/dsec_bot .

# Specify the command to run the application
CMD ["./dsec_bot"]