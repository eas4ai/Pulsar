# Local Development

Use `suprnova serve` for the normal development loop. It starts the Rust backend, Vite, and the TypeScript type watcher.

## Start the App

```bash
suprnova serve
```

Expected ports:

- Backend: `http://127.0.0.1:8765`
- Frontend: `http://127.0.0.1:5765`

If you need to pin the ports explicitly:

```bash
suprnova serve --port 8765 --frontend-port 5765
```

## Separate Terminals

Run the backend and frontend separately when you want more direct logs.

```bash
cargo run --bin pulsar
```

```bash
cd frontend
bun run dev -- --host 127.0.0.1 --port 5765
```

## Build Generated Content

Rebuild the manual after editing Markdown:

```bash
cargo run --bin console -- docs:build
```

Seed starter articles:

```bash
cargo run --bin console -- articles:seed
```

Promote a user for authoring:

```bash
cargo run --bin console -- users:promote --email user@example.com --role author
```

## Verification

Run these before handing off a change:

```bash
cargo test
cd frontend && bun run check && bun run build
```

Use targeted tests while iterating:

```bash
cargo test docs
cargo test article_flows
```

Continue with [Configuration](configuration.md).
