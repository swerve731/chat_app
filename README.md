# Chat App

A real-time chat application built with Rust, Axum, Tauri, Svelte, and PostgreSQL. The backend is an Axum server, and the frontend is a Tauri desktop app with a Svelte UI.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [Docker Desktop](https://www.docker.com/products/docker-desktop/)
- [Node.js](https://nodejs.org/) (for Tauri/Svelte)
- [pnpm](https://pnpm.io/) (recommended for Tauri, npm/yarn work too)
- Tauri Cli
    ```bash
    cargo install tauri-cli --version "^2.0.0" --locked
    ```
- Sqlx Cli
    ```bash
    cargo install sqlx-cli
    ```
## Project Structure

- **`/`**: Project root directory.
  - `docker-compose.yml`: Configuration for the PostgreSQL database running in Docker.
  - `.env`: Environment variables (e.g., `DATABASE_URL` for the database connection).
- **`api/`**: Rust backend directory.
  - Contains the Axum server code for handling API requests and database logic (using `sqlx` to interact with PostgreSQL).
- **`client/`**: Tauri-Svelte frontend directory.
  - Houses the Tauri desktop app with a Svelte frontend and Rust backend (in `src-tauri/` within this dir).

## Setup Instructions

### 1. Start the PostgreSQL Database

1. **Ensure Docker Desktop is running.**
2. **Start the database:**
   ```bash
   docker compose up -d
   ```
### 2. Start API

1. **Navigate to api dir:**
    ```bash
    cd api
    ```
2. **Start the API:**
    ```bash
    cargo run
    ```
### 3. Start client app

1. **Navigate to client dir:**
    ```bash
    cd client
    ```
2. **Install npm dependencies:**
    ```bash
    npm install
    ```
3. **Run tauri app:**
    ```bash
    cargo tauri dev
    ```