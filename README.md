# 🚀 Zyntra-Team: Infraestructura Distribuida para el Cálculo de Mandelbrot

---

## 📌 1. Descripción General del Proyecto
Este proyecto consiste en una **infraestructura base distribuida** diseñada para la ejecución de algoritmos de **alto rendimiento en Rust**.

El sistema utiliza una topología **Hub-and-Spoke** interconectada mediante una **red privada virtual (VPN)**, donde un nodo central coordina la asignación de tareas de renderizado del **conjunto de Mandelbrot** a múltiples nodos trabajadores (**workers**).

---

## 🧰 2. Requisitos de Software

### 🖥 Host Físico
- Windows 10 / 11  
- WSL2 habilitado  
- Virtualización habilitada en BIOS  

### 🖧 Nodos del sistema
- **Sistema Operativo:** Ubuntu  
- **Red:** WireGuard  
- **Contenedores:** Docker y Docker Compose  
- **Lenguaje:** Rust (Cargo)  

---

## 🔐 3. Instrucciones para levantar la VPN (WireGuard)

La comunicación entre el **Hub (10.0.0.1)** y los **Workers** se gestiona mediante **WireGuard**.

### Instalación
```bash
sudo apt install wireguard
```

### Generación de llaves
Crear el par de llaves dentro de `/etc/wireguard`:

```bash
wg genkey | tee privatekey | wg pubkey > publickey
```

### Configuración

**Hub**
- Configurar `server.conf`
- Definir la interfaz
- Registrar cada worker dentro de `[Peer]`

**Worker**
- Configurar `wg0.conf`
- Apuntar al **Endpoint público del Hub**

### Activación
```bash
wg-quick up wg0
```

> Nota: Las configuraciones en `/vpn` dentro del repositorio están **sanitizadas** y no incluyen llaves reales.

---

## 🐳 4. Instrucciones para desplegar contenedores

El sistema utiliza **Docker** para aislar procesos y garantizar la **escalabilidad**.

### Navegar al directorio
```bash
cd /docker
```

### Construir las imágenes
```bash
docker-compose build
```

### Levantar los contenedores
```bash
docker-compose up -d
```

---

## ⚙️ 5. Compilación y Ejecución del Sistema en Rust

El sistema se divide en un **coordinador (Hub)** y múltiples **ejecutores (Workers)**.

### Compilación
```bash
cargo build --release
```

### Ejecución del Hub
El coordinador:
- Inicia un **servidor HTTP en el puerto 3000**
- Gestiona la **cola de tareas**
- Reconstruye la **imagen final**

### Ejecución del Worker
Los workers:
1. Solicitan filas mediante **GET**
2. Procesan el cálculo
3. Devuelven resultados mediante **POST**

---

## 📎 6. Notas Importantes y Supuestos

### Finalización
Cuando se completan todas las filas, el **Hub envía `row 9999`** indicando que **no quedan más tareas**.

### Salida
La imagen final se genera como:

```
mandelbrot.png
```

en el volumen compartido:

```
/output
```

### Escalabilidad
El entorno está configurado para levantar **4 contenedores worker por nodo físico** mediante **Docker Compose**.

---

⭐ Proyecto desarrollado por **Zyntra-Team**
