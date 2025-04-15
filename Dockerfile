FROM rust:slim as builder

WORKDIR /app

# Install OpenSSL and pkg-config dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./
COPY telemetry-cli/Cargo.toml ./telemetry-cli/

# Create empty source files to build dependencies
RUN mkdir -p src/api src/models src/services src/config src/storage src/errors telemetry-cli/src
RUN touch src/lib.rs src/main.rs telemetry-cli/src/main.rs
RUN echo "fn main() {}" > src/main.rs
RUN echo "fn main() {}" > telemetry-cli/src/main.rs
RUN echo "pub mod api; pub mod models; pub mod services; pub mod config; pub mod storage; pub mod errors;" > src/lib.rs

# Build dependencies
RUN cargo build --release

# Remove the source code files created above
RUN rm -rf src telemetry-cli/src

# Copy actual source code
COPY src/ ./src/
COPY telemetry-cli/src/ ./telemetry-cli/src/
COPY tests/ ./tests/

# Build the actual application
RUN cargo test --release
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