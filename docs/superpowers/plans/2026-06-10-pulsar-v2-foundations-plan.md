# Pulsar V2 Foundations Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add the shared V2 primitives that every later domain depends on: RBAC permissions, profiles, taxonomy, badges, contribution events, and a reusable content rendering service.

**Architecture:** Extend existing Suprnova models and SeaORM migrations without replacing the auth foundation. Keep workflow models in `src/models`, route handlers in `src/controllers`, and DTO mapping beside each controller. Shared rendering lives in `src/content/rendering.rs` and is used by articles, resources, questions, answers, and comments.

**Tech Stack:** Rust/Suprnova models, SeaORM migrations, validator, chrono, serde, Vue/Vuetify pages, loopback HTTP tests.

---

## File Structure

- Create `src/migrations/m20260610_000007_create_v2_foundations.rs`.
- Modify `src/migrations/mod.rs` to register migration `000007`.
- Create models: `src/models/profile.rs`, `category.rs`, `topic.rs`, `tag.rs`, `badge.rs`, `reputation_event.rs`.
- Modify `src/models/mod.rs` to export new models.
- Modify `src/models/user.rs` only if profile convenience methods are needed.
- Modify `src/commands/users_promote.rs` to seed all V2 roles and permissions.
- Create `src/controllers/members.rs`, `admin_taxonomy.rs`, `admin_users.rs`.
- Modify `src/controllers/mod.rs` and `src/routes.rs`.
- Create frontend pages under `frontend/src/pages/members`, `frontend/src/pages/admin/taxonomy`, and `frontend/src/pages/admin/users`.
- Extend `frontend/src/types/inertia-props.ts`.
- Add `tests/v2_foundation_flows.rs`.

## Tasks

### Task 1: Schema and Models

- [ ] Write `tests/v2_foundation_flows.rs` with a `profile_is_created_for_registered_users` test that registers through `/register`, verifies the user row exists, and asserts a matching profile can be loaded by user id.
- [ ] Add migration `000007` with these tables:
  - `profiles`: `id`, `user_id` unique, `handle` unique, `display_name`, `bio`, `avatar_url`, `website_url`, `github_url`, `location`, `timezone`, timestamps.
  - `categories`: `id`, `name`, `slug` unique, `description`, `sort_order`, `is_visible`, timestamps.
  - `topics`: `id`, `name`, `slug` unique, `description`, `sort_order`, `is_visible`, timestamps.
  - `tags`: `id`, `name`, `slug` unique, timestamps.
  - `badges`: `id`, `key` unique, `name`, `description`, `icon`, `role_scoped`, timestamps.
  - `reputation_events`: `id`, `user_id`, `event_type`, `target_type`, `target_id`, `weight`, `summary`, timestamps.
- [ ] Add `#[model]` structs matching those columns. Each model exposes `find_by_slug`, and `Profile` exposes `find_by_handle`, `find_by_user_id`, and `ensure_for_user`.
- [ ] Register model exports in `src/models/mod.rs`.
- [ ] Call `Profile::ensure_for_user(&user)` in the registration success path in `src/controllers/auth.rs`.
- [ ] Run `bash -lc 'RUSTC=$(rustup which rustc) $(rustup which cargo) test v2_foundation_flows -- --nocapture'`; expect the profile creation test to pass.

### Task 2: V2 RBAC

- [ ] Replace the role constants in `src/commands/users_promote.rs` with `admin`, `moderator`, `author`, `contributor`, and `member`.
- [ ] Seed permissions from the V2 spec:
  - `articles.create`, `articles.submit`, `articles.review`, `articles.publish`
  - `questions.create`, `answers.accept_own`, `comments.create`
  - `resources.submit`, `resources.review`, `resources.publish`
  - `moderation.review`, `moderation.decide`
  - `users.manage`, `roles.manage`, `taxonomy.manage`, `settings.manage`
- [ ] Assign all permissions to `admin`; assign review/moderation permissions to `moderator`; assign article publish/review to `author`; assign submission permissions to `contributor`; assign create/comment/bookmark permissions to `member`.
- [ ] Add tests in `tests/v2_foundation_flows.rs` proving a member cannot open `/admin/taxonomy`, a moderator can open moderation queues after that plan lands, and an admin can open `/admin/users`.
- [ ] Run `cargo test rbac`.

### Task 3: Public Profiles and Members

- [ ] Implement `GET /members` and `GET /members/{handle}` in `src/controllers/members.rs`.
- [ ] Include profile summary props: handle, display name, bio, avatar, links, badges, contribution counts.
- [ ] Add Vue pages `frontend/src/pages/members/Index.vue` and `Show.vue` using `AppLayout`.
- [ ] Extend `/profile` settings so users can edit display name, handle, bio, avatar URL, website, GitHub URL, location, and timezone.
- [ ] Add tests for profile update validation: handle required, slug-safe, unique, and visible at `/members/{handle}`.
- [ ] Run `cargo test profile` and `cd frontend && bun run check`.

### Task 4: Taxonomy Admin and Public Taxonomy Pages

- [ ] Implement admin taxonomy routes:
  - `GET /admin/taxonomy`
  - `POST /admin/categories`
  - `PUT /admin/categories/{id}`
  - `POST /admin/topics`
  - `PUT /admin/topics/{id}`
  - `POST /admin/tags`
  - `PUT /admin/tags/{id}`
- [ ] Gate all taxonomy writes with `PermissionMiddleware::<User>::new("taxonomy.manage")`.
- [ ] Implement public pages `GET /topics`, `GET /topics/{slug}`, `GET /tags/{slug}`, and `GET /categories/{slug}` with empty contribution lists until later domains attach content.
- [ ] Add Vue pages for admin taxonomy manager and public taxonomy pages.
- [ ] Add HTTP tests for admin permission denial, admin creation success, and public topic visibility.

### Task 5: Shared Content Rendering Service

- [ ] Create `src/content/rendering.rs` with `RenderedContent { html, excerpt, description, plain_text, headings, has_code, has_math }`.
- [ ] Move article rendering through this service while keeping the current `render_article_content` API intact.
- [ ] Extend rendering behavior inspired by `rikhuijzer/fx`: GFM tables, math detection, footnote detection, stable heading ids, short preview extraction, title/description extraction, and RSS-safe HTML.
- [ ] Add unit tests for tables, math markers, footnotes, heading ids, and description extraction in `src/content/rendering.rs`.
- [ ] Run `cargo test content::rendering`.

### Task 6: Commit

- [ ] Run the full verification gate from the master plan.
- [ ] Commit with:

```bash
git add src tests frontend
git commit -m "Add Pulsar V2 foundations"
```
