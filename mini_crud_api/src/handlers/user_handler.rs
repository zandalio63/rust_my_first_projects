use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::json;
use sqlx::{Pool, Sqlite};

use crate::models::user::User;

#[derive(serde::Deserialize)]
pub struct NewUser {
    pub name: String,
    pub email : String,
}

// GET /users - obtener todos
pub async fn get_users(State(pool): State<Pool<Sqlite>>) -> Result<Json<Vec<User>>, StatusCode> {
    let users = sqlx::query_as::<_, User>("SELECT id, name, email FROM users")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(users))
}

// GET /users/:id - obtener por id
pub async fn get_user_by_id(
    State(pool): State<Pool<Sqlite>>,
    Path(id): Path<i64>,
) -> Result<Json<User>, StatusCode> {
    let user = sqlx::query_as::<_, User>("SELECT id, name, email FROM users WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(user))
}

// POST /users - crear nuevo usuario
pub async fn create_user(
    State(pool): State<Pool<Sqlite>>,
    Json(new_user): Json<NewUser>,
) -> Result<(StatusCode, Json<User>), StatusCode> {
    let created = sqlx::query_as::<_, User>("INSERT INTO users (name, email) VALUES (?, ?) RETURNING id, name, email",)
        .bind(new_user.name)
        .bind(new_user.email)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(created)))
}

// PUT /users/:id - actualizar usuario
pub async fn update_user(
    State(pool): State<Pool<Sqlite>>,
    Path(id) : Path<i64>,
    Json(updated) : Json<NewUser>,
) -> Result<Json<User>, StatusCode> {
    let user = sqlx::query_as::<_, User>("UPDATE users SET name = ?, email = ? WHERE id = ? RETURNING id, name, email")
        .bind(updated.name)
        .bind(updated.email)
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(user))
}

// DELETE /users/:id - eliminar usuario
pub async fn  delete_user(
    State(pool) : State<Pool<Sqlite>>,
    Path(id) : Path<i64>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let result = sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(Json(json!({"message": "Usuario eliminado"})))
}
