use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::{
    new_string_type,
    persistence::{PersistenceResult, PostgresRepository},
};

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "status", rename_all = "lowercase")]
pub enum Status {
    Timeout,
    Success,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Transaction {
    pub transaction_id: i32,
    pub client_id: i32,
    value: f32,
    description: Description,
    status: Option<Status>,
    when_started: DateTime<Utc>,
    when_finished: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Serialize, FromRow)]
pub struct NewTransaction {
    value: f32,
    description: Description,
}

impl PostgresRepository {
    pub async fn create_transaction(
        &self,
        id_client: i32,
        nt: NewTransaction,
    ) -> PersistenceResult<i32> {
        sqlx::query!(
            "
            INSERT INTO transaction (client_id, value, description, when_started) 
            VALUES ($1, $2, $3, NOW())
            RETURNING transaction_id
            ",
            id_client,
            nt.value,
            nt.description.as_str(),
        )
        .fetch_one(&self.pool)
        .await
        .map(|row| Ok(row.transaction_id))?
    }

    pub async fn find_transaction_by_id(&self, id: i32) -> Option<Transaction> {
        sqlx::query_as::<sqlx::Postgres, Transaction>(
            "
            SELECT * FROM transaction WHERE transaction_id = $1
            ",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .unwrap_or(None)
    }

    pub async fn close_transaction(&self, id: i32) -> PersistenceResult<i32> {
        sqlx::query!(
            "
            UPDATE transaction 
            SET when_finished = NOW(), status = 'success'
            WHERE transaction_id = $1 AND status IS NULL
            RETURNING transaction_id
            ",
            id
        )
        .fetch_one(&self.pool)
        .await
        .map(|row| Ok(row.transaction_id))?
    }

    pub async fn find_open_transaction_by_client_id(&self, id: i32) -> Option<Vec<Transaction>> {
        match sqlx::query_as(
            "SELECT * FROM transaction WHERE client_id = $1 AND (when_finished IS NULL AND status IS NULL)",
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await
        {
            Ok(ts) => Some(ts),
            Err(e) => {
                println!("{}", e);
                None
            }
        }
    }

    pub async fn check_timeout(&self) {
        let now = chrono::Utc::now();

        if let Ok(ts) = sqlx::query_as::<sqlx::Postgres, Transaction>(
            "SELECT * FROM transaction WHERE when_finished IS NULL",
        )
        .fetch_all(&self.pool)
        .await
        {
            for t in ts {
                if now.signed_duration_since(t.when_started).num_seconds() >= 60 {
                    match sqlx::query!(
                        "
                            UPDATE transaction
                            SET status = 'timeout'
                            WHERE transaction_id = $1
                            RETURNING transaction_id
                            ",
                        t.transaction_id
                    )
                    .fetch_one(&self.pool)
                    .await
                    {
                        Ok(_) => {}
                        Err(e) => println!("{}", e),
                    }
                }
            }
        }
    }
}

new_string_type!(
    Description,
    max_length = 20,
    error = "description is too big",
    min_length = 1,
    error_min = "description is too short"
);
