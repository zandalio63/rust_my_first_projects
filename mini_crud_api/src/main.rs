use axum::{Router, routing::get, serve};
use tokio::net::TcpListener;
use dotenvy::dotenv;

mod db;
mod models;
mod handlers;
mod routes;

#[tokio::main]
async fn main() {
    // Cargar variables desde .env
    dotenv().ok();
    
    // Crear conexion a SQlite
    let pool = db::connection::establish_connection().await;

    // Crear router basico
    let app = Router::new()
        .route("/", get(root_handler))
        .merge(routes::user_routes::user_routes(pool.clone()));

    // Iniciar servidor
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("API en http://127.0.0.1:3000");

    serve(listener, app).await.unwrap();
}

async fn root_handler() -> &'static str {
    "API funcionando"
}
