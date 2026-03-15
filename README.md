Markdown
# 🚀 Zyntra-Team: Infraestructura Distribuida para el Cálculo de Mandelbrot

---

## 📌 1. Descripción General del Proyecto
Este proyecto consiste en una **infraestructura base distribuida** diseñada para la ejecución de algoritmos de **alto rendimiento en Rust**.

El sistema utiliza una topología **Hub-and-Spoke** interconectada mediante una **red privada virtual (VPN)**, donde un nodo central coordina la asignación de tareas de renderizado del **conjunto de Mandelbrot** a múltiples nodos trabajadores (**workers**). 

Además, el sistema ahora incluye **telemetría avanzada**, midiendo la **latencia en milisegundos** y registrando la **identidad exacta de cada nodo** en tiempo real.

---

## 🧰 2. Requisitos de Software

### 🖥 Host Físico (Hub Central)
- Windows 10 / 11 (Gestiona nativamente la VPN y el Firewall)
- WSL2 habilitado (Ejecuta los contenedores Docker)
- Virtualización habilitada en BIOS
- **Archivo `.wslconfig`** configurado con al menos 8GB de RAM solo si quieres imagenes en resoluciones de 8K.

### 🖧 Nodos del sistema (Workers)
- **Sistema Operativo:** Ubuntu (Máquinas Virtuales)
- **Red:** WireGuard (Cliente)
- **Contenedores:** Docker y Docker Compose V2
- **Lenguaje:** Rust (Cargo)

---

## 🔐 3. Instrucciones para levantar la VPN (WireGuard)

La comunicación entre el **Hub (10.0.0.1)** y los **Workers (10.0.0.x)** se gestiona mediante **WireGuard**. Debido a la arquitectura, el Hub se gestiona desde Windows y los Workers desde Linux.

### Instalación y Configuración del Hub (Desde Windows PowerShell)
Toda la gestión en el host principal se realiza mediante línea de comandos para mayor control:
1. Instalar la CLI de WireGuard para Windows.
2. Definir la interfaz asignando la IP `10.0.0.1` y registrando a cada worker dentro de `[Peer]`.
3. **Regla de Firewall (CRÍTICO):** Para que las peticiones crucen Windows hacia WSL2, ejecutar en **PowerShell como Administrador**:
   ```powershell
   New-NetFirewallRule -DisplayName "Permitir API Gateway Mandelbrot" -Direction Inbound -LocalPort 3005 -Protocol TCP -Action Allow
Instalación y Configuración de los Workers (Desde Ubuntu)
Bash
sudo apt install wireguard
Generación de llaves (Workers)
Crear el par de llaves dentro de /etc/wireguard:

Bash
wg genkey | tee privatekey | wg pubkey > publickey
Configuración de Workers
Configurar wg0.conf

Apuntar al Endpoint público del Hub

Activación en Workers
Bash
sudo wg-quick up wg0
Nota: Las configuraciones en /vpn dentro del repositorio están sanitizadas y no incluyen llaves reales.

---

## 🐳 4. Instrucciones para desplegar contenedores
El sistema utiliza Docker para aislar procesos y garantizar la escalabilidad. Se recomienda usar la versión moderna de Docker Compose (sin guion) para evitar errores de compatibilidad (KeyError: ContainerConfig).

Navegar al directorio
Bash
cd /docker
Identificación de Nodos (Solo Workers)
Antes de levantar, definir en el docker-compose.yml de los workers el nombre de la máquina para la telemetría:

YAML
environment:
  - WORKER_NAME=Nodo-Ubuntu-1
Construir las imágenes
Bash
docker compose build --no-cache
Levantar los contenedores
Bash
docker compose up -d
## ⚙️ 5. Compilación y Ejecución del Sistema en Rust
El sistema se divide en un coordinador (**Hub**), un API Gateway, y múltiples ejecutores (Workers). Todo el código debe compilarse con la bandera de optimización para máxima velocidad.

   - Compilación (CRÍTICO)
Bash
cargo build --release
Ejecución del Hub y Gateway
   - El coordinador:
     Inicia un servidor HTTP en el puerto 3005

   - Gestiona la cola de tareas con un timeout extendido para evitar "Lost Tasks"

   - Reconstruye la i magen final

**Ejecución de los workers**:

   - Solicitan filas mediante GET

   - Procesan el cálculo e inician un cronómetro interno (Instant::now())

   - Devuelven resultados y su latencia mediante POST

## 📎 6. Notas Importantes y Supuestos
Falla por Memoria RAM (**Code 137**)
Si al generar imágenes de resoluciones gigantes (ej. 7680 filas) el contenedor falla al final con exited with code 137 (Out Of Memory), se debe aumentar la memoria en Windows creando el archivo %userprofile%\.wslconfig:

   - Ini, TOML
[wsl2]
memory=8GB
Y reiniciar con wsl --shutdown.

**Finalización**
Cuando se completan todas las filas, el Hub envía row 9999 indicando que no quedan más tareas.

**Salida**
La imagen final se genera como:

   - mandelbrot.png
en el volumen compartido:

/output
**Escalabilidad**
El entorno está configurado para levantar 4 contenedores worker por nodo físico mediante Docker Compose.

---

⭐ **Proyecto desarrollado por Zyntra-Team**
