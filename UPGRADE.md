# Upgrade Guide

## Pulsar 2.0.0 → 2.0.1

Release `v2.0.1` upgrades Suprnova from 2.0.0 to 3.2.1. Rust and frontend versions are 2.0.1. The framework tag in Cargo.toml resolves to `2bd4bd53d04fa4581fdbb152ebd70cfb682b342c` in Cargo.lock.

## Prepare and merge

Commit local changes. Preserve `.env`, the existing `APP_KEY`, database, uploads and previous deployment artifacts. Take a consistent database backup using SQLite's backup API or PostgreSQL's `pg_dump`; copying a live SQLite file can miss WAL data.

Configure a fork's `upstream` once to `https://github.com/eas4ai/Pulsar.git`, then:

```bash
git fetch upstream refs/tags/v2.0.1
git switch -c upgrade/suprnova-3.2.1
git merge --no-ff FETCH_HEAD
```

Resolve conflicts while retaining customizations and the released lockfiles. Do not replace an existing environment with `.env.example` or regenerate its encryption key.

## Build and migrate

Use Rust 1.94+ and Bun. From the repository root:

```bash
cargo build --locked --release --bins
(cd frontend && bun install --frozen-lockfile && bun run check && bun run build)
cargo run --locked --bin console -- docs:build
cargo test --locked
```

Deploy the binaries with `public/assets/` and regenerated `storage/content/docs/`. Stop writers and back up, then run migrations once with the deployment configuration:

```bash
./target/release/pulsar migrate
./target/release/pulsar serve --no-migrate
```

Run the server under your supervisor from the application directory. This release adds no Pulsar migration file; custom forks may have pending migrations. Article seeding is optional starter content, not an upgrade step.

## Production configuration

Keep `APP_ENV=production`, `APP_DEBUG=false`, your existing `APP_KEY` and the correct HTTPS `APP_URL`. Set `RATE_LIMIT_DRIVER=redis` and `RATE_LIMIT_REDIS_URL` for shared quotas. Exactly one server process may explicitly retain memory with `RATE_LIMIT_ALLOW_MEMORY_IN_PRODUCTION=true`; counters reset on restart and cannot protect replicas.

Production rejects non-delivering mail drivers such as `log`. For SMTP, rename `MAIL_HOST`, `MAIL_PORT`, `MAIL_USERNAME` and `MAIL_PASSWORD` to `MAIL_SMTP_HOST`, `MAIL_SMTP_PORT`, `MAIL_SMTP_USER` and `MAIL_SMTP_PASS`. Set `MAIL_FROM`. Use `MAIL_SMTP_ENCRYPTION=starttls` for a suitable relay on 587, or `tls` for implicit TLS on 465; both require credentials. Plaintext `none` is for a local development mail catcher. See [Deployment](content/docs/deployment.md).

## Verify and roll back

Check `/`, `/docs`, `/blog`, `/feed.xml`, login, registration, a built JS/CSS URL and actual reset/verification mail. The release passed 92 Rust tests, frontend checks/builds and an isolated production smoke check; your relay, Redis and deployment database still need verification.

If validation fails, stop writers and restore the previous binaries, matching assets/docs and configuration. If migrations changed state, restore the consistent database backup with its matching key before reopening traffic. Do not assume a down migration reconstructs lost data.
