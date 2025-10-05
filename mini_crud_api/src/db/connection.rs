use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use std::env;

// Inicializa y devuelve el pool de conexion a SQLite
pub async fn establish_connection() -> SqlitePool {
    // Lee la variable DATABASE_URL del .env
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL error!");

    // Crea el pool con configuracion basica
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("No se pudo conectar a la base de datos!!")
}
