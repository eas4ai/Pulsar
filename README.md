# Pulsar

Pulsar is a Suprnova starter kit for shipping a product site with account
management, generated documentation, articles, RSS, role-gated authoring, and a
Vue/Vuetify frontend. It is intended to be the source kit for downstream apps
such as `suprnova.app`.

## Stack

- Rust 2024, Suprnova, Tokio, SeaORM, SQLite by default.
- Vue 3, Inertia, Vuetify, Vite, and Bun for frontend tooling.
- Markdown content rendered through Suprnova content helpers.
- Integration tests drive the real HTTP router through an ephemeral loopback
  server.

## Local Setup

Copy `.env.example` to `.env`, then generate an app key before running outside
local test mode:

```bash
cp .env.example .env
rtk cargo run --bin console -- suprnova key:generate
```

Default development ports are intentionally uncommon:

- Backend: `http://localhost:8765`
- Vite: `http://localhost:5765`

Run the backend and frontend in separate terminals:

```bash
rtk cargo run --bin pulsar
cd frontend && rtk bun run dev -- --host 127.0.0.1 --port 5765
```

## Content Workflows

Documentation lives in `content/docs/*.md` and is compiled to
`storage/content/docs`:

```bash
rtk cargo run --bin console -- docs:build
```

Articles are stored in the database. Seed release examples with:

```bash
rtk cargo run --bin console -- articles:seed
```

Promote users for authoring with:

```bash
rtk cargo run --bin console -- users:promote user@example.com author
```

## Auth and Authoring

Registration sends an email verification link. Verified users can access
`/dashboard` and `/profile`; unverified users are held at `/verify-email`.
Members cannot access article authoring. Users with the `author` or `admin`
role can use `/admin/articles`, create drafts, publish articles, and feed
published content to `/blog` and `/feed.xml`.

## Verification Gates

Run these before handing work off:

```bash
rtk cargo run --bin console -- docs:build
rtk cargo run --bin console -- articles:seed
rtk bash -lc 'RUSTC=$(rustup which rustc) $(rustup which cargo) test'
cd frontend && rtk bun run check && rtk bun run build
```

## Key Routes

- Public: `/`, `/docs`, `/docs/getting-started`, `/blog`, `/blog/{slug}`,
  `/feed.xml`
- Guest auth: `/login`, `/register`, `/forgot-password`, `/reset-password`
- Authenticated: `/dashboard`, `/profile`, `/verify-email`
- Authoring: `/admin/articles`, `/admin/articles/create`,
  `/admin/articles/{id}/edit`
