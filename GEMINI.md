# 🌌 Project Context: Storable

> **Mission:** A private, high-performance cloud storage and file-sharing service built entirely in Rust.
> **INCLUDE:** This is a file for more detailed information regarding the dioxus framework. [crates\storable-web\AGENTS.md](crates\storable-web\AGENTS.md)

## 1. 🏗️ Architecture Overview

**Storable** follows a clean **4-Tier Clean Architecture** organized as a Rust Workspace. It enforces strict separation of concerns to ensure testability, scalability, and type safety across the full stack (Backend to Frontend).

### The Stack
| Component | Technology | Description |
| :--- | :--- | :--- |
| **Frontend** | **Dioxus 0.7 (WASM)** | A React-like Rust framework running in the browser. Shares types directly with the backend. |
| **API Layer** | **Axum 0.7+** | High-performance async HTTP server. Handles routing, auth, and streaming. |
| **Domain Core** | **Pure Rust** | Contains business logic, types, and error definitions. No database dependencies. |
| **Data Layer** | **SurrealDB (v2.x)** | Multi-model database (Graph + Document). Handles metadata and relationships. |

## 2. 📂 Project Structure

The project is a **Rust Workspace** with the following crate hierarchy:

```text
storable/                  # Workspace Root
├── Cargo.toml             # Workspace configuration
├── docker-compose.yml     # Orchestration for DB, API, and Web
├── db/                    # Database schemas (.surql)
├── docker/                # Dockerfiles for each service
└── crates/
    ├── storable-core/     # 🧠 THE BRAIN: Shared types, errors, traits.
    ├── storable-db/       # 💾 THE VAULT: Database repositories & models.
    ├── storable-api/      # 🔌 THE GATEWAY: Axum HTTP server & handlers.
    └── storable-web/      # 🖥️ THE FACE: Dioxus frontend application.

```

## 3. 💾 Database Schema (SurrealDB)

We use **SurrealDB** with a Graph-like structure for file nodes.

* **Namespace:** `storable`
* **Database:** `main`

### Tables

* **`user`**: Authentication and profile data.
* ID Format: `user:<uuid>`


* **`node`**: Represents both **Files** and **Folders**.
* ID Format: `node:<uuid>`
* `kind`: Enum `"file"` | `"folder"`
* `owner`: Record link to `user`
* `parent`: Recursive record link to another `node` (for folder nesting).


### ⚠️ Critical Implementation Detail

SurrealDB Record IDs (`Thing`) are handled as follows:

1. **DB Layer:** Uses `surrealdb::sql::Thing`.
2. **Core Layer:** Mapped to `UserId(String)` and `FileId(String)` wrappers.
3. **Conversion:** Must happen explicitly in `storable-db/src/models.rs` via `From/Into` traits.

## 4. 📦 Crate Details & Patterns

### `storable-core` (The Domain)

* **Responsibility:** Defines *what* the system is. Zero external I/O dependencies.
* **Key Types:**
* `FileMetadata`: The standard struct exchanged between all layers.
* `StorableError`: Centralized error enum (using `thiserror`).
* `UserId`, `FileId`: Tuple structs to prevent "Stringly typed" programming.


### `storable-db` (The Repository)

* **Responsibility:** Talking to SurrealDB.
* **Pattern:** **Repository Pattern**.
* `FileRepository`: Encapsulates raw SurrealQL queries.
* **Rule:** Never return `DbFile` (internal model) to the API. Always map to `FileMetadata`.


### `storable-api` (The Service)

* **Responsibility:** HTTP handling and Business Logic orchestration.
* **Pattern:** **Service Pattern**.
* `FileService` (Trait): Defines abstract operations (upload, download).
* `StorableFileService` (Impl): Connects the Repository to the logic.


* **State:** The Service is wrapped in an `Arc` and passed to Axum handlers via `State`.

### `storable-web` (The Frontend)

* **Responsibility:** UI and Interaction.
* **Networking:** Uses `reqwest` to call the API.
* **Type Safety:** Imports `storable-core` to ensure API responses perfectly match UI expectations.

## 5. 🐳 Development Workflow

### Prerequisites

* Rust 1.83+
* Docker & Docker Compose
* Dioxus CLI (`cargo install dioxus-cli --version 0.7.0-alpha.2`)

### Quick Start

```bash
# 1. Start Database and API
docker compose up

# 2. Run Frontend (in a separate terminal)
# Navigate to crates/storable-web
dx serve --platform web

### Networking Context

* **Inside Docker:** API talks to DB via `ws://db:8000`.
* **Browser:** Frontend talks to API via `http://localhost:8080`.

## 🤖 AI Context & Rules (For LLMs)

**When generating code for Storable, adhere to these strict rules:**

1. **Architecture Violation:** Never import `storable-db` inside `storable-web`. The frontend must only depend on `storable-core` (types) and communicate via HTTP.
2. **SurrealDB Syntax:** Always use `Thing` builders for Record IDs. Do not concat strings manually for IDs (e.g., avoid `let id = "node:" + uuid`). Use `Thing { tb: "node", id: ... }`.
3. **Error Handling:** Do not unwrap (`.unwrap()`) in `storable-api`. Map errors to `StorableError` and then to appropriate HTTP Status Codes.
4. **WASM Compatibility:** In `storable-core`, ensure code is platform-agnostic. Do not use blocking I/O (`std::fs`) in code shared with the frontend.
5. **Diagrams:** If explaining a flow, prefer creating a Sequence Diagram (Mermaid) showing the path: `Web -> API (Handler) -> API (Service) -> DB (Repo) -> SurrealDB`.
