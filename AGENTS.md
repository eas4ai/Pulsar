# Repository Guidelines

## Project Structure & Module Organization

Pulsar is a Rust/Suprnova backend with a Vue/Vuetify Inertia frontend. Backend source is under `src/`: controllers in `src/controllers`, models in `src/models`, migrations in `src/migrations`, console commands in `src/commands`, and routing in `src/routes.rs`. Frontend code is in `frontend/src`, with pages under `frontend/src/pages`, shared components under `frontend/src/components`, layout code in `frontend/src/layouts`, and Vuetify setup in `frontend/src/plugins`. Markdown docs live in `content/docs`; generated docs and runtime storage live under `storage`. Public icons and built assets are served from `public`.

## Build, Test, and Development Commands

Use `cargo run --bin pulsar` to run the backend on port `8765`. Use `cd frontend && bun run dev -- --host 127.0.0.1 --port 5765` for Vite. Build docs with `cargo run --bin console -- docs:build`; seed articles with `cargo run --bin console -- articles:seed`. Run the full Rust suite with `cargo test`. Run frontend checks with `cd frontend && bun run check && bun run build`.

## Coding Style & Naming Conventions

Rust uses edition 2024 and standard `cargo fmt` formatting. Keep controllers thin and put persistence behavior on models or content helpers. Vue components use PascalCase filenames, `<script setup lang="ts">`, and local CSS classes in `frontend/src/app.css`. Prefer existing Vuetify, MDI, and Inertia patterns before adding dependencies.

## Testing Guidelines

Integration tests live in `tests/*.rs` and should reuse `tests/common/http.rs` for loopback HTTP coverage. Name tests after observable behavior, such as `public_and_authenticated_routes_return_expected_statuses`. Add or update tests before changing behavior, then run the targeted test and the full gate.

## Commit & Pull Request Guidelines

History currently has only the initial commit, so use concise imperative subjects, for example `Add article RSS feed`. PRs should describe the behavior change, list verification commands, link issues when available, and include screenshots for visible frontend changes.

## Security & Configuration Notes

Never commit `.env`, database files, generated frontend assets, or secrets. Use uncommon local ports from `.env.example` (`8765` and `5765`) unless there is a documented conflict. Generate a real `APP_KEY` before non-local use.
