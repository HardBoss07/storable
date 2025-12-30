# Project Structure

```
storable/                  # repo root
├── Cargo.toml              # workspace
├── README.md
├── LICENSE
├── docker-compose.yml
├── crates/
│   ├── storable-core/      # domain logic (publishable)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── auth.rs
│   │       ├── fs.rs
│   │       ├── metadata.rs
│   │       ├── errors.rs
│   │       └── types.rs
│   │
│   ├── storable-db/        # DB abstraction (publishable)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── models.rs
│   │       └── repositories.rs
│   │
│   ├── storable-api/       # HTTP API (publishable)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs
│   │       └── services.rs
│   │
│   └── storable-web/       # Dioxus frontend (not published)
│       ├── Cargo.toml
│       └── src/
│           └── main.rs
│
├── db/
│   └── schema.surql
│
└── docker/
    ├── db/
    │   └── schema.surql
    ├── api/
    │   └── Dockerfile
    └── web/
        └── Dockerfile
```