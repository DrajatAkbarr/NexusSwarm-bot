![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Go](https://img.shields.io/badge/go-%2300ADD8.svg?style=for-the-badge&logo=go&logoColor=white)
![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white)

![Type](https://img.shields.io/badge/Type-Research%20Simulation-orange?style=for-the-badge)
![Security](https://img.shields.io/badge/Security-Red%20Teaming-red?style=for-the-badge)
![Status](https://img.shields.io/badge/Maintained-Yes-green?style=for-the-badge)

# NEXUS-SWARM: High-Concurrency Botnet Simulation

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Go](https://img.shields.io/badge/go-%2300ADD8.svg?style=for-the-badge&logo=go&logoColor=white)
![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white)
![Type](https://img.shields.io/badge/Type-Research%20Simulation-orange?style=for-the-badge)

## 📋 Overview
NexusSwarm is a distributed Load Testing & Stress Testing framework designed to simulate high-concurrency botnet attacks in a controlled environment. It demonstrates the use of **Golang** for Command & Control (C2) orchestration and **Rust** for high-performance, asynchronous agents.

## 🏗 Architecture
* **Commander (Go):** Centralized HTTP server that broadcasts attack orders to all connected nodes.
* **Soldier (Rust):** Lightweight, async agents capable of spawning thousands of concurrent requests using Tokio runtime.
* **Orchestration (Docker):** Fully containerized environment to simulate network swarming on a single machine.

## 🚀 How to Run

### Prerequisites
* Docker & Docker Compose

### 1. Build the Swarm
```bash
docker compose up --build

### 2. Scale the Army
Spawn 20 (or more) independent bot containers:
docker compose up -d --scale soldier=20

### 3. Launch Simulation
# Attack local target
http://localhost:8080/admin/attack?target=http://victim

⚠️ Disclaimer
This project is for EDUCATIONAL and RESEARCH purposes only. The author is not responsible for any misuse of this software. Do not use this tool on targets you do not have explicit permission to test.

Built with Love by [DrajatAkbarr]

