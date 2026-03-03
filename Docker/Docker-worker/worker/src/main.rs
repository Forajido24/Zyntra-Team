use shared::{Task, TaskResult};
use std::env;
use tokio::time::{sleep, Duration};

fn calcular_fila(task: &Task) -> Vec<u32> {
    let mut fila_pixels = Vec::with_capacity(task.width as usize);

    let x_min = -2.0;
    let x_max = 1.0;
    let y_min = -1.5;
    let y_max = 1.5;

    let y_coord = y_min + (task.row as f64 / task.height as f64) * (y_max - y_min);

    for x in 0..task.width {
        let x_coord = x_min + (x as f64 / task.width as f64) * (x_max - x_min);

        let mut z_r = 0.0;
        let mut z_i = 0.0;
        let mut iter = 0;

        while z_r * z_r + z_i * z_i <= 4.0 && iter < task.max_iter {
            let temp = z_r * z_r - z_i * z_i + x_coord;
            z_i = 2.0 * z_r * z_i + y_coord;
            z_r = temp;
            iter += 1;
        }

        fila_pixels.push(iter);
    }

    fila_pixels
}

#[tokio::main]
async fn main() {
    let coordinator_url =
        env::var("COORDINATOR_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());

    let worker_id = format!("Worker-{}", std::process::id());

    let client = reqwest::Client::new();

    println!("{} listo para calcular Mandelbrot...", worker_id);

    loop {
        if let Ok(res) = client
            .get(format!("{}/task", coordinator_url))
            .send()
            .await
        {
            if let Ok(task) = res.json::<Task>().await {
                if task.row == 9999 {
                    println!("{}: No hay más trabajo. Esperando...", worker_id);
                    sleep(Duration::from_secs(5)).await;
                    continue;
                }

                println!("{}: Calculando fila {}...", worker_id, task.row);

                let pixels = calcular_fila(&task);

                let result = TaskResult {
                    task_id: task.task_id,
                    worker_id: worker_id.clone(),
                    row: task.row,
                    data: pixels,
                };

                let _ = client
                    .post(format!("{}/result", coordinator_url))
                    .json(&result)
                    .send()
                    .await;
            }
        } else {
            sleep(Duration::from_secs(2)).await;
        }
    }
}
