# Upgrade Guide

## Pulsar 2.1.1 → 2.2.0

Release `v2.2.0` upgrades Suprnova from 4.1.0 to 4.6.1 and suprnova-markdown from 0.3.0 to 0.5.5. Rust and frontend versions are 2.2.0. Keep Rust 1.94+ and use Bun with the committed `frontend/bun.lock`.

### Prepare

Preserve the deployed configuration, `APP_KEY`, database, uploads, generated docs, binaries and assets. Take a consistent SQLite backup or PostgreSQL dump. Stop writers before the final backup and migration. Merge the release into a branch while keeping your application customizations:

```bash
git fetch upstream refs/tags/v2.2.0
git switch -c upgrade/suprnova-4.6.1
git merge --no-ff FETCH_HEAD
```

Keep the existing `APP_KEY`. The starter now serves the framework's MCP endpoint, OAuth consent pages and agent account claims. Generate a persistent signing key in a trusted terminal, then store both `AGENT_AUTH_KEY_ID` and `AGENT_AUTH_PRIVATE_KEY` in your deployment's secret configuration:

```bash
cargo run --locked --bin console -- agents:key
```

Retain a delivering production `MAIL_DRIVER`. Replicated servers need `RATE_LIMIT_DRIVER=redis` and `RATE_LIMIT_REDIS_URL`. Production boot refuses the default log mailer and per-process limiter unless their explicit acknowledgment flags are set; those flags are appropriate only when discarding mail or using a single process is intentional.

Use the same signing key on every instance of an environment. Boot refuses a missing key. Pulsar declares no agent tools or scopes; its existing publishing and account routes retain their UI authorization rules. Add tools and scopes explicitly when your application needs them.

### Build and migrate

```bash
suprnova generate-types
cargo build --locked --release --bins
(cd frontend && bun install --frozen-lockfile && bun run check && bun run build)
cargo run --locked --bin console -- docs:build
cargo test --locked
cargo clippy --all-targets
cargo audit
```

This release refreshes the dependencies named by the security audit. Keep `.cargo/audit.toml` with the application: its documented exception covers the inactive `rkyv` 0.7 compatibility dependency, for which no compatible fixed release exists. Review the exception and its expiry when changing dependency features. Six unmaintained dependency warnings remain for `basic-toml`, `bincode`, `paste`, `proc-macro-error2`, `ttf-parser` and `yaml-rust`; updating these packages within the current dependency constraints does not remove the warnings.

Use the Suprnova 4.6.1 CLI for type generation. Run it before the frontend check if you changed Rust props. Commit both generated type files. The starter now uses local documentation DTOs so the generator can describe their fields. The type generator refuses tool schemas for the three account forms because their email deserializer is custom, and for the taxonomy form because `sort_order` is `Option<Value>`. None of these forms is offered as a tool.

Deploy the binaries with the matching `public/assets/` and `storage/content/docs/`. Stop writers and run migrations once before restarting the fleet:

```bash
./target/release/pulsar migrate
./target/release/pulsar serve --no-migrate
```

The migrations add the RenderCache ledger, the agents tables and a case-insensitive unique index on `app_users.email`. Before migrating, merge accounts whose email addresses differ only in case and retain their related records. The index migration refuses such duplicates. Existing application, session and RBAC rows are preserved. Magnetar also migrates its own auth tables on boot, including the nullable verification address digest; allow the first instance to finish before starting the rest.

### Application changes

Suprnova 4.6.1 includes the Live action stack fix: the framework frames below an action stay within the 1 MiB budget in a debug build. It also includes the router audit, shared notes gate rule and Rust formatting fixes already present in this starter.

Keep process-wide services in `bootstrap::register` and HTTP middleware in `bootstrap::register_http_stack`. Console and worker images no longer need built frontend assets. The server installs agents, Live routes and RenderCache in that order. Keep `live::authorize()` in bootstrap and in router tests before installing the Live routes.

The application stylesheet and Vuetify styles are separate Vite entries in both Rust and `frontend/vite.config.ts`; remove the application stylesheet import from `frontend/src/main.ts` and the global Vuetify styles import from its plugin. The root template loads the integrity-checked Live async script from `/__live/assets` with the request's CSP nonce. Retain these routes in any proxy rules. Vue, Inertia and their shared utilities load in separate chunks and each page loads on demand. The Vuetify icons use SVG paths instead of the full icon font; add new application icons to `frontend/src/plugins/icons.ts`.

The Inertia asset version now follows the built manifest automatically. Port any local `INERTIA_VERSION` override to the shared `bootstrap::inertia_config()` builder if you still need a manual version.

Shared Inertia props are now registered by name (`auth` and `root`). A cacheable Inertia route with an opaque share hook or provider fails boot in 4.6.1. Keep named shares when adding cached routes, and declare which are public and which resolve per request. This release does not opt Pulsar's personalized pages into public static publication.

Sign-in, registration and password-reset requests trim and lowercase ASCII email addresses before validation. Sign-in and registration preserve a safe same-origin `return_to` for consent flows. Pulsar keeps its stricter 120-character display-name limit and its existing verification flow.

Fingerprinted Vite JS and CSS now use `Cache-Control: public, max-age=31536000, immutable`. Other public files retain Pulsar's one-day policy. Neither should issue session or CSRF cookies for an unchanged guest session. Purge any shared cache that stored cookie-bearing responses before this upgrade. If a public response needs a state cookie or embeds a CSRF token, the framework makes it `private, no-store`.

### Verify and roll back

Run `cargo test --test page_budget` after building assets and docs. It checks the actual server router, including Live assets, against the default page budgets over the home, docs, blog, members, topics and guest account entry pages.

After deployment, check those pages, one published article and its Markdown twin, `/feed.xml`, SEO endpoints, login, registration, verification mail, password reset, profile editing and role-gated authoring. Check `/oauth/approve` and `/agent/claim` send guests through sign-in, and `/connected-agents` lists only the signed-in user's grants. Verify the CSP nonce, fingerprinted asset cache headers and absence of asset cookies. Exercise production mail, Redis and the deployed database in their own environment.

To roll back, stop writers and restore the complete 2.1.1 database backup, binary, assets, generated docs and configuration together. Keep the original `APP_KEY`. Restore the previous release as a unit so the database and auth state match the code using them.

## Pulsar 2.1.0 → 2.1.1

Release `v2.1.1` prevents publicly cached responses from carrying session or CSRF cookies. Upgrade the application binary and frontend assets normally. No database migration or configuration change is required.

After deployment, request `/favicon.ico` and `/site.webmanifest` without cookies. Both responses should retain `Cache-Control: public, max-age=86400` and omit `Set-Cookie`. Ordinary pages should continue to issue session and CSRF cookies. A public response that must deliver another state cookie is automatically changed to `Cache-Control: private, no-store`.

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
