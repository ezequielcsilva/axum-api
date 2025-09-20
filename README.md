# Axum API - DDD + Clean Architecture

A modern Rust API built with Axum, following Domain-Driven Design (DDD) and Clean Architecture principles.

## 🏗️ Architecture Overview

This project implements a clean, scalable architecture with clear separation of concerns:

```
src/
├── domain/                      # 🏛️ Domain Layer (Pure Business Logic)
│   └── todos/                   # Todo Aggregate
│       ├── entities/            # Domain entities
│       ├── traits/              # Domain interfaces (ISP)
│       └── value_objects/       # DTOs, Pagination, etc.
├── application/                 # 🎯 Application Layer (Use Cases)
│   └── todos/                   # Todo Use Cases
│       ├── create_todo/         # Create Todo Use Case
│       ├── get_todo/            # Get Todo Use Case
│       ├── list_todos/          # List Todos Use Case
│       ├── update_todo/         # Update Todo Use Case
│       └── delete_todo/         # Delete Todo Use Case
├── infrastructure/              # 🔧 Infrastructure Layer
│   └── database/                # Database implementations
│       └── repositories/        # Repository implementations
├── api/                         # 🌐 API Layer (Interface)
│   └── handlers/                # HTTP handlers
│       ├── health.rs            # Health check handler
│       └── todo_handlers.rs     # Todo CRUD handlers
├── app.rs                       # Route configuration
├── state.rs                     # Application state
├── error.rs                     # Error handling
├── doc.rs                       # OpenAPI documentation
└── main.rs                      # Entry point
```

## 🎯 Architecture Layers

### 1. Domain Layer (`src/domain/`)
- **Purpose**: Contains pure business logic and rules
- **Dependencies**: None (zero external dependencies)
- **Contains**:
  - **Entities**: Core business objects (Todo)
  - **Value Objects**: DTOs, Pagination, etc.
  - **Traits**: Domain interfaces following Interface Segregation Principle

### 2. Application Layer (`src/application/`)
- **Purpose**: Orchestrates business logic through use cases
- **Dependencies**: Only Domain layer
- **Contains**:
  - **Use Cases**: Specific business operations
  - **Orchestration**: Coordinates between domain and infrastructure

### 3. Infrastructure Layer (`src/infrastructure/`)
- **Purpose**: Implements external concerns (database, external APIs)
- **Dependencies**: Domain and Application layers
- **Contains**:
  - **Repositories**: Database implementations
  - **External Services**: Third-party integrations

### 4. API Layer (`src/api/`)
- **Purpose**: Handles HTTP requests and responses
- **Dependencies**: Application layer
- **Contains**:
  - **Handlers**: HTTP route handlers
  - **Serialization**: Request/response mapping

## 🔄 Dependency Flow

```
HTTP Request → API Layer → Application Layer → Domain Layer
                     ↓
              Infrastructure Layer → Database
```

## 🚀 Features

- ✅ **RESTful API** with Axum
- ✅ **PostgreSQL** database with SQLx
- ✅ **Pagination** support
- ✅ **OpenAPI/Swagger** documentation
- ✅ **Docker** support
- ✅ **Clean Architecture** implementation
- ✅ **Domain-Driven Design** principles
- ✅ **Interface Segregation** (SOLID)
- ✅ **Use Cases** pattern
- ✅ **Repository** pattern
- ✅ **Performance Testing** endpoint
- ✅ **Direct Database Processing**

## 🛠️ Technologies Used

- **Rust** - Systems programming language
- **Axum** - Web framework
- **SQLx** - Async SQL toolkit
- **PostgreSQL** - Database
- **Docker** - Containerization
- **Utoipa** - OpenAPI documentation
- **Chrono** - Date and time handling
- **UUID** - Unique identifiers
- **Thiserror** - Error handling

## 🚀 Getting Started

### Prerequisites

- Rust (latest stable)
- Docker and Docker Compose
- PostgreSQL (via Docker)

