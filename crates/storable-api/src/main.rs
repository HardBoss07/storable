use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get},
    Json, Router,
};
use std::sync::Arc;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

// Import your internal modules
mod services;
use crate::services::{FileService, StorableFileService};
use storable_core::metadata::FileMetadata;
use storable_core::types::FileId;

// Type alias to make the State signature cleaner
type SharedService = Arc<dyn FileService + Send + Sync>;

#[tokio::main]
async fn main() {
    // 1. Initialize DB Connection (Matches your docker-compose env)
    let db = Surreal::new::<Ws>("db:8000")
        .await
        .expect("DB Connect Fail");
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .unwrap();
    db.use_ns("storable").use_db("main").await.unwrap();

    // 2. Setup Layers
    let repo = storable_db::repositories::FileRepository::new(db);
    let service = Arc::new(StorableFileService::new(repo)) as SharedService;

    // 3. Build the Router
    let app = Router::new()
        .route("/api/files/:id", get(get_file))
        .route("/api/files/:id", delete(delete_file))
        .with_state(service);

    // 4. Start the Server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("🚀 API Server running on http://0.0.0.0:8080");
    axum::serve(listener, app).await.unwrap();
}

// --- HANDLERS ---

async fn get_file(
    State(service): State<SharedService>,
    Path(id): Path<String>,
) -> Result<Json<FileMetadata>, (StatusCode, String)> {
    let file_id = FileId(id);

    service
        .download(&file_id)
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e)))
}

async fn delete_file(
    State(service): State<SharedService>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    let file_id = FileId(id);

    service
        .delete(&file_id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e)))
}
