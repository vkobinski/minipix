use std::{env, sync::Arc};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use models::transaction::NewTransaction;
use persistence::{check_timeout, PostgresRepository};
use tower_http::services::ServeDir;

mod models;
mod persistence;

type AppState = Arc<PostgresRepository>;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URl")
        .unwrap_or("postgresql://minipix:minipix@localhost:5432/minipix".to_string());

    let state = Arc::new(PostgresRepository::connect(&database_url, 3).await.unwrap());

    tokio::spawn(check_timeout(state.clone()));

    let app = Router::new()
        .route("/transaction/:id_transaction", get(get_transaction))
        .route("/client/:id_client", get(get_client))
        .route(
            "/client/:id_client/transaction",
            get(get_open_transaction_for_client),
        )
        .route("/client", post(create_client))
        .route("/client/:id_client/transaction", post(create_transaction))
        .route("/transaction/:id_transaction", post(end_transaction))
        .nest_service("/", ServeDir::new("dist"))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("[::1]:{}", 3000))
        .await
        .unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn get_transaction(
    State(repo): State<AppState>,
    Path(id_transaction): Path<i32>,
) -> impl IntoResponse {
    match repo.find_transaction_by_id(id_transaction).await {
        Some(transaction) => Ok(Json(transaction)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn get_client(State(repo): State<AppState>, Path(id_client): Path<i32>) -> impl IntoResponse {
    match repo.find_client_by_id(id_client).await {
        Some(client) => Ok(Json(client)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn create_client(State(repo): State<AppState>, name: String) -> impl IntoResponse {
    match repo.create_client(name).await {
        Ok(client) => Ok(Json(client)),
        Err(_e) => Err(StatusCode::NOT_FOUND),
    }
}

async fn create_transaction(
    State(repo): State<AppState>,
    Path(id_client): Path<i32>,
    transaction: Json<NewTransaction>,
) -> impl IntoResponse {
    match repo.create_transaction(id_client, transaction.0).await {
        Ok(transaction) => Ok(Json(transaction)),
        Err(_e) => Err(StatusCode::NOT_FOUND),
    }
}

async fn get_open_transaction_for_client(
    State(repo): State<AppState>,
    Path(id_client): Path<i32>,
) -> impl IntoResponse {
    match repo.find_open_transaction_by_client_id(id_client).await {
        Some(ts) => Ok(Json(ts)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn end_transaction(
    State(repo): State<AppState>,
    Path(id_transaction): Path<i32>,
) -> impl IntoResponse {
    match repo.close_transaction(id_transaction).await {
        Ok(transaction) => Ok(transaction.to_string()),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            format!("Transaction has timed out. {e}"),
        )),
    }
}
