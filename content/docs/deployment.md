# Deployment

Build the frontend and docs artifacts before packaging the server so Suprnova can serve generated files from the public tree and documentation JSON from storage.

## Build Steps

```bash
cargo run --bin console -- docs:build
cd frontend && bun run build
cargo build --release
```

## Environment

Set `DATABASE_URL`, `APP_KEY`, `APP_URL`, and mail settings for the target environment. Keep `SERVER_PORT` explicit instead of relying on common development defaults. The checked-in local example uses `8765` for the backend and `5765` for Vite.

## Generated Content

The docs builder reads Markdown from `content/docs/` and writes JSON artifacts into `storage/content/docs/`. Rebuild docs any time a Markdown chapter or `documentation.md` changes.
