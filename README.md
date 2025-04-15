# Rustegrate: iRacing Telemetry API

A Rust-based API for iRacing telemetry ingestion and monitoring, built to demonstrate modern backend patterns and cloud architecture.

## Architecture

This project follows a clean, modular architecture with clear separation of concerns:

- **API Layer**: Handles HTTP requests and responses using Actix-Web
- **Service Layer**: Contains business logic and coordinates between API and storage
- **Storage Layer**: Manages data persistence (in-memory with DashMap)
- **Models**: Domain entities and data transfer objects
- **Configuration**: Environment-based application settings
- **Error Handling**: Consistent error responses and logging

## Features

- REST API for iRacing telemetry data:
  - POST telemetry data (vehicle performance, lap times, tire temps, etc.)
  - GET telemetry history for a driver with filtering options
  - GET telemetry data for a specific session
  - GET available tracks and cars
  - DELETE outdated records
- In-memory storage using DashMap for concurrent access
- Optional database support (SQLite/Postgres) via feature flags
- CLI tool for simulating iRacing telemetry
- Structured logging with tracing
- Configuration via environment variables and .env files
- Docker support for containerized deployment
- CI pipeline with GitHub Actions

## API Endpoints

- `POST /api/v1/telemetry` - Create a new telemetry record
- `GET /api/v1/telemetry/{id}` - Get a specific telemetry record by ID
- `GET /api/v1/drivers/{driver_id}/telemetry` - Get telemetry history for a driver
- `DELETE /api/v1/drivers/{driver_id}/telemetry` - Delete old telemetry records
- `GET /api/v1/sessions/{session_id}/telemetry` - Get telemetry data for a session
- `GET /api/v1/racing/tracks` - Get all available tracks
- `GET /api/v1/racing/cars` - Get all available cars
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
   cargo run -p telemetry-cli -- send -d driver-001 -s session-001 -t "Daytona" -c "NASCAR Cup Series Next Gen Chevrolet Camaro ZL1" --session-type "Race" --speed 180.5 -r 7500 -g 4 -l 1 -f 95.5
   
   # Continuously simulate telemetry (one reading every second)
   cargo run -p telemetry-cli -- simulate -d driver-001 -t "Daytona" -c "NASCAR Cup Series Next Gen Chevrolet Camaro ZL1" -i 1 -l 10
   ```

### Environment Variables

Create a `.env` file in the project root with the following variables:

```
HOST=127.0.0.1
PORT=8080
LOG_LEVEL=debug
# For the CLI tool
SERVER_URL=http://localhost:8080
# Optional: DATABASE_URL=postgres://postgres:postgres@localhost:5432/telemetry
```

### Docker Deployment

1. Build and run using Docker Compose:
   ```bash
   docker-compose up --build
   ```

2. For production deployment, you can uncomment the database section in `docker-compose.yml` to use PostgreSQL.

## Telemetry Data Model

The iRacing telemetry model includes:

- **Session Information**: Driver ID, session ID, track name, car model
- **Vehicle Performance**: Speed, RPM, gear, throttle/brake/clutch positions
- **Lap Information**: Current lap, lap times
- **Vehicle Status**: Fuel level, tire temperatures
- **Forces and Position**: G-forces, track position, 3D coordinates

## License

This project is licensed under the MIT License - see the LICENSE file for details. 