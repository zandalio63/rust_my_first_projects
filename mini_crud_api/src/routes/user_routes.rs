use axum::{Router, routing::get};
use sqlx::{Pool, Sqlite};
use crate::handlers::user_handler::*;

pub fn user_routes(pool: Pool<Sqlite>) -> Router {
    Router::new()
        .route("/users", get(get_users).post(create_user))
        .route("/users/:id",
            get(get_user_by_id).put(update_user).delete(delete_user))
        .with_state(pool)
}
