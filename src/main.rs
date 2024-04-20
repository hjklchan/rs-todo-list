use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Json, Router,
};
use mysql::{Pool};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    // init logger
    tracing_subscriber::fmt::init();
    
    // init db
    let db_url = "mysql://root:root@localhost:3306/todo_app";
    // create connection pool
    let pool = Pool::new(db_url).unwrap();

    // create a new http app
    let app = Router::new()
        .route("/todo-list", get(list_handler))
        .route("/todo-list", post(create_handler))
        .route("/todo-list", patch(update_handler))
        .route("/todo-list/:id", delete(delete_handler))
        .layer(TraceLayer::new_for_http());
    // with database connection pool
    let app = app.with_state(pool);

    let listener = TcpListener::bind("127.0.0.1:8888").await.unwrap();
    tracing::debug!("listening on {}", "127.0.0.1:8888");

    axum::serve(listener, app).await.unwrap();
}

/// list_handler 待做列表
async fn list_handler(State(db_pool): State<Pool>) -> impl IntoResponse {
    "get ok"
}

#[derive(Debug, Deserialize)]
pub struct CreateTodoReq {
    description: String,
    completed: bool,
}

/// create_handler 创建待做
async fn create_handler(
    State(db_pool): State<Pool>,
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
    State(db_pool): State<Pool>,
    Json(input): Json<UpdateTodoReq>,
) -> impl IntoResponse {
    // todo
    println!("{input:#?}");
    "update ok"
}

/// delete_handler 删除待做
async fn delete_handler(Path(id): Path<String>, State(db_pool): State<Pool>) -> impl IntoResponse {
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
