use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::{
    new_string_type,
    persistence::{PersistenceResult, PostgresRepository},
};

#[derive(Serialize, Deserialize, FromRow)]
pub struct Client {
    pub client_id: i32,
    pub name: Name,
}

impl PostgresRepository {
    pub async fn create_client(&self, name: String) -> PersistenceResult<i32> {
        let name = Name(name);
        sqlx::query!(
            "
            INSERT INTO client (name) 
            VALUES ($1)
            RETURNING client_id
            ",
            name.as_str()
        )
        .fetch_one(&self.pool)
        .await
        .map(|row| Ok(row.client_id))?
    }

    pub async fn find_client_by_id(&self, id: i32) -> Option<Client> {
        sqlx::query_as("SELECT client_id, name from client where client_id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .unwrap_or(None)
    }
}

new_string_type!(
    Name,
    max_length = 20,
    error = "name is too big",
    min_length = 1,
    error_min = "name is too short"
);
