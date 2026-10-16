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
- `RATE_LIMIT_DRIVER=redis`
- `RATE_LIMIT_REDIS_URL=<redis url>`
- `MAIL_DRIVER=smtp` (or another delivering driver) and mail credentials for verification and reset links
- `SEO_DESCRIPTION`, `SEO_SOCIAL_IMAGE`, and the remaining `SEO_*` settings from `.env.example`

Use `openssl rand -base64 32 | tr '+/' '-_' | tr -d '='` to generate `APP_KEY`.

Production boot rejects the default in-memory rate limiter. Use Redis to share authentication quotas across replicas. For exactly one server process, `RATE_LIMIT_ALLOW_MEMORY_IN_PRODUCTION=true` explicitly permits in-memory limits; those limits reset on restart.

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
- `/sitemap.xml` and `/robots.txt` use the production `APP_URL`.
- `/llms.txt`, `/llms-full.txt`, and a documentation `.md` twin return text.
- `/login` and `/register` render.

Continue with [Customization](customization.md).
