use axum::{
    routing::{get, post},
    Json, Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
};
use shared::{Task, TaskResult, LogMessage};
use std::sync::Arc;
use std::net::SocketAddr;
use std::time::Duration;

// Estado compartido del Gateway
struct GatewayState {
    client: reqwest::Client,
    hub_url: String,
    logger_url: String,
}

#[tokio::main]
async fn main() {
    // Configuramos un cliente HTTP con tiempo de espera para evitar bloqueos
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .unwrap();

    let state = Arc::new(GatewayState {
        client,
        hub_url: "http://coordinator:3000".to_string(), // Nombre del servicio en Docker
        logger_url: "http://logger:4000/log".to_string(),
    });

    let app = Router::new()
        // Los trabajadores externos llamarán a estos puntos
        .route("/task", get(proxy_get_task))
        .route("/result", post(proxy_submit_result))
        .with_state(state);

    // Escuchamos en 0.0.0.0 para ser visibles desde la VPN (10.0.0.1)
    let addr = SocketAddr::from(([0, 0, 0, 0], 3005));
    println!("--- API Gateway ---");
    println!("Escuchando en http://{}", addr);
    println!("Redirigiendo a Hub en: http://coordinator:3000");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Lógica de Proxy para obtener tareas
async fn proxy_get_task(
    State(state): State<Arc<GatewayState>>
) -> impl IntoResponse {
    let url = format!("{}/task", state.hub_url);
    
    let res = state.client
        .get(&url)
        .send()
        .await;

    match res {
        Ok(response) => {
            if let Ok(task) = response.json::<Task>().await {
                (StatusCode::OK, Json(task)).into_response()
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, "Error al deserializar tarea del Hub").into_response()
            }
        }
        Err(_) => (StatusCode::SERVICE_UNAVAILABLE, "El Coordinador (Hub) no responde").into_response(),
    }
}

// Lógica de Proxy para enviar resultados
async fn proxy_submit_result(
    State(state): State<Arc<GatewayState>>,
    Json(payload): Json<TaskResult>,
) -> impl IntoResponse {
    let url = format!("{}/result", state.hub_url);

    let res = state.client
        .post(&url)
        .json(&payload)
        .send()
        .await;

    match res {
        Ok(_) => (StatusCode::OK, "Resultado enviado al Hub").into_response(),
        Err(_) => (StatusCode::SERVICE_UNAVAILABLE, "No se pudo entregar el resultado al Hub").into_response(),
    }
}
