use std::time::Duration;
use shared::LogMessage;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Progress {
    completed: u32,
    total: u32,
}

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let hub_url = "http://coordinator:3000/progress";
    let logger_url = "http://logger:4000/log";

    println!("Monitor iniciado. Rastreando progreso del Mandelbrot...");

    loop {
        // 1. Intentamos obtener el progreso desde el Hub
        if let Ok(response) = client.get(hub_url).send().await {
            if let Ok(stats) = response.json::<Progress>().await {
                let porcentaje = (stats.completed as f32 / stats.total as f32) * 100.0;
                
                println!("Progreso actual: {:.2}% ({} de {} filas)", 
                    porcentaje, stats.completed, stats.total);

                // 2. Reportamos el progreso al Logger para centralizar la info
                let log = shared::LogMessage {
                    source: "monitor".to_string(),
                    level: "info".to_string(),
                    message: format!("Progreso del renderizado: {:.2}%", porcentaje),
                    timestamp: 0, // Aquí podrías usar tiempo real
                };
                
                let _ = client.post(logger_url).json(&log).send().await;

                if stats.completed == stats.total {
                    println!("¡Monitor detectó que el renderizado ha finalizado!");
                    break;
                }
            }
        }

        // Esperamos 5 segundos antes de la siguiente consulta
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
