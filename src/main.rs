use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenv::dotenv;
use std::env;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

table! {
    users (id) {
        id -> Integer,
        name -> Text,
        first -> Text,
        last -> Text,
        email -> Text,
        // TODO implement google login
    }
}

// TODO implement nullables in tables below

table! {
    decks (id) {
        id -> Integer,
        user_id -> Integer,
        from -> Text,
        to -> Text,
        seen_at -> Timestamp,
    }
}

table! {
    cards (id) {
        id -> Integer,
        user_id -> Integer,
        deck_id -> Integer,
        from -> Text,
        to -> Text,
        example -> Text,
        audio_url -> Text,
        seen_at -> Timestamp,
        seen_for -> Integer, // ms
        rating -> Integer,
        prev_rating -> Integer,
        related -> Array<Integer>,
    }
}

#[derive(serde::Serialize, Selectable, Queryable)]
struct User {
    id: i32,
    name: String,
    first: String,
    last: String,
    email: String,
}

#[derive(serde::Deserialize, Insertable)]
#[diesel(table_name = users)]
struct NewUser {
    name: String,
    first: String,
    last: String,
    email: String,
}

#[derive(serde::Serialize, Selectable, Queryable)]
struct Card {
    id: i32,
    user_id: i32,
    deck_id: i32,
    from: String,
    to: String,
    example: String,
    audio_url: String,
    seen_at: chrono::NaiveDateTime,
    seen_for: i32,
    rating: i32,
    prev_rating: i32,
    related: Vec<i32>,
}

#[derive(serde::Deserialize, Insertable)]
#[diesel(table_name = cards)]
struct NewCard {
    user_id: i32,
    deck_id: i32,
    from: String,
    to: String,
    example: String,
    audio_url: String,
    seen_at: chrono::NaiveDateTime,
    seen_for: i32,
    rating: i32,
    prev_rating: i32,
    related: Vec<i32>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_tokio_postgres=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_url = env::var("DATABASE_URL").unwrap();

    let manager = deadpool_diesel::postgres::Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);

    let pool = deadpool_diesel::postgres::Pool::builder(manager)
        .build()
        .unwrap();

    {
        let conn = pool.get().await.unwrap();

        conn.interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
            .await
            .unwrap()
            .unwrap();
    }

    let app = Router::new()
        .route("/users", get(list_users))
        .route("/users/:id", get(get_user))
        .route("/users", post(create_user))
        .route("/cards", get(list_cards))
        .route("/cards", post(create_card))
        .with_state(pool);

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    // let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn list_users(
    State(pool): State<deadpool_diesel::postgres::Pool>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let res = conn
        .interact(|conn| users::table.select(User::as_select()).load(conn))
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    Ok(Json(res))
}

async fn get_user(
    State(pool): State<deadpool_diesel::postgres::Pool>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let res = conn
        .interact(|conn| users::table.select(User::as_select()).load(conn))
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    Ok(Json(res))
}

async fn create_user(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Json(new_user): Json<NewUser>,
) -> Result<Json<User>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let res = conn
        .interact(|conn| {
            diesel::insert_into(users::table)
                .values(new_user)
                .returning(User::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    Ok(Json(res))
}

async fn list_cards(
    State(pool): State<deadpool_diesel::postgres::Pool>,
) -> Result<Json<Vec<Card>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let res = conn
        .interact(|conn| cards::table.select(Card::as_select()).load(conn))
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    Ok(Json(res))
}

async fn create_card(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Json(new_card): Json<NewCard>,
) -> Result<Json<Card>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let res = conn
        .interact(|conn| {
            diesel::insert_into(cards::table)
                .values(new_card)
                .returning(Card::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    Ok(Json(res))
}

/// Utility function for mapping any error into a `500 Internal Server Error` response.
fn internal_error<E>(err: E) -> (StatusCode, String)
    where
        E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
