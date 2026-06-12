# Deployment

Deploy Pulsar as a Rust server plus generated frontend and documentation artifacts. The app does not require a Node process in production after assets are built.

## Release Build

```bash
cargo run --bin console -- docs:build
cd frontend && bun run build
cargo build --release
```

The Vite build writes assets into `public/assets`. The docs build writes JSON artifacts into `storage/content/docs`. The Rust binary serves both through Suprnova.

## Required Environment

- `APP_ENV=production`
- `APP_DEBUG=false`
- `APP_KEY=<generated key>`
- `APP_URL=https://your-domain.example`
- `DATABASE_URL=<sqlite or postgres url>`
- `SERVER_HOST=0.0.0.0`
- `SERVER_PORT=<platform port>`
- Mail settings for verification and reset links

Use `openssl rand -base64 32 | tr '+/' '-_' | tr -d '='` to generate `APP_KEY`.

## Database

SQLite is convenient for local development. For production, use Postgres unless your deployment target has a clear persistence story for SQLite files.

Run migrations before traffic reaches the new release:

```bash
cargo run --bin pulsar -- migrate
```

## Static Artifacts

Build artifacts should be created during deployment, not at request time. Rebuild docs any time `content/docs/*.md` or `content/docs/documentation.md` changes.

## Smoke Check

After deployment, verify:

- `/` returns the landing page.
- `/docs` renders the manual.
- `/blog` renders seeded or published articles.
- `/feed.xml` returns XML.
- `/login` and `/register` render.

Continue with [Customization](customization.md).
