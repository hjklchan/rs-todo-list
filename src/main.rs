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
const DATABASE_DSN: &'static str = "mysql://root:@localhost:3306/test";

// Models
#[derive(Debug, Serialize, Clone)]
struct Todo {
    id: i64,
    description: String,
    completed: i8,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // init logger
    tracing_subscriber::fmt::init();

    // init db
    // create connection pool
    let pool: Pool<MySql> = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(DATABASE_DSN)
        .await?;

    // create a new http app
    let app = Router::new()
        .route("/todo-list", get(list_handler))
        .route("/todo-list", post(create_handler))
        .route("/todo-list", patch(update_handler))
        .route("/todo-list/:id", delete(delete_handler))
        .layer(TraceLayer::new_for_http());
    // with database connection pool
    let app = app.with_state(pool);

    // create a listener and binding it
    let listener = TcpListener::bind(SERVER_ADDR).await?;
    tracing::debug!("listening on {}", SERVER_ADDR);
    // start app
    axum::serve(listener, app).await?;

    // everything is ok
    Ok(())
}

/// list_handler 待做列表
async fn list_handler(
    State(db_pool): State<Pool<MySql>>,
) -> Result<Json<Vec<Todo>>, (StatusCode, String)> {
    let recs = sqlx::query_as!(
        Todo,
        r#"SELECT `id`, `description`, `completed` FROM `todo_list` ORDER BY `id` DESC"#
    ).fetch_all(&db_pool).await.map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    Ok(Json(recs))
}

#[derive(Debug, Deserialize)]
pub struct CreateTodoReq {
    description: String,
}

/// create_handler 创建待做
async fn create_handler(
    State(db_pool): State<Pool<MySql>>,
    Json(input): Json<CreateTodoReq>,
) -> Result<(StatusCode, Json<Todo>), (StatusCode, String)> {
    let query = r#"INSERT INTO `todo_list`(`description`, `completed`) VALUES(?, false)"#;
    let query_result = sqlx::query(query)
        .bind(&input.description)
        .execute(&db_pool)
        .await;

    if let Err(err) = query_result {
        _ = err;
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("failed to create new todo"),
        ));
    };

    let last_insert_id = query_result.map_or(0, |res| res.last_insert_id());

    return Ok((
        StatusCode::CREATED,
        Json(Todo {
            id: last_insert_id as i64,
            description: input.description,
            completed: 0,
        }),
    ));
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodoReq {
    description: String,
    completed: bool,
}

/// update_handler 更新待做
async fn update_handler(
    State(_db_pool): State<Pool<MySql>>,
    Json(input): Json<UpdateTodoReq>,
) -> impl IntoResponse {
    // todo
    let _query = "TODO";
    println!("{input:#?}");
    "update ok"
}

/// delete_handler 删除待做
async fn delete_handler(
    Path(id): Path<String>,
    State(db_pool): State<Pool<MySql>>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let query: &'static str = "DELETE FROM `todo_list` WHERE `id` = ?";
    let query_result = sqlx::query(query).bind(id).execute(&db_pool).await;

    if let Err(err) = query_result {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()));
    }

    Ok((StatusCode::OK, "delete ok".into()))
}
