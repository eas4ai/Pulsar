# Pulsar V2 Dashboards, Discovery, Ops, and UI Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Finish V2 as an operable starter kit with role-aware dashboards, discovery/search, notifications/bookmarks, framework product surfaces, demo tooling, documentation, and a Claude Design-ready design-system brief.

**Architecture:** Build dashboards as role-specific controller modules backed by shared summary query helpers. Search starts as database-backed text search over rendered plain-text columns. Demo commands seed and reset local data without external services.

**Tech Stack:** Suprnova controllers and commands, SeaORM queries, Inertia props, Vue/Vuetify app shell, Bun frontend checks, loopback HTTP tests.

---

## File Structure

- Create models: `bookmark.rs`, `follow.rs`, `notification.rs`, `release.rs`, `site_setting.rs`.
- Create migrations:
  - `src/migrations/m20260610_000012_create_dashboard_support.rs`
  - `src/migrations/m20260610_000013_create_releases_settings.rs`
- Create controllers: `dashboard_member.rs`, `dashboard_contributor.rs`, `dashboard_author.rs`, `dashboard_moderator.rs`, `dashboard_admin.rs`, `search.rs`, `bookmarks.rs`, `notifications.rs`, `releases.rs`, `framework_surfaces.rs`, `admin_settings.rs`.
- Create commands: `v2_seed_demo.rs`, `v2_reset_demo.rs`, `v2_admin_create.rs`, `v2_search_rebuild.rs`.
- Create frontend pages under `frontend/src/pages/dashboard`, `admin`, `search`, `releases`, `packages`, `templates`, `integrations`, `tools`, `showcase`, `guides`, `tutorials`, and `learning-paths`.
- Create design-system components under `frontend/src/components/system`, `dashboard`, `search`, and `framework`.
- Update `README.md` and `AGENTS.md` after implementation.
- Add `tests/v2_dashboards_discovery_ops_flows.rs`.

## Tasks

### Task 1: Role-Aware Dashboard Routes

- [ ] Replace the single `/dashboard` controller response with a role-aware redirect or role-aware prop set:
  - member dashboard: onboarding, profile completion, questions, submissions, bookmarks, notifications, contribution summary.
  - contributor dashboard: drafts, submitted resources, review feedback, accepted/featured contributions, suggested next contribution.
  - author dashboard: content pipeline, scheduled posts, revisions, assigned reviews.
  - moderator dashboard: reports, submitted content/resources, flagged users, beginner questions needing attention.
  - admin dashboard: users/roles, content health, taxonomy, resources, moderation overview, settings, seed/reset controls.
- [ ] Add route families under `/dashboard`, `/moderation`, and `/admin`.
- [ ] Add tests that log in as each role and assert dashboard modules do not leak higher-privilege controls.
- [ ] Build Vue pages with dense operational layouts, sidebar navigation, and responsive drawer behavior.

### Task 2: Bookmarks, Follows, and Notifications

- [ ] Add `bookmarks` table with user id, target type, target id, timestamps, and a unique composite index.
- [ ] Add `follows` table for user-to-user and user-to-topic follows.
- [ ] Add `notifications` table: user id, actor id, type, target type, target id, title, body, read_at, timestamps.
- [ ] Implement `POST /bookmarks`, `DELETE /bookmarks/{id}`, `GET /dashboard/bookmarks`, `GET /notifications`, and `POST /notifications/{id}/read`.
- [ ] Emit notifications for accepted answers, review feedback, resource approval/rejection, article approval/rejection, and moderation decisions.
- [ ] Add tests for bookmark uniqueness, notification read state, and dashboard visibility.

### Task 3: Search and Discovery

- [ ] Implement `src/controllers/search.rs` with `GET /search?q=...`.
- [ ] Search across articles, docs catalog entries, questions, resources, releases, topics, tags, and profiles using rendered `plain_text` fields and title/slug matching.
- [ ] Define `SearchResult { title, url, kind, excerpt, badges, updated_at }`.
- [ ] Add global search overlay data to the app shell through a lightweight endpoint or Inertia props on search pages.
- [ ] Add `v2:search-rebuild` command that refreshes generated docs/search artifacts and prints indexed counts.
- [ ] Add tests for cross-content search results and empty-state copy.

### Task 4: Framework Product Surfaces and Releases

- [ ] Add `releases` table with version, title, slug, body markdown/html/plain text, released_at, upgrade_guide_url, status, timestamps.
- [ ] Implement `/releases`, `/releases/{version}`, and `/upgrade-guides/{slug}`.
- [ ] Implement learning routes `/guides`, `/guides/{slug}`, `/tutorials`, `/tutorials/{slug}`, `/learning-paths`, and `/learning-paths/{slug}` by composing `Article`, generated docs catalog entries, and `Resource` records whose type is `tutorial`, `external_article`, or `video`.
- [ ] Implement framework surface controllers for `/packages`, `/templates`, `/integrations`, `/tools`, `/showcase`, and `/learning-paths` by composing `Resource`, `Article`, and `Release` query helpers.
- [ ] Add pages that feel purpose-built, not generic resource filters: package install snippets, template setup commands, provider capability lists, tool platform chips, showcase screenshots.
- [ ] Add tests proving official/featured items sort ahead of community items while published date still orders within each group.

### Task 5: Demo and Local Ops Commands

- [ ] Add `v2:seed-demo` command that runs migrations, seeds roles, admin user, profiles, taxonomy, articles, resources, questions, answers, comments, reports, releases, bookmarks, notifications, and badges.
- [ ] Add `v2:reset-demo` command that deletes V2 demo rows by deterministic slug/email prefixes, then calls `v2:seed-demo`.
- [ ] Add `v2:admin-create --email --name --password` command that creates or promotes a local admin.
- [ ] Keep all commands local-service-free: no external mail, queue, object storage, analytics, or CI service required.
- [ ] Add idempotency tests for each command.

### Task 6: UI System and Claude Design Handoff

- [ ] Implement design tokens in `frontend/src/plugins/vuetify.ts`: neutral base, restrained teal/amber/blue accents, light/dark support, readable focus states.
- [ ] Build reusable components:
  - `EntityCard.vue`, `StatusChip.vue`, `RoleBadge.vue`, `StatsStrip.vue`, `QueueTable.vue`, `MarkdownBody.vue`, `EmptyState.vue`, `ContributionTimeline.vue`, `DashboardModule.vue`.
- [ ] Update `AppNav.vue` and `AppLayout.vue` to expose public nav, authenticated nav, contributor tools, moderator queues, and admin links only when relevant.
- [ ] Keep public pages spacious and educational; keep dashboards dense and operational.
- [ ] Create `docs/design/pulsar-v2-claude-design-handoff.md` summarizing page taxonomy, component inventory, layout requirements, and brand-neutral token guidance from the UI/UX spec.

### Task 7: Documentation and Final Verification

- [ ] Update `README.md` with V2 setup, seed/reset commands, route overview, role matrix, and local ports `8765`/`5765`.
- [ ] Update `AGENTS.md` with V2 development notes after features land.
- [ ] Run:

```bash
cargo run --bin console -- docs:build
cargo run --bin console -- v2:reset-demo
cargo check
bash -lc 'RUSTC=$(rustup which rustc) $(rustup which cargo) test'
cd frontend && bun run check && bun run build
```

- [ ] Commit with:

```bash
git add src tests frontend docs README.md AGENTS.md
git commit -m "Complete Pulsar V2 dashboards discovery ops and UI"
```
