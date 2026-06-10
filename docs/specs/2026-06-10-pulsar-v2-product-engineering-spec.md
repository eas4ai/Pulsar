# Pulsar V2 Product and Engineering Spec

## Purpose

Pulsar V2 is the unbranded, open-source framework website and community hub starter kit that downstream projects can brand and operate. The first downstream target is `suprnova.app`, but Pulsar itself must remain generic: a complete community platform template for any framework ecosystem.

The product principle is: build the community product, not just the framework website. V2 should help a curious developer learn safely, ask questions, contribute small resources, publish useful work, gain recognition, and eventually help maintain the ecosystem.

## Positioning

Pulsar V2 takes inspiration from framework communities that combine documentation, ecosystem products, learning content, public contribution, and community support. It should feel credible to expert Rust developers while reducing intimidation for people who are new to Rust or Suprnova.

This is not a commercial SaaS kit. Billing, organization plans, and production analytics are out of scope. CI is not required for Pulsar itself because it is a starter kit, not the production site.

## Roles

- Guest: browse public content, search, register, view profiles and resources.
- Member: manage profile, bookmark content, ask questions, comment, submit resources.
- Contributor: submit posts, tutorials, links, examples, templates, and showcase projects.
- Author: publish approved long-form content, manage revisions, schedule posts.
- Moderator: review submissions, resolve reports, manage content states, apply moderation decisions.
- Admin: manage users, roles, taxonomy, resources, content, settings, and demo data.

Roles are additive. Admins inherit all capabilities. Moderators can review and moderate but should not automatically control site settings.

## Core Domains

### Community Publishing

Extend the v1 article system into a community publishing workflow. Articles support first-party editorial content and user-submitted posts/tutorials.

Required states:

- draft
- submitted
- changes_requested
- approved
- scheduled
- published
- archived
- rejected

Required features:

- Markdown editor with preview.
- Autosave draft state.
- Revision history.
- Reviewer comments.
- Scheduled publishing.
- Public author attribution.
- Topic/tag/category assignment.
- Featured and official flags.
- RSS for published content.

### Questions and Answers

Add a Q&A module to make community support visible and searchable.

Required features:

- Ask question flow.
- Question detail with answers and comments.
- Accepted answer.
- State filters: latest, unanswered, unsolved, solved.
- Beginner-safe tags such as `new-to-rust` and `new-to-suprnova`.
- Moderator close/reopen and duplicate handling.
- Reputation events for accepted answers and helpful replies.

### Comments and Replies

Add first-class comments across articles, resources, showcase entries, and questions.

Use a shared comment model with a typed parent target:

- `target_type`
- `target_id`
- `parent_id`
- `author_id`
- `body_markdown`
- `body_html`
- `status`
- timestamps

Statuses: visible, hidden, flagged, removed. Nested replies should be limited to one or two levels for readability.

### Ecosystem Resource Directory

Use a shared `Resource` model for ecosystem directory items. This avoids duplicated CRUD while keeping the directory type-aware.

Resource types:

- official_package
- community_package
- plugin
- template
- provider_integration
- tool
- example_app
- showcase_project
- tutorial
- external_article
- video
- service

Core fields:

- title
- slug
- summary
- description_markdown
- description_html
- type
- status
- official
- featured
- url
- repository_url
- documentation_url
- submitted_by
- approved_by
- category_id
- tags
- metadata_json
- published_at
- timestamps

`metadata_json` must be validated through typed Rust structs per resource type. It is for type-specific fields, not a dumping ground. Common fields should stay in columns.

Examples of type metadata:

- Package: crate name, install command, latest version, compatibility.
- Template: setup command, preview image, demo URL.
- Provider integration: provider kind, supported framework features.
- Tool: platform support, install docs.
- Showcase: screenshot, project owner, built-with notes.

### Taxonomy and Discovery

Add shared taxonomy:

- categories
- topics
- tags

Categories are curated navigation buckets. Topics are community/discovery pages. Tags are flexible descriptors. The system must support topic pages, category pages, tag pages, and search results across articles, questions, resources, releases, and showcase entries.

Search can begin as database-backed text search with clear interfaces for later replacement by a dedicated search engine.

### Profiles and Contribution Recognition

Profiles should make helpful contribution visible without turning the community into a currency economy.

Profile fields:

- display name
- handle
- bio
- avatar
- website
- GitHub/social links
- location/timezone optional
- role badges
- contribution summary

Recognition is contribution-led:

