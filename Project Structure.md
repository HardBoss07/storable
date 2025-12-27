# Project Structure

```
storable/                # repo root
├── Cargo.toml            # workspace
├── README.md
├── LICENSE
├── docker/
│   ├── Dockerfile
│   └── docker-compose.yml
├── crates/
│   ├── storable-core/   # domain logic (publishable)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── auth.rs
│   │       ├── fs.rs
│   │       ├── metadata.rs
│   │       └── errors.rs
│   │
│   ├── storable-db/     # DB abstraction (publishable)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── models.rs
│   │       └── repo.rs
│   │
│   ├── storable-api/    # HTTP API (publishable)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs
│   │       ├── routes/
│   │       └── middleware.rs
│   │
│   └── storable-web/    # Dioxus frontend (not published)
│       ├── Cargo.toml
│       └── src/
│           └── main.rs
│
└── scripts/
    └── init.sh
```