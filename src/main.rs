use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

const SERVER_ADDR: &'static str = "127.0.0.1:8888";
const DATABASE_DSN: &'static str = "mysql://root:root@localhost:3306/todo_app";

#[tokio::main]
async fn main() {
    // init logger
    tracing_subscriber::fmt::init();

    // init db
    // create connection pool
    let pool: Pool<MySql> = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(DATABASE_DSN)
        .await
        .unwrap();

    // create a new http app
    let app = Router::new()
        .route("/todo-list", get(list_handler))
        .route("/todo-list", post(create_handler))
        .route("/todo-list", patch(update_handler))
        .route("/todo-list/:id", delete(delete_handler))
        .layer(TraceLayer::new_for_http());
    // with database connection pool
    let app = app.with_state(pool);

    let listener = TcpListener::bind(SERVER_ADDR).await.unwrap();
    tracing::debug!("listening on {}", SERVER_ADDR);

    axum::serve(listener, app).await.unwrap();
}

/// list_handler 待做列表
async fn list_handler(
    State(db_pool): State<Pool<MySql>>,
) -> Result<Json<Vec<TodoModel>>, (StatusCode, String)> {
    Ok(Json(Vec::new()))
}

#[derive(Debug, Deserialize)]
pub struct CreateTodoReq {
    description: String,
    completed: bool,
}

/// create_handler 创建待做
async fn create_handler(
    State(db_pool): State<Pool<MySql>>,
    Json(input): Json<CreateTodoReq>,
) -> impl IntoResponse {
    println!("{input:#?}");
    StatusCode::CREATED
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodoReq {
    description: String,
    completed: bool,
}

/// update_handler 更新待做
async fn update_handler(
    State(db_pool): State<Pool<MySql>>,
    Json(input): Json<UpdateTodoReq>,
) -> impl IntoResponse {
    // todo
    println!("{input:#?}");
    "update ok"
}

/// delete_handler 删除待做
async fn delete_handler(
    Path(id): Path<String>,
    State(db_pool): State<Pool<MySql>>,
) -> impl IntoResponse {
    _ = id;
    "delete ok"
}

// Models
#[derive(Debug, Serialize, Clone)]
struct TodoModel {
    id: u64,
    description: String,
    completed: bool,
}
