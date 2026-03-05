# Zyntra-Team

- Introducción:
  El proyecto Zyntra-Team consiste en la creación de una infraestructura base distribuida utilizando entornos Linux mediante WSL2,interconectados
  a través de una red privada virtual con WireGuard en topología hub-and-spoke. El objetivo principal es preparar un entorno técnico escalable
  para la futura implementación de un algoritmo distribuido desarrollado en Rust.

- Componentes principales:
  -Host: Nodo central que administra a los workers.
  -Host (contenedores): -HUB (Se encarga de repartir tareas a los workers)
                        -logger (Introducir descripción)
                        -monitor (Introducir descripción)
                        -shared (Introducir descripción)
                        -target (Introducir descripción)

  -Workers (Spokes): Nodos que se conectan al Hub. 
  -VPN WireGuard: Comunicación segura entre nodos.
  -Docker: 4 contenedores por nodo para garantizar escalabilidad.
  -Rust: Implementación del algoritmo distribuido.

- Requisitos de software:
  *En Host físico:
    -Windows 10/11.
    -Windows Subsystem for Linux (WSL2).
    -Virtualización habilitada en BIOS.
    -PowerShell con permisos de administrador.

  *En el Hub:
    -Wireguard en Windows.
    -Distribución Linux (Ubuntu).
    -Docker.
    -Docker Compose.
  
  *En cada instancia Worker:
    -Distribución Linux (Ubuntu).
    -Wireguard.    
    -Docker.
    -Docker Compose.
  
  *Sistema de organización 
    -Hub:
        docker-workers(HUB)/
        │
        ├── Cargo.toml        (workspace raíz)
        ├── Dockerfile
        ├── docker-compose.yml
        │
        ├── hub/
        │   ├── Cargo.toml
        │   └── src/
        │       └── main.rs
        |
        ├── shared/
        │   ├── Cargo.toml
        │   └── src/
        │       └── lib.rs
        │
        ├── api_gateway/
        │   ├── Cargo.toml
        │   └── src/
        │       └── main.rs
        │
        ├── monitor/
        │   ├── Cargo.toml
        │   └── src/
        │       └── main.rs
        │
        └── logger/
            ├── Cargo.toml
            └── src/
                └── main.rs
  -Workers
        docker-workers(HUB)/
        │
        ├── Cargo.toml        (workspace raíz)
        ├── Dockerfile
        ├── docker-compose.yml
        │
        ├── worker/
        │   ├── Cargo.toml
        │   └── src/
        │       └── main.rs
        |
        ├── shared/
            ├── Cargo.toml
            └── src/
                └── lib.rs
 

    
- Instrucciones para compilar y ejecutar el sistema distribuido en Rust:

  -Contruir la imagen del archivo compose:
    docker-compose build
  
  -Levantar contenedores:
    
  docker-compose up -d
  Primero levanatamos el Hub, para que se quede al pendiente de las peticiones de los workers.
  Al levantar los 4 contenedores de los workers se iniciará el proceso de ejecución del algoritmo de Rust, pidiendo tareas al Hub.
  

- Notas importantes y supuestos.

  En las 3 computadoras que actuarán como workers, se usarán los mismos archivos, el unico que varea será el Hub, que tomará los datos recolectados por filas, y las reconstruirá en una imagen.
  Al terminar las tareas, se les mandará un mensaje a los workers diciendo que ya no hay más tareas por ejecutar
  
