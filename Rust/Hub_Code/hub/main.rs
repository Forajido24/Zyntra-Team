use axum::{
    routing::{get, post},
    Json, Router,
    extract::State,
};
use shared::{Task, TaskResult};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU32, Ordering};
use image::{ImageBuffer, Rgb};
use serde::Serialize; // <-- Añadido para serializar el progreso

struct AppState {
    next_row: AtomicU32,
    completed_rows: AtomicU32,
    total_rows: u32,
    width: u32,
    image_data: Mutex<Vec<Vec<u32>>>,
}

// <-- NUEVA ESTRUCTURA AGREGADA
#[derive(Serialize)]
struct Progress {
    completed: u32,
    total: u32,
}

#[tokio::main]
async fn main() {
    let width = 3840;
    let height = 2160;

    let state = Arc::new(AppState {
        next_row: AtomicU32::new(0),
        completed_rows: AtomicU32::new(0),
        total_rows: height,
        width,
        image_data: Mutex::new(vec![vec![0; width as usize]; height as usize]),
    });

    let app = Router::new()
        .route("/task", get(get_task))
        .route("/result", post(submit_result))
        .route("/progress", get(get_progress)) // <-- NUEVA RUTA AGREGADA
        .with_state(state);

    println!("Coordinador iniciado en 0.0.0.0:3000");
    
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_task(State(state): State<Arc<AppState>>) -> Json<Task> {
    let row = state.next_row.fetch_add(1, Ordering::SeqCst);

    if row < state.total_rows {
        Json(Task {
            task_id: row,
            row,
            width: state.width,
            height: state.total_rows,
            max_iter: 5000,
        })
    } else {
        Json(Task {
            task_id: 0,
            row: 9999,
            width: 0,
            height: 0,
            max_iter: 0,
        })
    }
}

async fn submit_result(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<TaskResult>,
) -> &'static str {

    let mut img_data = state.image_data.lock().unwrap();
    img_data[payload.row as usize] = payload.data;

    let completadas = state.completed_rows.fetch_add(1, Ordering::SeqCst) + 1;

    println!(
        "Recibida fila {} de {} (por: {})",
        payload.row,
        state.total_rows,
        payload.worker_id
    );

    if completadas == state.total_rows {
        println!("¡Cálculo finalizado! Generando mandelbrot.png...");

        let mut img = ImageBuffer::new(state.width, state.total_rows);

        for (y, row_data) in img_data.iter().enumerate() {
            for (x, &iter) in row_data.iter().enumerate() {

                let pixel_color = if iter == 1000 {
                    Rgb([0, 0, 0])
                } else {
                    let color = (iter % 256) as u8;
                    Rgb([color, color, 255])
                };

                img.put_pixel(x as u32, y as u32, pixel_color);
            }
        }

        img.save("/output/mandelbrot.png").unwrap();
        println!("¡mandelbrot.png guardado!");
    }

    "OK"
}

// <-- NUEVA FUNCIÓN AGREGADA
async fn get_progress(State(state): State<Arc<AppState>>) -> Json<Progress> {
    Json(Progress {
        completed: state.completed_rows.load(Ordering::SeqCst),
        total: state.total_rows,
    })
}
