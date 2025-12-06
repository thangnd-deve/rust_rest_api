# REST API

A REST API project built with Rust, designed for building scalable web services with PostgreSQL database integration.

## Project Structure

```
rest_api/
├── src/
│   ├── main.rs          # Application entry point
│   ├── config.rs        # Configuration management
│   ├── database.rs      # Database connection and setup
│   ├── model.rs         # Data models and schemas
│   └── repository.rs    # Database operations and queries
├── request/
│   └── index.http       # HTTP request examples for testing
├── .env.example         # Environment variables template
├── .gitignore          # Git ignore rules
└── Cargo.toml          # Project dependencies and metadata
```

## Features

- RESTful API architecture
- PostgreSQL database integration
- JWT authentication support
- Environment-based configuration
- Modular code organization

## Prerequisites

- Rust (Edition 2024)
- PostgreSQL database
- Cargo package manager

## Getting Started

### 1. Clone the repository

```bash
git clone <repository-url>
cd rest_api
```

### 2. Set up environment variables

Copy the example environment file and configure your settings:

```bash
cp .env.example .env
```

Edit `.env` with your configuration:

```env
DATABASE_URL=postgresql://user:password@127.0.0.1:5432/db
DATABASE_USER=user
DATABASE_PASSWORD=password
DATABASE_HOST=127.0.0.1
DATABASE_PORT=5432
DATABASE_NAME=db
JWT_SECRET=your-secret-key-here-minimum-32-characters
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
RUST_LOG=info
```

### 3. Set up PostgreSQL database

Ensure PostgreSQL is installed and running, then create your database:

```bash
psql -U postgres
CREATE DATABASE db;
```

### 4. Build and run

```bash
# Build the project
cargo build

# Run the application
cargo run

# Run in release mode
cargo build --release
cargo run --release
```

The server will start on `http://127.0.0.1:8080` by default.

## API Endpoints

Example API endpoints (as defined in `request/index.http`):

### Get Resource
```http
GET http://127.0.0.1:8080/api/v1/resource
```

### Create Resource
```http
POST http://127.0.0.1:8080/api/v1/resource
Content-Type: application/json

{
  "name": "example",
  "value": 42
}
```

### Update Resource
```http
PUT http://127.0.0.1:8080/api/v1/resource/1
Content-Type: application/json

{
  "name": "updated_example",
  "value": 100
}
```

### Delete Resource
```http
DELETE http://127.0.0.1:8080/api/v1/resource/1
```

## Testing

You can test the API endpoints using the provided HTTP request file in the `request/` directory with tools like:
- VS Code REST Client extension
- IntelliJ HTTP Client
- curl or Postman

## Development

### Project Modules

- **config.rs**: Handles application configuration and environment variables
- **database.rs**: Manages database connections and connection pooling
- **model.rs**: Defines data structures and database schemas
- **repository.rs**: Implements data access layer and database queries
- **main.rs**: Application entry point and server setup

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| DATABASE_URL | Full PostgreSQL connection string | - |
| DATABASE_USER | Database username | user |
| DATABASE_PASSWORD | Database password | password |
| DATABASE_HOST | Database host address | 127.0.0.1 |
| DATABASE_PORT | Database port | 5432 |
| DATABASE_NAME | Database name | db |
| JWT_SECRET | Secret key for JWT (min 32 chars) | - |
| SERVER_HOST | Server host address | 127.0.0.1 |
| SERVER_PORT | Server port | 8080 |
| RUST_LOG | Logging level (trace, debug, info, warn, error) | info |

## Building for Production

```bash
cargo build --release
```

The optimized binary will be available at `target/release/rest_api`.

## License

[Add your license information here]

## Contributing

[Add contributing guidelines here]
