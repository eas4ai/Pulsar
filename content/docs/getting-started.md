# Getting Started

Pulsar is a Suprnova starter kit for building a product, framework, or community site without starting from an empty scaffold. It includes a Rust backend, Inertia/Vue frontend, Vuetify design system, account flows, generated docs, articles, RSS, public profiles, and role-gated admin surfaces.

This manual is written for people building with Pulsar. It is not Rustdoc. Rustdoc is useful when you need API signatures, but the manual explains how the app fits together and what to change first.

## First Run

Copy the environment file, generate an app key, build docs, and start the development servers.

```bash
cp .env.example .env
openssl rand -base64 32 | tr '+/' '-_' | tr -d '='
cargo run --bin console -- docs:build
suprnova serve
```

Copy the generated key into `APP_KEY` in `.env`. The default development ports are intentionally uncommon:

- Backend: `http://localhost:8765`
- Vite: `http://localhost:5765`

Open the app at `http://localhost:8765`.

## What You Get

- Public landing page, docs, blog, RSS feed, member profiles, and taxonomy pages.
- Registration, login, logout, email verification, password reset, and profile management.
- Verified-user dashboard and profile editing.
- Author/admin article publishing with Markdown rendering.
- Admin taxonomy management for categories, topics, and tags.
- A Vue/Vuetify frontend organized for downstream customization.

## Recommended Reading Path

Read [Project Structure](project-structure.md) next if you want to understand where code lives. Read [Local Development](local-development.md) if you are trying to run or test the kit. Read [Customization](customization.md) before replacing the sample content with your own product.

## Common Commands

```bash
cargo run --bin console -- docs:build
cargo run --bin console -- articles:seed
cargo run --bin console -- users:promote --email user@example.com --role author
cargo test
cd frontend && bun run check && bun run build
```

Continue with [Project Structure](project-structure.md).
