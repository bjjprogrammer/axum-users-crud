# Axum Users CRUD

A small REST API written in Rust with [Axum](https://github.com/tokio-rs/axum) that implements a full CRUD for users. Data is stored **in memory**, so it is lost when the server restarts.

The project is a learning exercise focused on a clean, layered architecture that can later be switched to a real database by replacing only the repository layer.

## Tech stack

- [Rust](https://www.rust-lang.org/) (edition 2024)
- [Axum 0.8](https://docs.rs/axum/0.8) for routing and HTTP handling
- [Tokio](https://tokio.rs/) as the async runtime
- [Serde](https://serde.rs/) / `serde_json` for JSON serialization

## Getting started

### Prerequisites

- Rust toolchain 1.85 or newer (required for edition 2024). Install it with [rustup](https://rustup.rs/).

### Run the server

```bash
cargo run
```

The server listens on `http://0.0.0.0:3000`.

Check that it is alive:

```bash
curl localhost:3000/checkAlive
# OK
```

## API

### Endpoints

| Method | Path          | Description               | Success response |
|--------|---------------|---------------------------|------------------|
| GET    | `/checkAlive` | Health check              | `200` `OK`       |
| GET    | `/users`      | List all users            | `200` + array    |
| POST   | `/users`      | Create a user             | `201` + user     |
| GET    | `/users/{id}` | Get a user by id          | `200` + user     |
| PUT    | `/users/{id}` | Partially update a user   | `200` + user     |
| DELETE | `/users/{id}` | Delete a user             | `204` no body    |

### Data model

```json
{
  "id": 1,
  "name": "Ana",
  "email": "ana@mail.com"
}
```

- `POST /users` requires both `name` and `email`.
- `PUT /users/{id}` accepts `name`, `email`, or both. Missing fields are left unchanged.

### Business rules

- `name` must not be empty or whitespace only.
- `email` must contain `@`.
- `email` must be unique across users.

### Errors

Errors produced by the application use a consistent JSON body:

```json
{ "error": "Usuario no encontrado" }
```

| Status | When                                                          |
|--------|---------------------------------------------------------------|
| `400`  | Validation failed (empty name, invalid email)                 |
| `404`  | The user does not exist                                       |
| `409`  | The email is already registered by another user               |

Some requests are rejected by Axum itself before reaching the handlers. These responses are plain text, not the JSON format above:

| Status | When                                                          |
|--------|---------------------------------------------------------------|
| `400`  | Malformed JSON body, or a non-numeric id such as `/users/abc` |
| `405`  | HTTP method not supported for that path                       |
| `415`  | Missing `Content-Type: application/json` header               |
| `422`  | Valid JSON that does not match the expected shape             |

### Examples

```bash
# Create
curl -X POST localhost:3000/users \
  -H 'Content-Type: application/json' \
  -d '{"name":"Ana","email":"ana@mail.com"}'

# List
curl localhost:3000/users

# Get one
curl localhost:3000/users/1

# Update only the name
curl -X PUT localhost:3000/users/1 \
  -H 'Content-Type: application/json' \
  -d '{"name":"Ana María"}'

# Delete
curl -X DELETE localhost:3000/users/1
```

## Testing with Postman

The file [`axum-users-crud.postman_collection.json`](./axum-users-crud.postman_collection.json) contains a ready-to-use collection with test scripts for every endpoint, covering both success and error cases.

1. Start the server with `cargo run`.
2. In Postman, click **Import** and select the collection file.
3. Right-click the collection, choose **Run collection**, then **Run**.

Requests must run in order: the create requests store the generated `userId`, `userEmail` and `secondUserId` in collection variables that later requests depend on. Emails are randomized with `{{$randomEmail}}`, so the collection can be run repeatedly without restarting the server.

The base URL is stored in the `baseUrl` collection variable (`http://localhost:3000` by default).

## Project structure

```text
src/
├── main.rs              # Entry point: builds the state and starts the server
├── state.rs             # AppState shared across all requests
├── error.rs             # AppError and its conversion into HTTP responses
├── models/
│   └── user.rs          # User entity and request DTOs (CreateUser, UpdateUser)
├── repositories/
│   └── users.rs         # In-memory storage (HashMap behind a RwLock)
├── services/
│   └── users.rs         # Business rules and validation
├── handlers/
│   ├── health.rs        # Health check handler
│   └── users.rs         # HTTP handlers: extract input, call services, build responses
└── routes/
    ├── mod.rs           # Main router, mounts every resource
    └── users.rs         # /users routes
```

## Architecture

Each request flows through the layers in one direction, and each layer only knows about the one below it:

```text
routes → handlers → services → repositories → HashMap
                       │
                       └─ AppError → JSON response with the right status code
```

| Layer          | Responsibility                                                                 |
|----------------|--------------------------------------------------------------------------------|
| `routes`       | Map URLs and HTTP methods to handlers                                          |
| `handlers`     | Translate HTTP into Rust: extract `State`, `Path` and `Json`, return responses |
| `services`     | Apply business rules and turn missing data into `AppError`                     |
| `repositories` | Store and retrieve data, with no business logic                                |
| `models`       | Define the data shapes used by every other layer                               |

### Shared state

The repository is wrapped as `Arc<UserRepository>` inside `AppState`, and stores its data in a `RwLock<HashMap<u64, User>>`:

- `Arc` lets every request share the same repository instance cheaply.
- `RwLock` allows many concurrent readers or one exclusive writer, keeping the map consistent across threads.

### Known limitations

- Data lives in memory and is lost on restart.
- The duplicate-email check and the insert are separate operations, so two simultaneous requests with the same email could both succeed. A database `UNIQUE` constraint would solve this.
- Email validation is intentionally minimal (it only checks for `@`).

## Next steps

- Replace the in-memory repository with a database (for example, PostgreSQL with `sqlx`).
- Customize Axum's JSON and path rejections so every error uses the same JSON format.
- Add structured logging with `tracing` and `tower-http`.
- Add integration tests under `tests/`.
- Load configuration such as the port from environment variables.
