# Customization

Pulsar is intentionally unbranded. Treat it as a working product shell that you adapt to your own framework, SaaS, or community.

## Replace Product Copy

Start with:

- `src/controllers/home.rs` for landing page copy and capability lists.
- `content/docs/*.md` for manual chapters.
- seeded article content in `src/commands/articles_seed.rs`.

Rebuild generated content after editing Markdown:

```bash
cargo run --bin console -- docs:build
```

## Replace Visual Identity

Update:

- `frontend/src/components/BrandMark.vue`
- `frontend/src/app.css`
- `frontend/src/plugins/vuetify.ts`
- files under `public/`

Set the public identity in `.env`: `APP_NAME`, `APP_URL`, `VITE_APP_NAME`, and
the `SEO_*` values control titles, canonical URLs, social cards, language,
theme color, and structured data. Replace `public/social-card.jpg` with a
1280×720 image or update its configured dimensions.

Keep the layout system intact until your replacement design covers every page. Partial restyles make a starter kit feel unfinished.

## Search and Agent Discovery

Public pages include server-rendered metadata and JSON-LD. `/sitemap.xml` and
`/robots.txt` guide search crawlers. `/llms.txt` maps the site for language
models, while `/llms-full.txt` provides the complete public corpus.

Documentation and published articles also have `.md` twins, such as
`/docs/getting-started.md`. The HTML page advertises each twin with a Markdown
alternate link. Article lookup always filters to published records.

## Add Domain Models

For new product data:

1. Add a migration in `migrations/`.
2. Add or update a model under `src/models/`.
3. Add controller handlers under `src/controllers/`.
4. Register routes in `src/routes.rs`.
5. Add Vue pages under `frontend/src/pages`.
6. Add integration tests in `tests/`.

## Keep the Kit Generic

Pulsar should stay useful as an open starter kit. Put brand-specific behavior, private business rules, and production-only integrations in downstream apps.

Continue with [Troubleshooting](troubleshooting.md).
