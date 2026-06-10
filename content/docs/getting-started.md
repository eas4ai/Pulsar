# Getting Started

Pulsar is a Suprnova starter kit with a Rust backend, an Inertia/Vue frontend, and pre-rendered documentation. Development uses less-common local ports by default: the backend listens on `8765`, and Vite listens on `5765`.

## Local Setup

Install dependencies, run migrations, and build the static docs before opening the app.

```bash
cargo run --bin console -- docs:build
cd frontend && bun run build
cargo run --bin pulsar
```

## Frontend Development

Run Vite from `frontend/` when editing Vue pages.

```bash
cd frontend
bun run dev -- --host 127.0.0.1 --port 5765
```

Read the [authentication guide](authentication.md) before adding account-bound pages.
