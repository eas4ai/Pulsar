# Articles and RSS

Pulsar includes a database-backed publishing surface for articles and release-style posts. Articles are separate from the manual: docs are generated from Markdown files, while articles are managed through models and admin routes.

## Public Routes

- `/blog` lists published articles.
- `/blog/{slug}` shows one published article.
- `/feed.xml` exposes published articles as RSS.

Only published articles appear publicly.

## Seed Content

Run:

```bash
cargo run --bin console -- articles:seed
```

This creates starter examples so the blog and feed are visible during local development.

## Admin Authoring

Authoring routes live under `/admin/articles` and require a verified account with article permissions.

- `/admin/articles`
- `/admin/articles/create`
- `/admin/articles/{id}/edit`
- `/admin/articles/{id}/publish`

Promote a user before testing authoring:

```bash
cargo run --bin console -- users:promote --email user@example.com --role author
```

## Rendering

Articles use the same Markdown rendering direction as the docs system: headings, links, code blocks, math flags, excerpts, and rendered HTML are stored for fast display.

## Categories and Tags

Articles carry category and tag data. Maintain taxonomy from `/admin/taxonomy`, then use those terms consistently in articles and public navigation.

Continue with [Community and Admin](community-and-admin.md).
