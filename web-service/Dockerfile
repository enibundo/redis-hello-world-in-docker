# Use a Rust image based on Debian for the builder
FROM rust:1.72-bullseye AS builder

WORKDIR /app
COPY . .

# Build the Rust application
RUN cargo build --release

# Runtime environment
FROM debian:bullseye-slim
WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the built binary
COPY --from=builder /app/target/release/web-service /app/web-service

# Expose the port and start the service
EXPOSE 8080
CMD ["./web-service"]