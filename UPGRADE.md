# Upgrade Guide

## Pulsar 2.0.1 → 2.1.0

Release `v2.1.0` upgrades Suprnova from 3.2.1 to 4.1.0. Rust and frontend versions are 2.1.0. The framework tag resolves to commit `3aa40bd7c266d41d86a21c0eebbbb1d5a2d09d1d` in `Cargo.lock`.

## Prepare

Preserve the deployed `.env`, `APP_KEY`, database, uploads, and previous artifacts. Take a consistent backup with SQLite's backup API or PostgreSQL's `pg_dump`; copying a live SQLite file can omit WAL data. Stop writers before the final backup and migration.

Fetch and merge the release while retaining application customizations:

```bash
git fetch upstream refs/tags/v2.1.0
git switch -c upgrade/suprnova-4.1.0
git merge --no-ff FETCH_HEAD
```

Do not replace `.env` with `.env.example` or regenerate `APP_KEY`.

## Build and migrate

Use Rust 1.94+ and Bun:

```bash
cargo build --locked --release --bins
(cd frontend && bun install --frozen-lockfile && bun run check && bun run build)
cargo run --locked --bin console -- docs:build
cargo test --locked
```

Deploy the binaries with `public/assets/` and `storage/content/docs/`. With writers stopped, run migrations exactly once, then start without automatic migration:

```bash
./target/release/pulsar migrate
./target/release/pulsar serve --no-migrate
```

This release converts the legacy session table to Suprnova 4's layout and converts RBAC tables to the Spatie-compatible layout. Rows are preserved, but the old binary cannot use the new layouts. Users with stale remember-me cookies may need to sign in again.

## Configuration

Set `APP_ENV` explicitly in every environment. Production should keep `APP_ENV=production`, `APP_DEBUG=false`, its existing `APP_KEY`, and the canonical HTTPS `APP_URL`.

Set `REDIS_PREFIX=` initially to keep keys written by Suprnova 3.2.1. Change it only after deliberately migrating or expiring existing cache, queue, and application Redis keys. Keep signed identifiers enabled. Pulsar ships SQLite and PostgreSQL support; a custom MySQL build must set the framework metadata option `unsigned_ids = false` while retaining signed IDs.

The application enables Suprnova's balanced security headers and passes the CSP nonce to Vue. Start production with `SECURITY_HEADERS_CSP_REPORT_ONLY=true`, inspect reports, then set it to `false`. Leave HSTS preload disabled unless the domain has intentionally joined the preload list.

Reporting remains disabled with `REPORTING_DRIVER=null`. To opt in, use `REPORTING_DRIVER=sentry`, set `REPORTING_DSN` and `REPORTING_RELEASE=pulsar@2.1.0`, and choose log capture, trace sampling, and PII settings deliberately. Pulsar has no SSR bundle, so Suprnova's bundle detection continues to use client rendering.

This release adds `suprnova-markdown` 0.3.0 plus server-rendered metadata, JSON-LD, `/sitemap.xml`, `/robots.txt`, `/llms.txt`, and `/llms-full.txt`. Copy the `SEO_*` values from `.env.example`, set `APP_URL` to the canonical HTTPS origin, and replace `public/social-card.jpg` when applying your brand. Markdown twins at `/docs/{slug}.md` and `/blog/{slug}.md` are public, marked `noindex`, and only expose published articles.

For replicated production servers, use `RATE_LIMIT_DRIVER=redis` and configure `RATE_LIMIT_REDIS_URL`. Retain the production mail configuration described in [Deployment](content/docs/deployment.md).

## Verify and roll back

Check `/`, `/docs`, `/blog`, `/feed.xml`, `/sitemap.xml`, `/robots.txt`, `/llms.txt`, one documentation `.md` twin, login, registration, a built JS/CSS URL, response security headers, and real reset/verification mail. Confirm sessions survive a normal request and role-gated authoring still works. Verify Redis, mail, reporting, and the production database in their deployed environment.

To roll back, stop writers and restore the complete pre-upgrade database backup together with the 2.0.1 binary, matching assets, generated docs, configuration, and `APP_KEY`. Restoring only the old binary is unsafe after the session and RBAC conversions.
