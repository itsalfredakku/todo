use sqlx::sqlite::SqlitePool;
use crate::api::Todo;
use anyhow::Result;

pub async fn list_todos(pool: &SqlitePool) -> Result<Vec<Todo>> {
    let todos = sqlx::query_as!(Todo, "SELECT id, description, done FROM todos ORDER BY id")
        .fetch_all(pool)
        .await?;
    Ok(todos)
}

pub async fn create_todo(pool: &SqlitePool, description: String) -> Result<()> {
    sqlx::query!("INSERT INTO todos (description) VALUES (?)", description)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn read_todo(pool: &SqlitePool, id: i64) -> Result<Todo> {
    let todo = sqlx::query_as!(Todo, "SELECT id, description, done FROM todos WHERE id = ?", id)
        .fetch_one(pool)
        .await?;
    Ok(todo)
}

pub async fn update_todo(pool: &SqlitePool, todo: &Todo) -> Result<()> {
    sqlx::query!(
        "UPDATE todos SET description = ?, done = ? WHERE id = ?",
        todo.description,
        todo.done,
        todo.id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete_todo(pool: &SqlitePool, id: i64) -> Result<()> {
    sqlx::query!("DELETE FROM todos WHERE id = ?", id)
        .execute(pool)
        .await?;
    Ok(())
}
