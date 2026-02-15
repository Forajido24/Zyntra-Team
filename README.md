# Zyntra-Team

- Reporte siguiendo la guía de reportes y que incluya la descripción general del proyecto:
  El proyecto Zyntra-Team consiste en la creación de una infraestructura base distribuida utilizando entornos Linux mediante WSL2,interconectados
  a través de una red privada virtual con WireGuard en topología hub-and-spoke. El objetivo principal es preparar un entorno técnico escalable
  para la futura implementación de un algoritmo distribuido desarrollado en Rust.

- Componentes principales:
  -Host (Hub): Nodo central que administra la VPN.
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

  *En cada instancia WSL2
    -Distribución Linux (Ubuntu).
    -WireGuard.
    -Docker.
    -Docker Compose.
  
  *(Pendiente para fase Rust)
    -Rust.
 
- Instrucciones para levantar la VPN (resumidas):
    -Instala WireGuard:
      sudo apt update
      sudo apt install wireguard -y
  
    -Genera claves:
      wg genkey | tee privatekey | wg pubkey > publickey
  
    -Configuración del Host:
      [Interface]
      PrivateKey = CLAVE_PRIVADA_HOST
      Address = 10.0.0.1/24
      ListenPort = 51820
      
      [Peer]
      PublicKey = CLAVE_PUBLICA_WORKER1
      AllowedIPs = 10.0.0.2/32
      
      [Peer]
      PublicKey = CLAVE_PUBLICA_WORKER2
      AllowedIPs = 10.0.0.3/32

      [Peer]
      PublicKey = CLAVE_PUBLICA_WORKER3
      AllowedIPs = 10.0.0.4/32

    -Configuración del Worker:
      [Interface]
      PrivateKey = CLAVE_PRIVADA_WORKER
      Address = 10.0.0.2/24
      
      [Peer]
      PublicKey = CLAVE_PUBLICA_HOST
      Endpoint = IP_PUBLICA_HOST:51820
      AllowedIPs = 10.0.0.0/24
      PersistentKeepalive = 25

    -Activar la VPN:
      sudo wg-quick up wg0

    -Verificamos que se haya activado:
      sudo wg
  
- Instrucciones para desplegar contenedores:
  -Instalar Docker:
    sudo apt install docker.io -y
    sudo systemctl enable docker
    sudo systemctl start docker

  -Crear docker-compose.yml:
    (PONER EL FINAL)
  
  -Levantar contenedores:
    docker compose up -d

  -Verificamos que se hayan levantado los contenedores:
    docker ps
  
- Instrucciones para compilar y ejecutar el sistema distribuido en Rust:
(PENDIENTE)

- Notas importantes y supuestos.
  
