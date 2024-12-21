# Use the official Rust image as the build environment
FROM rust:1.72 AS builder

# Set the working directory
WORKDIR /app

# Copy the source code into the container
COPY . .

# Build the Rust binary in release mode
RUN cargo build --release

# Use Debian Bookworm Slim as the runtime image
FROM debian:bookworm-slim

# Set the working directory
WORKDIR /app

# Install necessary dependencies
RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/subscriber-service /app/subscriber-service

# Expose ports if needed (optional)
EXPOSE 8080

# Run the service
CMD ["./subscriber-service"]