### Installation

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd axum-api
   ```

2. **Start PostgreSQL with Docker**
   ```bash
   docker run --name postgres-axum-api \
     -e POSTGRES_PASSWORD=postgres \
     -e POSTGRES_DB=axum_api \
     -p 5432:5432 \
     -d postgres:15
   ```

3. **Start the API Server**
   ```bash
   cargo run --bin axum-api
   ```

The API will be available at `http://localhost:3000`

## 📚 API Documentation

- **Swagger UI**: `http://localhost:3000/docs`
- **OpenAPI JSON**: `http://localhost:3000/api-docs/openapi.json`

## 🔗 API Endpoints

### Health Check
- `GET /health` - Health check endpoint

### Todos
- `GET /todos` - List todos (paginated)
- `POST /todos` - Create a new todo
- `GET /todos/{id}` - Get a specific todo
- `PUT /todos/{id}` - Update a todo
- `DELETE /todos/{id}` - Delete a todo
- `GET /todos/done/{done}` - Get todos by completion status

### Performance Testing
- `POST /todos/performance-test` - Test system performance with direct database processing
  - **Request Body**: `{"message_count": 100, "batch_size": 20}`
  - **Response**: Performance metrics (todos/sec, success rate, etc.)

### Query Parameters

#### Pagination
- `page` (optional): Page number (default: 1)
- `limit` (optional): Items per page (default: 10, max: 100)

## 🏗️ Project Structure Details

### Domain Layer
```
src/domain/todos/
├── entities/
│   └── todo.rs          # Todo entity
├── traits/
│   └── mod.rs           # Domain interfaces
└── value_objects/
    └── mod.rs           # DTOs and value objects
```

### Application Layer
```
src/application/todos/
├── create_todo/         # Create Todo Use Case
├── get_todo/            # Get Todo Use Case
├── list_todos/          # List Todos Use Case
├── update_todo/         # Update Todo Use Case
└── delete_todo/         # Delete Todo Use Case
```

### Infrastructure Layer
```
src/infrastructure/
└── database/
    └── repositories/
        └── postgres_todo_repository.rs
```

### API Layer
```
src/api/
└── handlers/
    ├── health.rs        # Health check
    └── todo_handlers.rs # Todo CRUD operations
```

## 🧪 Testing

The project is structured to support comprehensive testing:

- **Unit Tests**: Test individual components in isolation
- **Integration Tests**: Test the complete flow
- **Use Case Tests**: Test business logic
- **Repository Tests**: Test data access

## 🚀 Performance Testing

The system includes a built-in performance testing endpoint that demonstrates Rust's high-performance database processing capabilities:

### How it works:
1. **API Endpoint**: `POST /todos/performance-test` creates multiple todos directly in the database
2. **Batch Processing**: Todos are created in configurable batches
3. **Direct Database**: No message queue overhead, direct PostgreSQL operations
4. **Metrics**: Real-time performance metrics are returned

### Example Usage:
```bash
# Test with 100 todos, batch size of 20
curl -X POST http://localhost:3000/todos/performance-test \
  -H "Content-Type: application/json" \
  -d '{"message_count": 100, "batch_size": 20}'
```

### Performance Results:
- **Up to 92 todos/second** processing rate
- **100% success rate** in database operations
- **Sub-millisecond** response times
- **Direct database processing** with PostgreSQL

## 🔧 Development

### Adding New Features

1. **Domain**: Add entities, value objects, and traits
2. **Application**: Create use cases
3. **Infrastructure**: Implement repositories
4. **API**: Add handlers and routes

### Adding New Domains

1. Create new domain folder: `src/domain/{domain_name}/`
2. Create application use cases: `src/application/{domain_name}/`
3. Implement repositories: `src/infrastructure/database/repositories/`
4. Add API handlers: `src/api/handlers/`

## 📝 License

This project is licensed under the MIT License.

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## 📞 Support

For questions or support, please open an issue in the repository.