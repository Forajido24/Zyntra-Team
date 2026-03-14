//Instera este archivo en carpeta de src, este codigo solo ayuda para saber el progreso de las tareas procesadas.

use axum::{routing::post, Json, Router};
use std::net::SocketAddr;
use shared::LogMessage; // Importamos la estructura de tu librería compartida

#[tokio::main]
async fn main() {
    // Definimos la ruta para recibir los logs del Hub, Monitor y Gateway
    let app = Router::new()
        .route("/log", post(handle_log));

    // Puerto 4000 como acordamos para evitar conflictos con el Hub (3000)
    let addr = SocketAddr::from(([0, 0, 0, 0], 4000));
    println!("Logger iniciado y escuchando en http://{}", addr);

    // El servidor se queda bloqueado aquí esperando peticiones (.await)
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Esta es la función que procesa los mensajes recibidos
async fn handle_log(Json(payload): Json<LogMessage>) -> &'static str {
    println!(
        "[{}] [{}] - {}",
        payload.source,
        payload.level.to_uppercase(),
        payload.message
    );
    "OK"
} 
