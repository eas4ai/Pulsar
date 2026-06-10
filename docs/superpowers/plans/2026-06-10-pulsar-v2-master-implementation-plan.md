# Pulsar V2 Master Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build Pulsar V2 into a feature-complete, unbranded Suprnova community hub starter kit that can later power `suprnova.app`.

**Architecture:** Keep Pulsar's current Rust/Suprnova/Inertia/Vue/Vuetify shape. Add V2 as domain modules with typed models, migrations, controller modules, Inertia props, Vue pages, HTTP flow tests, and seed/reset commands. Use dedicated models for workflow-heavy content and a shared `Resource` model for type-aware ecosystem directory entries.

**Tech Stack:** Rust 2024, Suprnova, SeaORM migrations, SQLite/Postgres-compatible schema, Vue 3.5, Inertia 3, Vuetify 3, Bun, Markdown rendering through Suprnova content helpers extended with the useful markdown-rs lessons from `rikhuijzer/fx`.

---

## Source Specs

- `docs/specs/2026-06-10-pulsar-v2-product-engineering-spec.md`
- `docs/specs/2026-06-10-pulsar-v2-uiux-claude-design-spec.md`

## Current Baseline

- Backend entry points: `src/routes.rs`, `src/bootstrap.rs`, `src/controllers/*`.
- Data layer: `src/models/*.rs` with `#[suprnova::model]`, migrations in `src/migrations/*`.
- Commands: `src/commands/*`, auto-registered by module declarations.
- Frontend: `frontend/src/pages`, `frontend/src/components`, `frontend/src/types/inertia-props.ts`.
- Tests: loopback HTTP harness in `tests/common/http.rs`; V2 should add focused `tests/v2_*.rs` files.
- Development ports: backend `8765`, Vite `5765`. Keep these uncommon defaults.

## Development Order

1. **Foundations:** RBAC permissions, profile model, taxonomy, typed content render service, contribution events.
2. **Resource Directory:** Shared `Resource` model, typed metadata validation, public directory pages, submission/review flows.
3. **Publishing and Static Content:** Article workflow states, revisions, reviewer comments, autosave, scheduled publishing, RSS, markdown/static service improvements.
4. **Q&A, Comments, Moderation:** Questions, answers, accepted answers, shared comments, reports, decisions, moderation queues.
5. **Dashboards, Discovery, Ops, UI:** Role-aware dashboards, search, bookmarks, notifications, reputation display, framework surfaces, demo seed/reset tooling, Claude Design handoff.

## Plan Files

- `docs/superpowers/plans/2026-06-10-pulsar-v2-foundations-plan.md`
- `docs/superpowers/plans/2026-06-10-pulsar-v2-resource-directory-plan.md`
- `docs/superpowers/plans/2026-06-10-pulsar-v2-publishing-static-content-plan.md`
- `docs/superpowers/plans/2026-06-10-pulsar-v2-qa-comments-moderation-plan.md`
- `docs/superpowers/plans/2026-06-10-pulsar-v2-dashboards-discovery-ops-ui-plan.md`

## Cross-Domain Rules

- Use `rtk` for commands: `rtk cargo`, `rtk bun`, `rtk git`.
- Do not introduce CI requirements into Pulsar V2.
- Do not add billing, subscriptions, organizations, production analytics, or commercial SaaS features.
- Keep public copy unbranded. The downstream brand site can rename surfaces later.
- Public recognition is contribution-led: badges, history, accepted answers, featured resources, and helpful activity. Avoid token or currency framing.
- Keep `metadata_json` only for type-specific `Resource` fields. Shared fields stay in columns.
- Prefer reusable controller helpers and DTO mapping over ad hoc JSON construction in each handler.
- Every new user-visible route gets a loopback HTTP test and an Inertia prop type.

## Verification Gates

Run these after each domain lands:

```bash
rtk cargo check
rtk bash -lc 'RUSTC=$(rustup which rustc) $(rustup which cargo) test'
cd frontend && rtk bun run check
cd frontend && rtk bun run build
```

Run these after content or seed changes:

```bash
rtk cargo run --bin console -- docs:build
rtk cargo run --bin console -- v2:seed-demo
rtk cargo run --bin console -- v2:reset-demo
```

## Completion Criteria

V2 is complete when a new downstream framework community can clone Pulsar, run local seed commands, browse the public hub, register, contribute content/resources/questions, receive moderation feedback, see role-appropriate dashboards, and operate admin/moderator workflows without adding missing community primitives first.
