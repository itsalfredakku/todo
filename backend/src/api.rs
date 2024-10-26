use axum::{
    extract::{Form, Path, State},
    routing::{get, post},
    Json, Router,
};
use axum::response::Redirect;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use crate::db;

#[derive(Deserialize)]
pub struct NewTodo {
    description: String,
}

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub id: i64,
    pub description: String,
    pub done: bool,
}

pub fn router() -> Router<SqlitePool> {
    Router::new()
        .route("/", get(list))
        .route("/create", post(create))
        .route("/read/:id", get(read))
        .route("/update", post(update))
        .route("/delete/:id", post(delete))
}

async fn list(State(pool): State<SqlitePool>) -> axum::Result<Json<Vec<Todo>>> {
    let todos = db::list_todos(&pool).await?;
    Ok(Json(todos))
}

async fn create(State(pool): State<SqlitePool>, Form(todo): Form<NewTodo>) -> axum::Result<Redirect> {
    db::create_todo(&pool, todo.description).await?;
    Ok(Redirect::to("/"))
}

async fn read(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> axum::Result<Json<Todo>> {
    let todo = db::read_todo(&pool, id).await?;
    Ok(Json(todo))
}

async fn update(State(pool): State<SqlitePool>, Form(todo): Form<Todo>) -> axum::Result<Redirect> {
    db::update_todo(&pool, &todo).await?;
    Ok(Redirect::to("/"))
}

async fn delete(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> axum::Result<Redirect> {
    db::delete_todo(&pool, id).await?;
    Ok(Redirect::to("/"))
}
