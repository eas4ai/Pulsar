# Project Structure

Pulsar follows the standard Suprnova app shape: Rust owns routing, controllers, models, commands, and generated responses; Vue owns page rendering; Markdown owns the manual and starter content.

## Backend

Backend source lives in `src/`.

- `src/routes.rs` registers public, authenticated, verified, admin, and fallback routes.
- `src/controllers/` contains request handlers and Inertia props.
- `src/models/` contains app models such as `User`, `Profile`, `Article`, `Category`, `Topic`, and `Tag`.
- `src/middleware/` contains local middleware wrappers.
- `src/commands/` contains console commands such as `docs:build` and `articles:seed`.
- `migrations/` contains SeaORM migrations.

Controllers should stay thin. Put persistence behavior on models and shared rendering behavior in content helpers.

## Frontend

Frontend source lives in `frontend/src/`.

- `pages/` maps to Inertia page names returned by controllers.
- `components/` holds reusable UI pieces.
- `layouts/` holds the persistent app shell.
- `plugins/` configures Vuetify.
- `types/` holds generated page props and shared DTO declarations.
- `app.css` holds design tokens and page-level styling.

Generated props live in `frontend/src/types/inertia-props.ts`. Do not hand-edit that file. Shared DTO declarations live beside it.

## Content

Manual chapters live in `content/docs`. `content/docs/documentation.md` is the table of contents. Run `cargo run --bin console -- docs:build` to generate `storage/content/docs/*.json`.

Articles are database-backed. Run `cargo run --bin console -- articles:seed` to seed starter articles.

## Public Assets

Static assets live in `public/`. Vite writes production bundles to `public/assets`, which is generated and should not be hand-edited.

Continue with [Local Development](local-development.md).
