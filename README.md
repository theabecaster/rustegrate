# Rustegrate: Device Telemetry API

A Rust-based API for device telemetry ingestion and monitoring, built to demonstrate modern backend patterns and cloud architecture.

## Architecture

This project follows a clean, modular architecture with clear separation of concerns:

- **API Layer**: Handles HTTP requests and responses using Actix-Web
- **Service Layer**: Contains business logic and coordinates between API and storage
- **Storage Layer**: Manages data persistence (in-memory with DashMap)
- **Models**: Domain entities and data transfer objects
- **Configuration**: Environment-based application settings
- **Error Handling**: Consistent error responses and logging

## Features

- REST API for telemetry data:
  - POST telemetry data (temperature, humidity, pressure)
  - GET telemetry history for a device with filtering options
  - DELETE outdated records
- In-memory storage using DashMap for concurrent access
- Optional database support (SQLite/Postgres) via feature flags
- CLI tool for simulating device telemetry
- Structured logging with tracing
- Configuration via environment variables and .env files
- Docker support for containerized deployment
- CI pipeline with GitHub Actions

## API Endpoints

- `POST /api/v1/telemetry` - Create a new telemetry record
- `GET /api/v1/telemetry/{id}` - Get a specific telemetry record by ID
- `GET /api/v1/devices/{device_id}/telemetry` - Get telemetry history for a device
- `DELETE /api/v1/devices/{device_id}/telemetry` - Delete old telemetry records
- `GET /api/v1/health` - Health check endpoint

## Getting Started

### Prerequisites

- Rust 1.70+ (stable)
- Docker and Docker Compose (optional, for containerized deployment)

### Running Locally

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/rustegrate.git
   cd rustegrate
   ```

2. Build and run the API server:
   ```bash
   cargo run
   ```

3. Use the CLI tool to send simulated telemetry data:
   ```bash
   # Send a single data point
   cargo run -p telemetry-cli -- send -d device-001 -t 23.5 -h 45.0 -p 1013.0
   
   # Continuously simulate telemetry (one reading every 5 seconds)
   cargo run -p telemetry-cli -- simulate -d device-001 -i 5
   ```

### Environment Variables

Create a `.env` file in the project root with the following variables:

```
HOST=127.0.0.1
PORT=8080
LOG_LEVEL=debug
# Optional: DATABASE_URL=postgres://postgres:postgres@localhost:5432/telemetry
```

### Docker Deployment

1. Build and run using Docker Compose:
   ```bash
   docker-compose up --build
   ```

2. For production deployment, you can uncomment the database section in `docker-compose.yml` to use PostgreSQL.

## Azure Cloud Deployment Considerations

This project is designed with Azure cloud deployment in mind:

- **Azure Container Registry**: Store the Docker image
- **Azure Kubernetes Service**: Deploy and scale the API
- **Azure Database for PostgreSQL**: Persistent storage (optional)
- **Azure Monitor**: Collect telemetry and logs
- **Azure Key Vault**: Store secrets
- **Azure Event Hubs**: Optional integration for event-driven architecture

## Testing

Run the test suite:

```bash
cargo test
```

## License

This project is licensed under the MIT License - see the LICENSE file for details. 