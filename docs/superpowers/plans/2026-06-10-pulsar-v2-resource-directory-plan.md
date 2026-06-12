# Pulsar V2 Resource Directory Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the categorized ecosystem directory for packages, templates, integrations, tools, examples, showcase projects, tutorials, external articles, videos, and services.

**Architecture:** Use one shared `Resource` model with common columns and validated `metadata_json` typed by resource kind. Public routes are purpose-built for each resource family while admin and submission workflows share controller helpers.

**Tech Stack:** Rust typed metadata structs, serde validation, Suprnova model macros, SeaORM migration, Inertia props, Vue/Vuetify cards/forms/tables.

---

## File Structure

- Create `src/migrations/m20260610_000008_create_resources.rs`.
- Create `src/models/resource.rs`.
- Create `src/content/resource_metadata.rs`.
- Create controllers: `src/controllers/resources.rs`, `admin_resources.rs`, `contributor_resources.rs`.
- Modify `src/controllers/mod.rs`, `src/routes.rs`, and `src/models/mod.rs`.
- Create Vue pages under `frontend/src/pages/resources`, `frontend/src/pages/admin/resources`, and `frontend/src/pages/contributor/resources`.
- Create components under `frontend/src/components/resources`.
- Extend `frontend/src/types/inertia-props.ts`.
- Add `tests/v2_resource_directory_flows.rs`.

## Tasks

### Task 1: Resource Schema and Metadata Types

- [ ] Write failing tests for `resource_metadata_rejects_wrong_fields` and `submitted_resource_requires_review_before_public`.
- [ ] Add `resources` table with fields from the V2 spec: title, slug, summary, description markdown/html, plain text, type, status, official, featured, urls, submitter/approver ids, category id, tags JSON, metadata JSON, published timestamp, and timestamps.
- [ ] Add indexes on `slug`, `type`, `status`, `official`, `featured`, `category_id`, and `submitted_by`.
- [ ] Define allowed resource types as string constants in `src/models/resource.rs`.
- [ ] Define status constants: `draft`, `submitted`, `changes_requested`, `approved`, `published`, `archived`, `rejected`.
- [ ] Create metadata structs in `src/content/resource_metadata.rs`:
  - `PackageMetadata { crate_name, install_command, latest_version, compatibility }`
  - `TemplateMetadata { setup_command, preview_image_url, demo_url }`
  - `ProviderIntegrationMetadata { provider_kind, supported_features }`
  - `ToolMetadata { platforms, install_docs_url }`
  - `ShowcaseMetadata { screenshot_url, project_owner, built_with }`
  - `MediaMetadata { duration_seconds, provider, thumbnail_url }`
- [ ] Implement `validate_resource_metadata(resource_type, metadata_json)` returning typed validation errors for missing type-specific fields.
- [ ] Run `cargo test resource_metadata`.

### Task 2: Submission and Review Workflow

- [ ] Implement member/contributor routes:
  - `GET /resources/submit`
  - `POST /resources`
  - `GET /dashboard/submissions/resources`
  - `GET /dashboard/submissions/resources/{id}/edit`
  - `PUT /dashboard/submissions/resources/{id}`
- [ ] Gate creation with `resources.submit` and ownership checks.
- [ ] Implement admin/moderator review routes:
  - `GET /admin/resources`
  - `GET /admin/resources/{id}`
  - `POST /admin/resources/{id}/approve`
  - `POST /admin/resources/{id}/request-changes`
  - `POST /admin/resources/{id}/reject`
  - `POST /admin/resources/{id}/publish`
- [ ] Gate review actions with `resources.review` and publishing with `resources.publish`.
- [ ] Record `approved_by` and `published_at` when publishing.
- [ ] Add HTTP tests for member submission, public hiding before publish, moderator request changes, contributor resubmission, and admin publish.

### Task 3: Public Directory Routes

- [ ] Implement:
  - `/resources`
  - `/resources/{type}`
  - `/resources/{type}/{slug}`
  - `/packages`
  - `/templates`
  - `/integrations`
  - `/tools`
  - `/showcase`
- [ ] Public index props include curated categories, type counts, featured resources, latest approved/published resources, and filter state.
- [ ] Detail props include common fields, typed metadata, author/submitter profile, category, tags, and related resources.
- [ ] Add frontend pages for all route families using shared `ResourceCard`, `ResourceMetaList`, `ResourceTypeTabs`, and `ResourceSubmitForm`.
- [ ] Add tests proving each public route renders only `published` resources and type aliases map correctly.

### Task 4: Seed Data

- [ ] Create `src/commands/v2_seed_demo.rs` or extend the later shared V2 seed command with resource examples across every type.
- [ ] Include at least one official package, community package, template, provider integration, tool, example app, showcase project, tutorial, external article, video, and service.
- [ ] Ensure seeding is idempotent by checking slug before insert.
- [ ] Add a test that runs the seed command twice and asserts no duplicate slugs.

### Task 5: Commit

- [ ] Run:

```bash
cargo check
bash -lc 'RUSTC=$(rustup which rustc) $(rustup which cargo) test v2_resource_directory_flows -- --nocapture'
cd frontend && bun run check
```

- [ ] Commit with:

```bash
git add src tests frontend
git commit -m "Add Pulsar V2 resource directory"
```
