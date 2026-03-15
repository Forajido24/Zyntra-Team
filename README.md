🚀 Zyntra-Team: Infraestructura Distribuida para el Cálculo de Mandelbrot
📌 1. Descripción General del Proyecto
Este proyecto consiste en una infraestructura base distribuida diseñada para la ejecución de algoritmos de alto rendimiento en Rust.

El sistema utiliza una topología Hub-and-Spoke interconectada mediante una red privada virtual (VPN) con WireGuard. Un nodo central (Hub) coordina la asignación de tareas de renderizado del conjunto de Mandelbrot a múltiples nodos trabajadores (workers) distribuidos, recopilando métricas de latencia y rendimiento en tiempo real.

🧰 2. Arquitectura y Requisitos de Software
Debido a las particularidades del enrutamiento de red, el sistema se divide estratégicamente entre el Host Físico (Windows) y su subsistema Linux (WSL2) para el Hub, mientras que los Workers operan en entornos Linux puros.

🖥 Hub (Host Central)
Sistema Operativo Base: Windows 10 / 11 (Maneja el túnel de WireGuard y el Firewall).

Entorno de Ejecución: WSL2 (Ubuntu) habilitado con Docker Desktop.

Memoria RAM: Configuración en .wslconfig recomendada de 8GB+ para renderizados 4K/8K.

🖧 Nodos Trabajadores (Workers)
Sistema Operativo: Ubuntu (Máquinas Virtuales o Físicas).

Red: Cliente WireGuard (CLI).

Contenedores: Docker y Docker Compose V2 (docker compose).

Lenguaje Base: Rust (Cargo).

🔐 3. Configuración de la VPN (WireGuard) y Red
La comunicación entre el Hub (10.0.0.1) y los Workers (10.0.0.x) requiere configuraciones específicas para sortear los bloqueos de red entre Windows, WSL2 y Docker.

Hub (Configuración en Windows vía PowerShell)
A diferencia de configuraciones tradicionales, la gestión del túnel en el host central se realiza nativamente desde PowerShell para mayor control y automatización:

Gestión del Túnel: Se utiliza la CLI de WireGuard desde PowerShell para administrar la interfaz. El archivo de configuración (wg0.conf) se define asignando la IP 10.0.0.1 al servidor y registrando las llaves públicas de los Workers en la sección [Peer].

Activación: El túnel se levanta directamente ejecutando los comandos de WireGuard en la terminal de PowerShell como administrador.

Regla de Firewall (CRÍTICO): Para evitar que Windows Defender bloquee las peticiones entrantes desde la VPN antes de que lleguen a los contenedores de Docker en WSL2, se debe ejecutar el siguiente comando en PowerShell (como Administrador):

PowerShell
New-NetFirewallRule -DisplayName "Permitir API Gateway Mandelbrot" -Direction Inbound -LocalPort 3005 -Protocol TCP -Action Allow
Workers (Configuración en Ubuntu)
Instalación: sudo apt install wireguard

Generación de llaves: wg genkey | tee privatekey | wg pubkey > publickey

Configuración: Modificar /etc/wireguard/wg0.conf apuntando al endpoint público del Hub.

Activación: sudo wg-quick up wg0

Nota: Las configuraciones en /vpn dentro del repositorio están sanitizadas y no incluyen llaves reales.

⚙️ 4. Lógica del Sistema y Nuevas Funcionalidades
El sistema desarrollado en Rust incorpora características avanzadas de telemetría y tolerancia a fallos:

API Gateway y Control de Tiempos (Timeouts): Las peticiones de los workers pasan por un Gateway configurado con un tiempo de espera amplio (ej. 30-120 segundos) para evitar descartar tareas pesadas (Lost Tasks) durante el cálculo de áreas densas del fractal.

Identificación de Nodos: Cada worker envía su identidad real basada en variables de entorno (WORKER_NAME), permitiendo al Hub saber exactamente qué máquina virtual completó cada fila.

Métricas de Latencia: Los workers miden el tiempo exacto (en milisegundos) que toma renderizar una fila usando std::time::Instant y lo adjuntan en su carga útil (TaskResult), lo que permite monitorear el rendimiento distribuido en tiempo real.

Modo Release: Todo el ecosistema compila utilizando --release para aprovechar las optimizaciones matemáticas de Rust, reduciendo los tiempos de cálculo en un 99%.

🐳 5. Instrucciones para desplegar contenedores
El sistema utiliza Docker Compose para orquestar los microservicios (API Gateway, Coordinator, Logger) y escalar los workers.

Despliegue del Hub (En WSL2)
Navegar al directorio del Hub y ejecutar:

Bash
docker compose build --no-cache
docker compose up -d
Nota: Se recomienda usar la versión moderna docker compose (sin guion) para evitar el error obsoleto KeyError: 'ContainerConfig' de versiones antiguas.

Despliegue de los Workers (En Ubuntu VMs)
En el archivo docker-compose.yml de los workers, definir la variable de identificación y levantar los contenedores:

YAML
environment:
  - COORDINATOR_URL=http://10.0.0.1:3005
  - WORKER_NAME=Nodo-Ubuntu-01
Luego ejecutar:

Bash
sudo docker compose build --no-cache
sudo docker compose up -d
(El sistema está diseñado para levantar 4 contenedores worker por nodo físico).

📎 6. Notas Importantes y Solución de Problemas
Renderizado y Código 137 (Out Of Memory)
Al generar resoluciones masivas (ej. 8K / 7680 filas), el Coordinador ensambla millones de píxeles en memoria RAM antes de guardar. Si el contenedor crashea con el Código 137 al final del proceso:

Crear/modificar el archivo %userprofile%\.wslconfig en Windows.

Aumentar la memoria asignada a WSL2:

Ini, TOML
[wsl2]
memory=8GB
Reiniciar WSL con wsl --shutdown en PowerShell.

Finalización y Salida
Cuando se completan todas las filas, el Hub devuelve la tarea row: 9999, lo que indica a los workers que entren en estado de reposo.

La imagen resultante se exporta automáticamente en el volumen compartido /output como mandelbrot.png.

⭐ Proyecto desarrollado por Zyntra-Team
