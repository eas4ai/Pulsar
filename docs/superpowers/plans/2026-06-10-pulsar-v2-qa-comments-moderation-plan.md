# Pulsar V2 Q&A, Comments, and Moderation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add community support, shared discussion, and moderation as first-class product surfaces.

**Architecture:** Use dedicated `Question` and `Answer` models for solved-state behavior, a shared typed-parent `Comment` model for discussion across domains, and moderation report/decision models that can target any community object.

**Tech Stack:** Rust/Suprnova controllers, ownership checks, RBAC middleware, shared markdown rendering, Vue/Vuetify forms and queue tables, loopback HTTP flow tests.

---

## File Structure

- Create migrations:
  - `src/migrations/m20260610_000010_create_questions_answers_comments.rs`
  - `src/migrations/m20260610_000011_create_moderation.rs`
- Create models: `question.rs`, `answer.rs`, `comment.rs`, `moderation_report.rs`, `moderation_decision.rs`, `moderation_note.rs`.
- Create controllers: `questions.rs`, `answers.rs`, `comments.rs`, `moderation.rs`, `admin_moderation.rs`.
- Modify `src/controllers/mod.rs`, `src/models/mod.rs`, `src/routes.rs`.
- Create frontend pages under `frontend/src/pages/questions`, `frontend/src/pages/moderation`, and `frontend/src/pages/admin/moderation`.
- Create components under `frontend/src/components/community` and `frontend/src/components/moderation`.
- Extend `frontend/src/types/inertia-props.ts`.
- Add `tests/v2_qa_comments_moderation_flows.rs`.

## Tasks

### Task 1: Questions and Answers

- [ ] Write failing tests for asking a question, answering it, accepting an answer, filtering unanswered questions, and blocking guests from posting.
- [ ] Add `questions` table: title, slug, body markdown/html/plain text, author id, status, solved_at, accepted_answer_id, duplicate_of_id, closed_by, closed_at, close_reason, tags JSON, timestamps.
- [ ] Add `answers` table: question id, author id, body markdown/html/plain text, status, accepted_at, timestamps.
- [ ] Implement public routes:
  - `GET /questions`
  - `GET /questions?filter=unanswered`
  - `GET /questions?filter=unsolved`
  - `GET /questions?filter=solved`
  - `GET /questions/{slug}`
- [ ] Implement authenticated routes:
  - `GET /questions/ask`
  - `POST /questions`
  - `POST /questions/{id}/answers`
  - `POST /questions/{id}/answers/{answer_id}/accept`
- [ ] Add beginner-safe tags to seed data: `new-to-rust`, `new-to-suprnova`, `good-first-question`.
- [ ] Record reputation events when an answer is accepted.

### Task 2: Shared Comments

- [ ] Add `comments` table with `target_type`, `target_id`, `parent_id`, `author_id`, `body_markdown`, `body_html`, `plain_text`, `status`, timestamps.
- [ ] Allow targets: `article`, `question`, `answer`, `resource`, `showcase`.
- [ ] Enforce nested replies at one level by rejecting a reply whose parent already has `parent_id`.
- [ ] Implement:
  - `POST /comments`
  - `POST /comments/{id}/hide`
  - `POST /comments/{id}/restore`
  - `DELETE /comments/{id}`
- [ ] Gate create with `comments.create`; gate hide/restore with moderation permissions.
- [ ] Render comments on article, resource, question, and answer detail pages.
- [ ] Add tests for visibility, nested reply limit, owner deletion, moderator hide/restore, and guest denial.

### Task 3: Reports and Decisions

- [ ] Add `moderation_reports`: reporter id, target type, target id, reason, details, status, assigned_to, timestamps.
- [ ] Add `moderation_decisions`: moderator id, target type, target id, action, reason, note, timestamps.
- [ ] Add `moderation_notes`: moderator id, target type, target id, body, timestamps.
- [ ] Implement report route `POST /moderation/reports`.
- [ ] Implement moderation dashboard routes:
  - `GET /moderation`
  - `GET /moderation/reports`
  - `GET /moderation/submissions`
  - `GET /moderation/resources`
  - `GET /moderation/users`
  - `GET /moderation/spam`
  - `GET /moderation/decisions/{id}`
- [ ] Implement decision actions: approve, reject, request changes, hide, restore, archive, mark spam, warn user, suspend user.
- [ ] Apply target changes in the same handler that records the decision.
- [ ] Add tests proving a decision row is written and the target state changes.

### Task 4: Moderator UI

- [ ] Build queue components: `ModerationQueueTable.vue`, `ReportCard.vue`, `DecisionTimeline.vue`, `TargetPreview.vue`, `DecisionForm.vue`.
- [ ] Build moderator pages matching the UI/UX spec: dashboard, content review, resource review, question/report queue, comment report queue, user report queue, spam review, recent decisions.
- [ ] The queue row props include severity, target type, target title, author, reporter, age, and current status.
- [ ] Avoid admin-only wording in moderator screens; moderators review and decide, admins configure.
- [ ] Run `cd frontend && bun run check && bun run build`.

### Task 5: Commit

- [ ] Run:

```bash
cargo check
bash -lc 'RUSTC=$(rustup which rustc) $(rustup which cargo) test v2_qa_comments_moderation_flows -- --nocapture'
cd frontend && bun run check
```

- [ ] Commit with:

```bash
git add src tests frontend
git commit -m "Add Pulsar V2 community support and moderation"
```
