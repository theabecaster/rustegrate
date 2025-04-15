FROM rust:slim as builder

WORKDIR /app

# Install OpenSSL and pkg-config dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy the entire project for building
COPY . .

# Build the application
RUN cargo build --release

# Create a smaller runtime image
FROM debian:bookworm-slim

WORKDIR /app

# Install SSL certificates for HTTPS
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the compiled binaries
COPY --from=builder /app/target/release/rustegrate /app/
COPY --from=builder /app/target/release/telemetry-cli /app/

# Create a non-root user to run the application
RUN groupadd -r appuser && useradd -r -g appuser appuser
RUN chown appuser:appuser /app/rustegrate /app/telemetry-cli

USER appuser

# Set environment variables
ENV HOST=0.0.0.0
ENV PORT=8080
ENV LOG_LEVEL=info

EXPOSE 8080

# Run the server
CMD ["/app/rustegrate"] 