- accepted answers
- published posts
- featured resources
- approved links/packages/templates
- showcase projects
- moderation/curation badges
- helpful commenter/reviewer indicators

Use internal reputation events for ordering and dashboards, but public UI should emphasize badges, contribution history, and featured work rather than token language.

### Moderation

Moderation must be a core feature, not an afterthought.

Models:

- moderation_reports
- moderation_decisions
- moderation_notes

Moderatable targets:

- article
- question
- answer
- comment
- resource
- profile

Required queues:

- submitted content
- submitted resources
- reported content
- reported users/profiles
- spam review
- changes requested

Decisions should record moderator, reason, target, action, and timestamp. Actions include approve, reject, request changes, hide, restore, archive, mark spam, warn user, and suspend user.

### Role-Aware Dashboards

Dashboards are required product surfaces.

Member dashboard:

- onboarding checklist
- profile completion
- my questions
- my submissions
- bookmarks
- notifications
- contribution summary

Contributor dashboard:

- draft posts
- submitted resources
- review feedback
- accepted/featured contributions
- suggested next contribution

Author dashboard:

- content pipeline
- scheduled posts
- revisions
- article/resource performance summary
- assigned review tasks

Moderator dashboard:

- reports queue
- submitted content queue
- submitted resources queue
- recent decisions
- flagged users
- beginner questions needing help

Admin dashboard:

- users and roles
- content health
- taxonomy manager
- resource manager
- moderation overview
- site settings
- seed/demo reset tools

### Framework Product Surfaces

V2 includes official framework hub pages:

- packages
- templates
- provider integrations
- tools
- example apps
- showcase projects
- changelog/releases
- upgrade guides
- learning paths

These surfaces may render from `Resource`, `Article`, and `Release` models, but public routes should feel purpose-built.

### Starter-Kit Operations

Required commands:

- seed V2 demo data
- reset demo data
- promote user role
- rebuild generated docs/search index
- generate starter admin user

Commands must be safe in local development and clearly documented. They should not require external services.

## Data Model Summary

Dedicated models:

- Article
- ArticleRevision
- Question
- Answer
- Comment
- Resource
- Release
- Category
- Topic
- Tag
- Profile
- Bookmark
- Follow
- Notification
- ReputationEvent
- Badge
- ModerationReport
- ModerationDecision
- SiteSetting

Existing v1 models should be extended where practical. Avoid duplicating article/post concepts unless behavior diverges enough to justify a new model.

## Public Routes

Representative route families:

- `/`
- `/docs`
- `/docs/{slug}`
- `/guides`
- `/guides/{slug}`
- `/blog`
- `/blog/{slug}`
- `/questions`
- `/questions/ask`
- `/questions/{slug}`
- `/resources`
- `/resources/{type}`
- `/resources/{type}/{slug}`
- `/packages`
- `/templates`
- `/integrations`
- `/tools`
- `/showcase`
- `/members`
- `/members/{handle}`
- `/topics`
- `/topics/{slug}`
- `/tags/{slug}`
- `/releases`
- `/releases/{version}`
- `/feed.xml`

Authenticated/admin route families should mirror the dashboard taxonomy and avoid exposing admin-only language to normal users.

## Permissions

Permission names should be explicit and composable:

- articles.create
- articles.submit
- articles.review
- articles.publish
- questions.create
- answers.accept_own
- comments.create
- resources.submit
- resources.review
- resources.publish
- moderation.review
- moderation.decide
- users.manage
- roles.manage
- taxonomy.manage
- settings.manage

Use RBAC for broad role capability and ownership checks for user-owned content.

## Testing Requirements

Add tests for:

- role redirects and permission denials
- user submission workflows
- moderator approval/rejection flows
- resource type validation
- Q&A solved state
- comments visibility and moderation
- dashboard route access per role
- public search and taxonomy pages
- seed/reset command idempotency
- RSS/feed output per content family

Prefer the existing loopback HTTP harness for user-visible route behavior.

## Development Order

1. Taxonomy, profiles, and shared contribution primitives.
2. Resource directory model, admin review, and public directory pages.
3. Community publishing workflow and revisions.
4. Q&A, answers, comments, solved states.
5. Moderation reports, decisions, and queues.
6. Role dashboards.
7. Framework product surfaces.
8. Search, feeds, bookmarks, notifications, reputation.
9. Demo seed/reset tooling and final documentation.

## Completion Criteria

Pulsar V2 is complete when a downstream framework can brand it into a public hub with no major missing community primitives. The kit must include routes, models, role-aware dashboards, moderation, seeded demo data, tests, and documentation for local use.
