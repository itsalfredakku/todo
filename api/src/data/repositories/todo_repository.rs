use crate::{models::todo::Todo, db::DB};
use surrealdb::{error::Db::Thrown, Error};

pub struct TodoRepository {
    table: String,
}

impl TodoRepository {
    pub fn new() -> Self {
        TodoRepository {
            table: String::from("todo"),
        }
    }

    pub async fn get_all(&self) -> Result<Vec<Todo>, Error> {
        let records = DB.select(&self.table).await?;
        Ok(records)
    }

    pub async fn get_by_id(&self, id: String) -> Result<Todo, Error> {
        if let Some(record) = DB.select((&self.table, id.clone())).await? {
            return Ok(record);
        }

        Err(Error::Db(Thrown(format!("Todo with id {} not found", id))))
    }

    pub async fn get_by_title(&self, title: String) -> Result<Todo, Error> {
        if let Some(record) = DB
            .query("SELECT * FROM todo WHERE title = $title")
            .bind(("title", title.clone()))
            .await?
            .take(0)?
        {
            return Ok(record);
        }

        Err(Error::Db(Thrown(format!("Todo with title {} not found", title))))
    }

    pub async fn create_todo(&self, content: Todo) -> Result<Vec<Todo>, Error> {
        let record = DB
            .create(&self.table)
            .content(content)
            .await?
            .ok_or_else(|| Error::Db(Thrown("Failed to create todo".to_string())))?;
        Ok(record)
    }

    pub async fn update_todo(&self, id: String, content: Todo) -> Result<Todo, Error> {
        let record = DB
            .update((&self.table, id.clone()))
            .content(content)
            .await?
            .ok_or(Error::Db(Thrown(format!("Todo with id {} not found", id))))?;
        Ok(record)
    }

    pub async fn delete_todo(&self, id: String) -> Result<Todo, Error> {
        let result = DB
            .delete((&self.table, id.clone()))
            .await?
            .ok_or(Error::Db(Thrown(format!("Todo with id {} not found", id))))?;
        Ok(result)
    }
}
